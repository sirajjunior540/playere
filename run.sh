#!/bin/bash

# Disable icon loading completely
export TAURI_SKIP_DEVSERVER_CHECK=true
export WEBKIT_DISABLE_COMPOSITING_MODE=1

# Run the app
cargo run --release "$@"