mod api;
mod engine;
mod models;
mod rules;
mod storage;

use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting Fraud Detection Engine...");

    // Initialize storage
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let redis_client = storage::redis::RedisClient::new(&redis_url).await?;

    // Initialize rule engine
    let rule_engine = rules::RuleEngine::load_from_file("config/rules.json")?;

    // Initialize fraud engine
    let fraud_engine = Arc::new(engine::FraudEngine::new(redis_client, rule_engine));

    // Build API routes
    let app = Router::new()
        .route("/api/v1/transaction", post(api::handlers::process_transaction))
        .route("/health", axum::routing::get(api::handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(fraud_engine);

    // Start server
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("✅ Server listening on {}", addr);
    tracing::info!("📊 Metrics available at http://{}/metrics", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
