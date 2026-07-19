#!/bin/bash
set -e

echo "=== Decimal Exercise - Verification Loop ==="

echo ""
echo "[1/3] Code formatting (cargo fmt --all -- --check)..."
cargo fmt --all -- --check

echo ""
echo "[2/3] Compilation & Linting (cargo clippy)..."
cargo clippy -- -W clippy::pedantic


echo ""
echo "[3/3] Tests (cargo test)..."
cargo test --manifest-path Cargo.toml

# cover by the above checks, but can be uncommented for a more thorough verification
# echo ""
# echo "Overall verification..."
# cargo test --manifest-path Cargo.toml && cargo fmt --all -- --check && cargo clippy

echo ""
echo "✓ All checks passed!"
