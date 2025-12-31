use crate::models::{FraudCheckResult, Transaction, TransactionStatus, UserProfile};
use crate::rules::RuleEngine;
use crate::storage::redis::RedisClient;

pub struct FraudEngine {
    redis: RedisClient,
    rule_engine: RuleEngine,
}

impl FraudEngine {
    pub fn new(redis: RedisClient, rule_engine: RuleEngine) -> Self {
        Self {
            redis,
            rule_engine,
        }
    }

    pub async fn check_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<FraudCheckResult, Box<dyn std::error::Error>> {
        // Get user profile
        let mut profile = self.redis.get_user_profile(&transaction.user_id).await?;

        // Count recent transactions (sliding window: last 60 seconds)
        let recent_tx_count = self
            .redis
            .count_recent_transactions(&transaction.user_id, 60)
            .await?;

        // Evaluate rules
        let (risk_score, rules_triggered) =
            self.rule_engine
                .evaluate(transaction, &profile, recent_tx_count);

        // Determine if fraud
        let is_fraud = self.rule_engine.should_block(&rules_triggered) || risk_score > 0.7;

        // Update profile
        self.update_profile(&mut profile, transaction).await?;

        // Record transaction in sliding window
        let tx_id = uuid::Uuid::new_v4().to_string();
        self.redis
            .add_transaction(&transaction.user_id, &tx_id)
            .await?;

        // Update metrics
        if is_fraud {
            let _ = self.redis.increment_counter("fraud:blocked:total").await;
        } else {
            let _ = self.redis.increment_counter("fraud:approved:total").await;
        }

        Ok(FraudCheckResult {
            is_fraud,
            risk_score,
            rules_triggered,
        })
    }

    async fn update_profile(
        &self,
        profile: &mut UserProfile,
        transaction: &Transaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Update average amount
        let total = profile.avg_transaction_amount * profile.transaction_count as f64;
        profile.transaction_count += 1;
        profile.avg_transaction_amount =
            (total + transaction.amount) / profile.transaction_count as f64;

        // Update last transaction time
        profile.last_transaction_time = Some(transaction.timestamp);

        // Add device if new (limit to 5 devices)
        if !profile.known_devices.contains(&transaction.device_id) {
            profile.known_devices.push(transaction.device_id.clone());
            if profile.known_devices.len() > 5 {
                profile.known_devices.remove(0);
            }
        }

        // Add location if new (limit to 5 locations)
        if !profile.known_locations.contains(&transaction.location) {
            profile.known_locations.push(transaction.location.clone());
            if profile.known_locations.len() > 5 {
                profile.known_locations.remove(0);
            }
        }

        // Save profile
        self.redis.update_user_profile(profile).await?;

        Ok(())
    }

    pub fn determine_status(&self, result: &FraudCheckResult) -> TransactionStatus {
        if result.is_fraud {
            TransactionStatus::Blocked
        } else if result.risk_score > 0.5 {
            TransactionStatus::Review
        } else {
            TransactionStatus::Approved
        }
    }
}
