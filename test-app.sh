#!/bin/bash

# Test script for YouTube Player
echo "🧪 Testing YouTube Player..."

# Test 1: Build the app
echo "1️⃣ Building the app..."
if cargo build --release; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Test 2: Run the app briefly to check for crashes
echo "2️⃣ Testing app startup..."
./target/release/playere "https://www.youtube.com/watch?v=dQw4w9WgXcQ" &
APP_PID=$!
sleep 3

if kill -0 $APP_PID 2>/dev/null; then
    echo "✅ App started successfully"
    kill $APP_PID
else
    echo "❌ App crashed on startup"
    exit 1
fi

# Test 3: Test URL parsing
echo "3️⃣ Testing URL parsing..."
TEST_URLS=(
    "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
    "https://youtu.be/dQw4w9WgXcQ"
    "https://www.youtube.com/embed/dQw4w9WgXcQ"
)

for url in "${TEST_URLS[@]}"; do
    ./target/release/playere "$url" &
    PID=$!
    sleep 1
    if kill -0 $PID 2>/dev/null; then
        echo "✅ URL format works: $url"
        kill $PID
    else
        echo "❌ URL format failed: $url"
    fi
done

# Test 4: Check app bundle
echo "4️⃣ Testing app bundle..."
if [ -d "target/release/bundle/YouTube Player.app" ]; then
    echo "✅ App bundle exists"
    # Test the bundle
    open "target/release/bundle/YouTube Player.app" --args "https://www.youtube.com/watch?v=dQw4w9WgXcQ" &
    sleep 2
    if pgrep -f "YouTube Player" > /dev/null; then
        echo "✅ App bundle runs successfully"
        pkill -f "YouTube Player"
    else
        echo "❌ App bundle failed to start"
    fi
else
    echo "❌ App bundle not found"
fi

echo ""
echo "🎉 All tests completed!"
echo ""
echo "📦 Ready to use:"
echo "   Binary: ./target/release/playere"
echo "   App Bundle: ./target/release/bundle/YouTube Player.app"
echo ""
echo "🚀 Usage:"
echo "   ./target/release/playere 'https://www.youtube.com/watch?v=VIDEO_ID'"
echo "   open './target/release/bundle/YouTube Player.app' --args 'https://www.youtube.com/watch?v=VIDEO_ID'"