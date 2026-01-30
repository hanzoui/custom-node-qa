#!/usr/bin/env bash
set -euo pipefail

echo "🔧 Installing comfy-qa CLI..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed"
    echo "Install from: https://rustup.rs/"
    exit 1
fi

# Build and install
cargo install --path cli

echo "✅ comfy-qa installed successfully"
echo ""
echo "Try: comfy-qa --help"
