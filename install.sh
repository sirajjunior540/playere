#!/bin/bash

# Installation script for YouTube Player

echo "🎬 YouTube Player Installer"
echo "=========================="

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This installer is for macOS only."
    echo "For other platforms, please build from source."
    exit 1
fi

# Check if app bundle exists
APP_BUNDLE="target/release/bundle/YouTube Player.app"
if [ ! -d "$APP_BUNDLE" ]; then
    echo "❌ App bundle not found. Please run ./build-release.sh first."
    exit 1
fi

# Install location
INSTALL_DIR="/Applications"

echo ""
echo "This will install YouTube Player to $INSTALL_DIR"
echo "You may be prompted for your password."
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Remove old version if exists
    if [ -d "$INSTALL_DIR/YouTube Player.app" ]; then
        echo "🗑️  Removing old version..."
        sudo rm -rf "$INSTALL_DIR/YouTube Player.app"
    fi
    
    # Copy new version
    echo "📦 Installing YouTube Player..."
    sudo cp -R "$APP_BUNDLE" "$INSTALL_DIR/"
    
    echo "✅ Installation complete!"
    echo ""
    echo "You can now:"
    echo "1. Launch YouTube Player from Applications folder"
    echo "2. Or run from terminal: open '/Applications/YouTube Player.app'"
    echo ""
    echo "To use with a URL:"
    echo "open '/Applications/YouTube Player.app' --args 'https://www.youtube.com/watch?v=VIDEO_ID'"
else
    echo "❌ Installation cancelled."
fi