#!/usr/bin/env bash
set -e

BIN_NAME="rust-cli"
INSTALL_DIR="$HOME/.local/bin"

echo "Building rust-cli..."
cargo build --release

echo "Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "target/release/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "✅ Installed successfully"
echo "Run: rust-cli --help"