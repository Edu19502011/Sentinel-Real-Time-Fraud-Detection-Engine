use crate::models::{Transaction, UserProfile};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub threshold: f64,
    pub risk_score: f64,
    pub action: RuleAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    VelocityCheck,
    AmountAnomaly,
    UnknownDevice,
    UnknownLocation,
    HighAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Block,
    Review,
    Alert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    pub rules: Vec<Rule>,
}

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: RuleConfig = serde_json::from_str(&content)?;
        Ok(Self {
            rules: config.rules,
        })
    }

    pub fn evaluate(
        &self,
        transaction: &Transaction,
        profile: &UserProfile,
        recent_tx_count: u64,
    ) -> (f64, Vec<String>) {
        let mut total_risk = 0.0;
        let mut triggered_rules = Vec::new();

        for rule in &self.rules {
            if self.check_rule(rule, transaction, profile, recent_tx_count) {
                total_risk += rule.risk_score;
                triggered_rules.push(rule.name.clone());
            }
        }

        // Normalize risk score to 0-1 range
        let normalized_risk = (total_risk / self.rules.len() as f64).min(1.0);

        (normalized_risk, triggered_rules)
    }

    fn check_rule(
        &self,
        rule: &Rule,
        transaction: &Transaction,
        profile: &UserProfile,
        recent_tx_count: u64,
    ) -> bool {
        match rule.rule_type {
            RuleType::VelocityCheck => recent_tx_count as f64 > rule.threshold,
            RuleType::AmountAnomaly => {
                if profile.avg_transaction_amount > 0.0 {
                    transaction.amount > profile.avg_transaction_amount * rule.threshold
                } else {
                    false
                }
            }
            RuleType::UnknownDevice => {
                !profile.known_devices.is_empty()
                    && !profile.known_devices.contains(&transaction.device_id)
            }
            RuleType::UnknownLocation => {
                !profile.known_locations.is_empty()
                    && !profile.known_locations.contains(&transaction.location)
            }
            RuleType::HighAmount => transaction.amount > rule.threshold,
        }
    }

    pub fn should_block(&self, triggered_rules: &[String]) -> bool {
        triggered_rules
            .iter()
            .any(|rule_name| {
                self.rules
                    .iter()
                    .find(|r| &r.name == rule_name)
                    .map(|r| r.action == RuleAction::Block)
                    .unwrap_or(false)
            })
    }
}
