# Define OS-specific variables that work everywhere
macos_target_env := if os() != "windows" { "MACOSX_DEPLOYMENT_TARGET=11.0" } else { "" }
target := if os() == "windows" { "x86_64-pc-windows-msvc" } else { "universal-apple-darwin" }

# Set shell to PowerShell on Windows
set windows-shell := ["powershell.exe", "-c"]

# Clear screen function that works on all platforms
clear:
    @if ("{{os()}}" == "windows") { cls } else { clear }

# Debug target
debug: clear
    @echo
    cargo tauri dev

# Platform-specific release targets
release-macos: clear
    @echo
    {{macos_target_env}} cargo tauri build --target universal-apple-darwin --verbose

release-windows: clear
    @echo
    cargo tauri build --target x86_64-pc-windows-msvc --verbose

release-macos-production:
    @echo
    {{macos_target_env}} APPLE_SIGNING_IDENTITY="Developer ID Application: Öner Efe Dasguney (C4G7YDX6RS)" cargo tauri build --target universal-apple-darwin --verbose

# Default release target
release: clear
    @echo
    {{macos_target_env}} cargo tauri build --target {{target}} --verbose

# Clean target
clean:
    echo "Cleaning all build files..."
    cd src-tauri
    cargo clean

