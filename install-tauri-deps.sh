#!/bin/bash

# Tauri System Dependencies Installation Script
# Run this script with sudo: sudo bash install-tauri-deps.sh

set -e

echo "Installing Tauri system dependencies..."

apt-get update

apt-get install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf

echo "✅ System dependencies installed successfully!"
echo ""
echo "Next steps:"
echo "1. Source Cargo environment: . \"\$HOME/.cargo/env\""
echo "2. Install Tauri CLI: cargo install tauri-cli --version \"^2.0.0\""
echo "3. Run: npm run tauri init"
