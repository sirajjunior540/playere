#!/bin/bash

echo "🔍 Debug script for playere crash issues"
echo "========================================"

# Check we're in the right directory
echo "1. Checking directory..."
pwd
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Not in a Rust project directory. Please cd to the playere project folder."
    exit 1
fi
echo "✅ In correct directory"

# Check configuration files
echo ""
echo "2. Checking configuration..."
if [[ ! -f "tauri.conf.json" ]]; then
    echo "❌ tauri.conf.json not found"
    exit 1
fi

if [[ ! -f "dist/index.html" ]]; then
    echo "❌ dist/index.html not found"
    exit 1
fi
echo "✅ Configuration files exist"

# Check Rust/Cargo versions
echo ""
echo "3. Checking versions..."
cargo --version
rustc --version
echo "macOS version: $(sw_vers -productVersion)"

# Clean build
echo ""
echo "4. Performing clean build..."
cargo clean
echo "✅ Cleaned previous build"

echo ""
echo "5. Building release version..."
if cargo build --release; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Test run
echo ""
echo "6. Testing app (will run for 3 seconds)..."
./target/release/playere "https://www.youtube.com/watch?v=dQw4w9WgXcQ" &
APP_PID=$!
sleep 3

if kill -0 $APP_PID 2>/dev/null; then
    echo "✅ App is running successfully!"
    kill $APP_PID
    echo ""
    echo "🎉 SUCCESS: The app works correctly!"
    echo ""
    echo "Usage:"
    echo "  ./target/release/playere 'https://www.youtube.com/watch?v=VIDEO_ID'"
    echo "  cargo run --release 'https://www.youtube.com/watch?v=VIDEO_ID'"
else
    echo "❌ App crashed during startup"
    echo ""
    echo "💡 Troubleshooting steps:"
    echo "1. Make sure you're running from the playere project directory"
    echo "2. Try: cargo clean && cargo build --release"
    echo "3. Check Console.app for crash logs"
    echo "4. Ensure no antivirus is blocking the app"
fi