.PHONY: help build test lint format check clean install

help:
	@echo "Available commands:"
	@echo "  make build    - Build the project"
	@echo "  make test     - Run tests"
	@echo "  make lint     - Run clippy linter"
	@echo "  make format   - Format code with rustfmt"
	@echo "  make check    - Run format check, lint, and tests"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make install  - Install the CLI locally"

build:
	cargo build --release

test:
	cargo test --verbose

lint:
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt

format-check:
	cargo fmt -- --check

check: format-check lint test
	@echo "✓ All checks passed!"

clean:
	cargo clean

install: build
	cargo install --path .
