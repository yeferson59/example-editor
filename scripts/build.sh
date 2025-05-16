#!/usr/bin/env bash
set -euo pipefail

# Build script for Rust Editor

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Check requirements
command -v rustc >/dev/null 2>&1 || { echo "rustc is required but not installed. Aborting." >&2; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed. Aborting." >&2; exit 1; }

# Parse arguments
BUILD_TYPE="debug"
RUN_TESTS=false
BUILD_DOCS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_TYPE="release"
            shift
            ;;
        --with-tests)
            RUN_TESTS=true
            shift
            ;;
        --with-docs)
            BUILD_DOCS=true
            shift
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

# Setup build directory
BUILD_DIR="target/${BUILD_TYPE}"
mkdir -p "$BUILD_DIR"

echo "Building Rust Editor (${BUILD_TYPE} build)..."

# Build the project
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release
else
    cargo build
fi

# Run tests if requested
if [[ "$RUN_TESTS" == true ]]; then
    echo "Running tests..."
    cargo test --workspace
    
    echo "Running clippy..."
    cargo clippy -- -D warnings
    
    echo "Checking formatting..."
    cargo fmt -- --check
fi

# Build documentation if requested
if [[ "$BUILD_DOCS" == true ]]; then
    echo "Building documentation..."
    cargo doc --no-deps --workspace
fi

# Copy assets and configurations
echo "Copying assets and configurations..."
mkdir -p "$BUILD_DIR/config"
cp -r config/* "$BUILD_DIR/config/"

echo "Build completed successfully!"
exit 0
