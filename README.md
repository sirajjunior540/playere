# YouTube Player - Minimal Desktop YouTube Player

A lightweight, distraction-free YouTube player built with Rust and Tauri. Watch YouTube videos in a frameless window without comments, suggestions, or other distractions.

## Features

- **Frameless Window**: Clean, borderless design
- **Drag to Move**: Click and drag the top area to move the window
- **Always on Top**: Window stays above other applications
- **URL Auto-conversion**: Automatically converts YouTube URLs to embedded format
- **Keyboard Shortcuts**: 
  - `ESC` to close the player
  - `Enter` to load video from URL input
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

### For Regular Users

Download the latest release for your platform:

- **macOS**: Download `YouTube Player.dmg` or `YouTube Player.app`
- **Windows**: Download `YouTube Player.msi` or `YouTube Player.exe`
- **Linux**: Download `YouTube Player.AppImage` or `YouTube Player.deb`

### Building from Source

#### Prerequisites

1. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install system dependencies:

**macOS**:
```bash
# Xcode Command Line Tools (if not installed)
xcode-select --install
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Windows**:
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

#### Build Steps

1. Clone the repository:
```bash
git clone https://github.com/yourusername/playere.git
cd playere
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage

### Running with Command Line

```bash
# Run with a YouTube URL
./playere "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# Run without URL (paste in the app)
./playere
```

### Using the Application

1. **Launch**: Double-click the app icon or run from terminal
2. **Load Video**: 
   - Paste a YouTube URL in the input field
   - Press Enter or click "Load Video"
3. **Move Window**: Click and drag the top area of the window
4. **Close**: Click the ✕ button (appears on hover) or press ESC

### Supported URL Formats

The player accepts various YouTube URL formats:
- `https://www.youtube.com/watch?v=VIDEO_ID`
- `https://youtu.be/VIDEO_ID`
- `https://www.youtube.com/embed/VIDEO_ID`
- `https://www.youtube.com/v/VIDEO_ID`

## Building Release Packages

### macOS (.app and .dmg)

```bash
# Build the app
cargo tauri build

# The .app will be in:
# target/release/bundle/macos/YouTube Player.app

# The .dmg will be in:
# target/release/bundle/dmg/YouTube Player.dmg
```

### Windows (.msi and .exe)

```bash
# Build the app
cargo tauri build

# The .msi installer will be in:
# target/release/bundle/msi/YouTube Player.msi

# The .exe will be in:
# target/release/YouTube Player.exe
```

### Linux (.deb, .AppImage)

```bash
# Build the app
cargo tauri build

# The .deb package will be in:
# target/release/bundle/deb/youtube-player_VERSION_amd64.deb

# The AppImage will be in:
# target/release/bundle/appimage/youtube-player_VERSION_amd64.AppImage
```

## Development

### Project Structure

```
playere/
├── src/
│   └── main.rs          # Rust backend code
├── dist/
│   └── index.html       # Frontend HTML/CSS/JS
├── icons/
│   └── icon.png         # App icon
├── tauri.conf.json      # Tauri configuration
├── Cargo.toml           # Rust dependencies
└── build.rs             # Build script
```

### Key Technologies

- **Rust**: Backend logic and window management
- **Tauri**: Framework for building desktop apps
- **HTML/CSS/JavaScript**: Frontend interface
- **YouTube Embed API**: For video playback

### Customization

You can customize the player by modifying:

1. **Window Size**: Edit `width` and `height` in `tauri.conf.json`
2. **Always on Top**: Toggle `alwaysOnTop` in `tauri.conf.json`
3. **Styling**: Modify CSS in `dist/index.html`
4. **Embed Parameters**: Edit the URL parameters in `src/main.rs`

## Troubleshooting

### Video Not Playing

1. Check your internet connection
2. Ensure the YouTube URL is valid
3. Some videos may have embedding restrictions

### Window Not Dragging

- Make sure to click and drag only the top area of the window
- The drag region is the top 40 pixels of the window

### Build Errors

1. Ensure all prerequisites are installed
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Uses YouTube's embedded player