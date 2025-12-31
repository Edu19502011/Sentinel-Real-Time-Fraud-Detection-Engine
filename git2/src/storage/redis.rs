use crate::models::UserProfile;
use redis::{aio::ConnectionManager, AsyncCommands, RedisError};

#[derive(Clone)]
pub struct RedisClient {
    conn: ConnectionManager,
}

impl RedisClient {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }

    /// Sliding window: count transactions in last N seconds
    pub async fn count_recent_transactions(
        &self,
        user_id: &str,
        window_seconds: i64,
    ) -> Result<u64, RedisError> {
        let mut conn = self.conn.clone();
        let key = format!("user:{}:txns", user_id);
        let now = chrono::Utc::now().timestamp();
        let min_time = now - window_seconds;

        // Remove old entries
        let _: () = conn.zrembyscore(&key, "-inf", min_time).await?;

        // Count entries in window
        let count: u64 = conn.zcount(&key, min_time, now).await?;

        Ok(count)
    }

    /// Add transaction to sliding window
    pub async fn add_transaction(
        &self,
        user_id: &str,
        transaction_id: &str,
    ) -> Result<(), RedisError> {
        let mut conn = self.conn.clone();
        let key = format!("user:{}:txns", user_id);
        let now = chrono::Utc::now().timestamp();

        // Add to sorted set with timestamp as score
        let _: () = conn.zadd(&key, transaction_id, now).await?;

        // Set expiration (keep data for 1 hour)
        let _: () = conn.expire(&key, 3600).await?;

        Ok(())
    }

    /// Get user profile
    pub async fn get_user_profile(&self, user_id: &str) -> Result<UserProfile, RedisError> {
        let mut conn = self.conn.clone();
        let key = format!("user:{}:profile", user_id);

        let data: Option<String> = conn.get(&key).await?;

        match data {
            Some(json) => {
                let profile: UserProfile = serde_json::from_str(&json)
                    .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Parse error", e.to_string())))?;
                Ok(profile)
            }
            None => Ok(UserProfile {
                user_id: user_id.to_string(),
                ..Default::default()
            }),
        }
    }

    /// Update user profile
    pub async fn update_user_profile(&self, profile: &UserProfile) -> Result<(), RedisError> {
        let mut conn = self.conn.clone();
        let key = format!("user:{}:profile", profile.user_id);

        let json = serde_json::to_string(profile)
            .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "Serialize error", e.to_string())))?;

        let _: () = conn.set_ex(&key, json, 86400).await?; // 24h expiration

        Ok(())
    }

    /// Increment counter (for metrics)
    pub async fn increment_counter(&self, key: &str) -> Result<u64, RedisError> {
        let mut conn = self.conn.clone();
        conn.incr(key, 1).await
    }
}
