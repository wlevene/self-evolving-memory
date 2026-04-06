.PHONY: all build run test clean docker docker-run docker-stop help

# Default target
all: build

# Build all components
build: build-rust build-python build-typescript build-web
	@echo "✅ All components built successfully"

# Build Rust backend
build-rust:
	cd . && cargo build --release

# Build Python SDK
build-python:
	cd sdk/python && pip install -e .

# Build TypeScript SDK
build-typescript:
	cd sdk/typescript && npm ci && npm run build

# Build Web UI
build-web:
	cd web-ui && npm ci && npm run build

# Run server
run:
	cargo run --release

# Run in development mode (with hot reload)
dev:
	cargo watch -x run

# Run Web UI in development
dev-web:
	cd web-ui && npm run dev

# Run tests
test: test-rust test-python test-typescript
	@echo "✅ All tests passed"

test-rust:
	cargo test

test-python:
	cd sdk/python && pytest

test-typescript:
	cd sdk/typescript && npm test

# Clean build artifacts
clean:
	cargo clean
	rm -rf web-ui/dist
	rm -rf web-ui/node_modules
	rm -rf sdk/typescript/dist
	rm -rf sdk/typescript/node_modules

# Docker commands
docker:
	docker-compose build

docker-run:
	docker-compose up -d

docker-stop:
	docker-compose down

docker-logs:
	docker-compose logs -f

# Health check
health:
	curl -f http://localhost:3000/health

# API demo
demo:
	@echo "Creating memory..."
	curl -X POST http://localhost:3000/memories \
		-H "Content-Type: application/json" \
		-d '{"content":"Hello from Self-Evolving Memory!","pool":"explicit","type":"fact"}'
	@echo ""
	@echo "Listing memories..."
	curl http://localhost:3000/memories?limit=5
	@echo ""
	@echo "Getting stats..."
	curl http://localhost:3000/stats

# Install all dependencies
install:
	cargo fetch
	cd web-ui && npm ci
	cd sdk/typescript && npm ci

# Help
help:
	@echo "Self-Evolving Memory - Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build        Build all components"
	@echo "  run          Run the server"
	@echo "  dev          Run with hot reload (requires cargo-watch)"
	@echo "  dev-web      Run Web UI development server"
	@echo "  test         Run all tests"
	@echo "  clean        Clean build artifacts"
	@echo "  docker       Build Docker images"
	@echo "  docker-run   Start Docker containers"
	@echo "  docker-stop  Stop Docker containers"
	@echo "  health       Check server health"
	@echo "  demo         Run API demo"
	@echo "  install      Install dependencies"
	@echo "  help         Show this help"