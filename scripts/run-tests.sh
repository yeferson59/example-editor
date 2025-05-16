#!/usr/bin/env bash
set -euo pipefail

# Test runner script for Rust Editor

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Parse arguments
RUN_BENCHES=false
CHECK_COVERAGE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --with-benches)
            RUN_BENCHES=true
            shift
            ;;
        --with-coverage)
            CHECK_COVERAGE=true
            shift
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

echo "Running tests..."

# Run cargo test with all features
cargo test --workspace --all-features

# Run clippy
echo "Running clippy..."
cargo clippy --workspace --all-features -- -D warnings

# Check formatting
echo "Checking formatting..."
cargo fmt --all -- --check

# Run benchmarks if requested
if [[ "$RUN_BENCHES" == true ]]; then
    echo "Running benchmarks..."
    cargo bench
fi

# Check coverage if requested
if [[ "$CHECK_COVERAGE" == true ]]; then
    echo "Checking code coverage..."
    cargo install cargo-tarpaulin
    cargo tarpaulin --workspace --all-features --out Html
fi

echo "All tests completed successfully!"
exit 0
