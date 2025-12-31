use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub user_id: String,
    pub amount: f64,
    pub merchant: String,
    pub location: String,
    pub device_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transaction_id: Uuid,
    pub status: TransactionStatus,
    pub risk_score: f64,
    pub rules_triggered: Vec<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Approved,
    Blocked,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub avg_transaction_amount: f64,
    pub transaction_count: u64,
    pub last_transaction_time: Option<chrono::DateTime<chrono::Utc>>,
    pub known_devices: Vec<String>,
    pub known_locations: Vec<String>,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            avg_transaction_amount: 0.0,
            transaction_count: 0,
            last_transaction_time: None,
            known_devices: Vec::new(),
            known_locations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FraudCheckResult {
    pub is_fraud: bool,
    pub risk_score: f64,
    pub rules_triggered: Vec<String>,
}
