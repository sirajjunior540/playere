# Build script for YouTube Player on Windows
# Creates release packages for Windows

Write-Host "🚀 Building YouTube Player Release Packages..." -ForegroundColor Green

# Ensure we're in the project directory
Set-Location $PSScriptRoot

# Clean previous builds
Write-Host "🧹 Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Check if tauri-cli is installed
try {
    cargo tauri --version | Out-Null
} catch {
    Write-Host "📦 Installing tauri-cli..." -ForegroundColor Yellow
    cargo install tauri-cli
}

# Build for Windows
Write-Host "🔨 Building for Windows..." -ForegroundColor Yellow
cargo tauri build

Write-Host "✅ Built for Windows" -ForegroundColor Green
Write-Host "📦 Packages created:" -ForegroundColor Cyan
Write-Host "  - MSI Installer: target\release\bundle\msi\YouTube Player_0.1.0_x64_en-US.msi"
Write-Host "  - Executable: target\release\YouTube Player.exe"
Write-Host ""
Write-Host "🎉 Build complete! Check the target\release\bundle directory for packages." -ForegroundColor Green