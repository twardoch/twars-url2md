#!/bin/bash
# this_file: install.sh

# Installation script for twars-url2md
# This script automatically downloads and installs the latest release

set -euo pipefail

# Default values
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
REPO="twardoch/twars-url2md"
GITHUB_API_URL="https://api.github.com"
FORCE=false
VERSION=""

# Color output functions
red() { echo -e "\033[31m$1\033[0m"; }
green() { echo -e "\033[32m$1\033[0m"; }
yellow() { echo -e "\033[33m$1\033[0m"; }
blue() { echo -e "\033[34m$1\033[0m"; }

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --help|-h)
            echo "twars-url2md installer"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --install-dir DIR  Install directory (default: /usr/local/bin)"
            echo "  --version VERSION  Specific version to install (default: latest)"
            echo "  --force           Force installation even if already installed"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0"
            echo "  $0 --install-dir ~/.local/bin"
            echo "  $0 --version v1.4.2"
            echo "  curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash"
            exit 0
            ;;
        *)
            red "❌ Unknown option: $1"
            exit 1
            ;;
    esac
done

# Detect platform
detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "$os" in
        Linux)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-aarch64" ;;
                *) red "❌ Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64) echo "macos-aarch64" ;;
                *) red "❌ Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            case "$arch" in
                x86_64) echo "windows-x86_64" ;;
                *) red "❌ Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        *)
            red "❌ Unsupported operating system: $os"
            exit 1
            ;;
    esac
}

# Get latest release version
get_latest_version() {
    local version
    version=$(curl -fsSL "$GITHUB_API_URL/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$version" ]; then
        red "❌ Failed to get latest version"
        exit 1
    fi
    echo "$version"
}

# Download and install binary
install_binary() {
    local version="$1"
    local platform="$2"
    local binary_name="twars-url2md"
    local archive_name="twars-url2md-$platform"
    local download_url extension
    
    # Determine file extension and download URL
    case "$platform" in
        windows-*)
            extension=".zip"
            binary_name="twars-url2md.exe"
            ;;
        *)
            extension=".tar.gz"
            ;;
    esac
    
    archive_name="$archive_name$extension"
    download_url="https://github.com/$REPO/releases/download/$version/$archive_name"
    
    echo "📥 Downloading $archive_name..."
    
    # Create temporary directory
    local temp_dir
    temp_dir=$(mktemp -d)
    trap "rm -rf '$temp_dir'" EXIT
    
    # Download archive
    if ! curl -fsSL -o "$temp_dir/$archive_name" "$download_url"; then
        red "❌ Failed to download $archive_name"
        red "   URL: $download_url"
        exit 1
    fi
    
    # Extract archive
    echo "📦 Extracting binary..."
    cd "$temp_dir"
    case "$extension" in
        .zip)
            if command -v unzip >/dev/null 2>&1; then
                unzip -q "$archive_name"
            else
                red "❌ unzip is required but not installed"
                exit 1
            fi
            ;;
        .tar.gz)
            tar -xzf "$archive_name"
            ;;
    esac
    
    # Check if binary exists
    if [ ! -f "$binary_name" ]; then
        red "❌ Binary not found in archive"
        exit 1
    fi
    
    # Make binary executable
    chmod +x "$binary_name"
    
    # Create install directory if it doesn't exist
    if [ ! -d "$INSTALL_DIR" ]; then
        echo "📁 Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi
    
    # Install binary
    echo "🚀 Installing to $INSTALL_DIR..."
    if ! mv "$binary_name" "$INSTALL_DIR/"; then
        red "❌ Failed to install binary to $INSTALL_DIR"
        red "   You may need to run with sudo or choose a different install directory"
        exit 1
    fi
    
    green "✅ Successfully installed twars-url2md $version"
    echo "   Binary: $INSTALL_DIR/twars-url2md"
}

# Check if already installed
check_existing() {
    local binary_path="$INSTALL_DIR/twars-url2md"
    if [ -f "$binary_path" ] && [ "$FORCE" = false ]; then
        yellow "⚠️  twars-url2md is already installed at $binary_path"
        local current_version
        current_version=$("$binary_path" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo "unknown")
        echo "   Current version: $current_version"
        echo "   Use --force to overwrite"
        exit 0
    fi
}

# Main installation
main() {
    echo "🔧 Installing twars-url2md..."
    echo ""
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    echo "Platform: $platform"
    
    # Get version
    local version
    if [ -n "$VERSION" ]; then
        version="$VERSION"
        echo "Version: $version (specified)"
    else
        version=$(get_latest_version)
        echo "Version: $version (latest)"
    fi
    
    echo "Install directory: $INSTALL_DIR"
    echo ""
    
    # Check if already installed
    check_existing
    
    # Install binary
    install_binary "$version" "$platform"
    
    # Verify installation
    echo ""
    echo "🔍 Verifying installation..."
    local binary_path="$INSTALL_DIR/twars-url2md"
    if [ -f "$binary_path" ]; then
        local installed_version
        installed_version=$("$binary_path" --version 2>/dev/null || echo "Version check failed")
        echo "   $installed_version"
        green "✅ Installation verified"
    else
        red "❌ Installation verification failed"
        exit 1
    fi
    
    # Show usage information
    echo ""
    echo "🎉 Installation complete!"
    echo ""
    echo "Usage examples:"
    echo "  twars-url2md --input urls.txt --output ./markdown"
    echo "  echo 'https://example.com' | twars-url2md --stdin"
    echo "  twars-url2md --help"
    echo ""
    echo "Note: Make sure $INSTALL_DIR is in your PATH environment variable."
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        yellow "⚠️  $INSTALL_DIR is not in your PATH"
        echo "   Add it to your shell profile:"
        echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
}

# Run main function
main "$@"