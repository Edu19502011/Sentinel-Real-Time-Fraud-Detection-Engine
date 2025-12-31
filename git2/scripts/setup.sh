#!/bin/bash

echo "🚀 Setting up Fraud Detection Engine"
echo "===================================="

# Check prerequisites
echo -e "\n📋 Checking prerequisites..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose not found. Please install Docker Compose first."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

echo "✅ All prerequisites met!"

# Start infrastructure
echo -e "\n🐳 Starting infrastructure (Redis, Redpanda, ClickHouse, Grafana)..."
docker-compose up -d redis redpanda clickhouse prometheus grafana

echo -e "\n⏳ Waiting for services to be ready..."
sleep 10

# Check service health
echo -e "\n🏥 Checking service health..."

if docker-compose ps | grep -q "redis.*Up"; then
    echo "✅ Redis is running"
else
    echo "❌ Redis failed to start"
fi

if docker-compose ps | grep -q "redpanda.*Up"; then
    echo "✅ Redpanda is running"
else
    echo "❌ Redpanda failed to start"
fi

if docker-compose ps | grep -q "clickhouse.*Up"; then
    echo "✅ ClickHouse is running"
else
    echo "❌ ClickHouse failed to start"
fi

# Build Rust application
echo -e "\n🦀 Building Rust application..."
cargo build --release

echo -e "\n✅ Setup complete!"
echo -e "\n📊 Access points:"
echo "  - API: http://localhost:8080"
echo "  - Grafana: http://localhost:3000 (admin/admin)"
echo "  - Prometheus: http://localhost:9090"
echo "  - ClickHouse: http://localhost:8123"

echo -e "\n🎯 Next steps:"
echo "  1. Run the application: cargo run --release"
echo "  2. Test the API: ./scripts/test-api.sh"
echo "  3. Run load tests: cd k6 && k6 run load-test.js"
