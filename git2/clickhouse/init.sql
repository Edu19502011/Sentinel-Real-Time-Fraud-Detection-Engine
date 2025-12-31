-- Create database
CREATE DATABASE IF NOT EXISTS fraud_detection;

-- Transactions table
CREATE TABLE IF NOT EXISTS fraud_detection.transactions (
    transaction_id UUID,
    user_id String,
    amount Float64,
    merchant String,
    location String,
    device_id String,
    status String,
    risk_score Float64,
    rules_triggered Array(String),
    timestamp DateTime DEFAULT now(),
    processing_time_ms UInt64
) ENGINE = MergeTree()
ORDER BY (timestamp, user_id)
PARTITION BY toYYYYMM(timestamp)
TTL timestamp + INTERVAL 90 DAY;

-- Fraud events table
CREATE TABLE IF NOT EXISTS fraud_detection.fraud_events (
    event_id UUID,
    user_id String,
    transaction_id UUID,
    fraud_type String,
    risk_score Float64,
    timestamp DateTime DEFAULT now()
) ENGINE = MergeTree()
ORDER BY (timestamp, user_id)
PARTITION BY toYYYYMM(timestamp)
TTL timestamp + INTERVAL 180 DAY;

-- Materialized view for fraud statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS fraud_detection.fraud_stats_hourly
ENGINE = SummingMergeTree()
ORDER BY (hour, status)
AS SELECT
    toStartOfHour(timestamp) as hour,
    status,
    count() as transaction_count,
    avg(risk_score) as avg_risk_score,
    avg(processing_time_ms) as avg_processing_time
FROM fraud_detection.transactions
GROUP BY hour, status;
