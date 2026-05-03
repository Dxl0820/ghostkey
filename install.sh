#!/bin/bash

# GhostKey Installation Script

set -e

echo "Installing GhostKey..."

# Check if Rust is installed
if ! command -v rustup &> /dev/null; then
    echo "Error: Rust is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "Rust version: $RUST_VERSION"

# Build the project
echo "Building GhostKey..."
cargo build --release

# Install the binary
echo "Installing binary..."
cargo install --path .

echo ""
echo "✓ GhostKey installed successfully!"
echo ""
echo "Get started:"
echo "  ghostkey init          # Initialize a new vault"
echo "  ghostkey add <name>    # Add a credential"
echo "  ghostkey list          # List all credentials"
echo ""
echo "For more information, run: ghostkey --help"
