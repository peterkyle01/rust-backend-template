.PHONY: help build run test clean migrate docker-build docker-run

# Default target
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Development
build: ## Build the project
	cargo build

run: ## Run the project locally
	cargo run

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

# Database
migrate: ## Run database migrations
	sqlx migrate run

# Docker
docker-build: ## Build Docker image
	docker build -t rust-backend .

docker-run: ## Run Docker container
	docker run -p 8080:8080 --env-file .env rust-backend

# Production
release: ## Build release binary
	cargo build --release

# Setup and Run Scripts
setup: ## Run complete project setup
	@echo "🚀 Rust MVC Backend Setup"
	@echo "========================"
	@if [ ! -f .env ]; then \
		echo "❌ .env file not found"; \
		echo "Please copy your Neon connection string to .env file:"; \
		echo ""; \
		echo "DATABASE_URL=postgresql://username:password@your-hostname.neon.tech/database?sslmode=require"; \
		echo "JWT_SECRET=your-super-secret-jwt-key"; \
		echo ""; \
		exit 1; \
	fi
	@if grep -q "your-hostname.neon.tech" .env; then \
		echo "⚠️  Please update DATABASE_URL in .env with your actual Neon connection string"; \
		exit 1; \
	fi
	@echo "✅ Environment configuration found"
	@if ! command -v sqlx &> /dev/null; then \
		echo "📦 Installing SQLx CLI..."; \
		cargo install sqlx-cli --no-default-features --features rustls,postgres; \
	fi
	@echo "🗄️ Running database migrations..."
	@sqlx migrate run
	@if [ $$? -eq 0 ]; then \
		echo "✅ Database migrations completed"; \
	else \
		echo "❌ Database migrations failed"; \
		echo "Please check your DATABASE_URL in .env file"; \
		exit 1; \
	fi
	@echo "🔨 Building the project..."
	@cargo build
	@if [ $$? -eq 0 ]; then \
		echo "✅ Build completed successfully"; \
		echo ""; \
		echo "🎉 Setup complete! You can now run:"; \
		echo "   make run-binary    # Start with environment variables"; \
		echo "   make run           # Start with cargo run"; \
		echo "   make test          # Run tests"; \
		echo ""; \
		echo "The API will be available at http://localhost:8080"; \
	else \
		echo "❌ Build failed"; \
		exit 1; \
	fi

run-prod: ## Run the release binary with environment variables from .env
	@if [ ! -f .env ]; then \
		echo "❌ .env file not found"; \
		echo "Please create a .env file with your configuration"; \
		exit 1; \
	fi
	@echo "Starting Rust backend (Release) with configuration from .env file:"
	@set -a && . ./.env && set +a && \
	echo "HOST: $${HOST:-0.0.0.0}" && \
	echo "PORT: $${PORT:-8080}" && \
	echo "DATABASE_URL: $$DATABASE_URL" && \
	echo "JWT_EXPIRY_HOURS: $${JWT_EXPIRY_HOURS:-24}" && \
	echo "BCRYPT_COST: $${BCRYPT_COST:-12}" && \
	echo "RUST_LOG: $${RUST_LOG:-info}" && \
	echo "" && \
	echo "Running the release binary..." && \
	./target/release/rust-backend

# Linting and formatting
fmt: ## Format code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

check: fmt lint test ## Run all checks
