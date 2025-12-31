use crate::engine::FraudEngine;
use crate::models::{Transaction, TransactionResponse};
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn process_transaction(
    State(engine): State<Arc<FraudEngine>>,
    Json(transaction): Json<Transaction>,
) -> Result<Json<TransactionResponse>, StatusCode> {
    let start = std::time::Instant::now();

    // Check transaction
    let result = engine
        .check_transaction(&transaction)
        .await
        .map_err(|e| {
            tracing::error!("Error checking transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let status = engine.determine_status(&result);
    let processing_time = start.elapsed().as_millis() as u64;

    tracing::info!(
        user_id = %transaction.user_id,
        amount = %transaction.amount,
        status = ?status,
        risk_score = %result.risk_score,
        processing_time_ms = %processing_time,
        "Transaction processed"
    );

    Ok(Json(TransactionResponse {
        transaction_id: Uuid::new_v4(),
        status,
        risk_score: result.risk_score,
        rules_triggered: result.rules_triggered,
        processing_time_ms: processing_time,
    }))
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "fraud-detection-engine",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
