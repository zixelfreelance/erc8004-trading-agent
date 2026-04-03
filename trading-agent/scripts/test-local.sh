#!/usr/bin/env bash
# Local test suite — mirrors what CI runs on GitHub
set -euo pipefail

echo "=== fmt ==="
cargo fmt -- --check

echo "=== clippy ==="
cargo clippy --release -- -D warnings

echo "=== test ==="
cargo test --release

echo ""
echo "All checks passed."
