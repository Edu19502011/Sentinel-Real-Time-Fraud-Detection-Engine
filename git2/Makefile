.PHONY: help build run test clean docker-up docker-down load-test

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in release mode
	cargo build --release

run: ## Run the application
	cargo run --release

dev: ## Run in development mode with auto-reload
	cargo watch -x run

test: ## Run all tests
	cargo test

test-api: ## Run API integration tests
	./scripts/test-api.sh

lint: ## Run clippy linter
	cargo clippy -- -D warnings

format: ## Format code with rustfmt
	cargo fmt

check: lint format test ## Run all checks (lint, format, test)

docker-build: ## Build Docker image
	docker build -t fraud-detection-engine:latest .

docker-up: ## Start all services with Docker Compose
	docker-compose up -d

docker-down: ## Stop all services
	docker-compose down

docker-logs: ## Show logs from all services
	docker-compose logs -f

setup: ## Setup development environment
	./scripts/setup.sh

load-test: ## Run k6 load tests
	cd k6 && k6 run load-test.js

benchmark: ## Run Rust benchmarks
	cargo bench

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

install: ## Install the binary
	cargo install --path .

docs: ## Generate and open documentation
	cargo doc --open

watch: ## Watch for changes and run tests
	cargo watch -x test

redis-cli: ## Connect to Redis CLI
	docker-compose exec redis redis-cli

clickhouse-cli: ## Connect to ClickHouse CLI
	docker-compose exec clickhouse clickhouse-client

metrics: ## Show current metrics
	curl -s http://localhost:8080/metrics

health: ## Check API health
	curl -s http://localhost:8080/health | jq .
