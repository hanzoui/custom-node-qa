.PHONY: help install test validate format check build clean

help:
	@echo "Available commands:"
	@echo "  make install    - Install CLI locally"
	@echo "  make test       - Run tests"
	@echo "  make validate   - Validate all projects"
	@echo "  make format     - Format code"
	@echo "  make check      - Check code quality"
	@echo "  make build      - Build release binary"
	@echo "  make clean      - Clean build artifacts"

install:
	cargo install --path cli

test:
	cargo test --manifest-path cli/Cargo.toml

validate:
	cargo run --release --manifest-path cli/Cargo.toml -- validate

format:
	cargo fmt --manifest-path cli/Cargo.toml
	npm run format

check:
	cargo clippy --manifest-path cli/Cargo.toml -- -D warnings
	npm run format:check

build:
	cargo build --release --manifest-path cli/Cargo.toml

clean:
	cargo clean --manifest-path cli/Cargo.toml
