#!/usr/bin/env bash
set -euo pipefail

# Installation script for Rust Editor

# Default installation directory
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="$HOME/.config/rust-editor"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --prefix=*)
            INSTALL_DIR="${1#*=}"
            shift
            ;;
        --config-dir=*)
            CONFIG_DIR="${1#*=}"
            shift
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Check if the binary exists
if [[ ! -f "target/release/rust-editor" ]]; then
    echo "Binary not found. Building project..."
    cargo build --release
fi

# Create installation directories
sudo mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"/{plugins,themes,config}

# Install the binary
sudo cp "target/release/rust-editor" "$INSTALL_DIR/"

# Copy default configurations
cp -r config/* "$CONFIG_DIR/config/"

# Set up permissions
sudo chmod 755 "$INSTALL_DIR/rust-editor"

echo "Installation completed successfully!"
echo "Binary installed to: $INSTALL_DIR/rust-editor"
echo "Configuration directory: $CONFIG_DIR"

exit 0
