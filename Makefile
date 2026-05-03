.PHONY: build test fmt clippy clean install check all dev audit coverage pre-commit help

# Default target
all: check build

# Build targets
build:
	cargo build

build-release:
	cargo build --release

# Test targets
test:
	cargo test

test-unit:
	cargo test --lib

test-integration:
	cargo test --test vault_test

test-verbose:
	cargo test -- --nocapture

# Code quality targets
fmt:
	cargo fmt

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings

lint: fmt clippy

# Security targets
audit:
	cargo audit

deny:
	cargo deny check

security: audit deny

# Coverage
coverage:
	cargo tarpaulin --out html

coverage-ci:
	cargo tarpaulin --out xml

# Development
dev:
	cargo watch -x run

dev-test:
	cargo watch -x test

# Clean
clean:
	cargo clean

# Install
install:
	cargo install --path .

# Run
run:
	cargo run -- $(ARGS)

# Pre-commit
pre-commit: fmt clippy test

# Check everything
check: fmt-check clippy test

# Help
help:
	@echo "Available targets:"
	@echo "  build          - Build the project"
	@echo "  build-release  - Build release version"
	@echo "  test           - Run all tests"
	@echo "  test-unit      - Run unit tests only"
	@echo "  test-integration - Run integration tests only"
	@echo "  test-verbose   - Run tests with output"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check formatting"
	@echo "  clippy         - Run clippy lints"
	@echo "  lint           - Run fmt and clippy"
	@echo "  audit          - Audit dependencies"
	@echo "  deny           - Run cargo-deny checks"
	@echo "  security       - Run all security checks"
	@echo "  coverage       - Generate coverage report"
	@echo "  dev            - Run in development mode"
	@echo "  dev-test       - Run tests in watch mode"
	@echo "  clean          - Clean build artifacts"
	@echo "  install        - Install binary"
	@echo "  run            - Run with arguments (ARGS=...)"
	@echo "  pre-commit     - Run pre-commit checks"
	@echo "  check          - Run all checks"
	@echo "  help           - Show this help"
