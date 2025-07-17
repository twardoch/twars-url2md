#!/bin/bash
# this_file: scripts/build.sh

set -euo pipefail

# Build script for twars-url2md
# This script handles building the project with proper version information

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "🔨 Building twars-url2md..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Check if we're in a git repository
if [ -d .git ]; then
    echo "📋 Git repository detected"
    
    # Get git version info
    GIT_VERSION=$(git describe --tags --always --dirty 2>/dev/null || echo "unknown")
    GIT_COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
    GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    
    echo "   Version: $GIT_VERSION"
    echo "   Commit: ${GIT_COMMIT:0:8}"
    echo "   Branch: $GIT_BRANCH"
else
    echo "⚠️  Not in a git repository"
fi

# Build in release mode
echo "🏗️  Building release binary..."
cargo build --release

# Get the binary path
BINARY_PATH="target/release/twars-url2md"

if [ -f "$BINARY_PATH" ]; then
    echo "✅ Build successful!"
    echo "   Binary: $BINARY_PATH"
    echo "   Size: $(du -h "$BINARY_PATH" | cut -f1)"
    
    # Show version info
    echo "📊 Version information:"
    "$BINARY_PATH" --version 2>/dev/null || echo "   (Version command not available)"
else
    echo "❌ Build failed - binary not found"
    exit 1
fi

# Create builds directory if it doesn't exist
mkdir -p builds

# Copy binary to builds directory with version info
if [ -d .git ]; then
    BUILD_NAME="twars-url2md-${GIT_VERSION}-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"
else
    BUILD_NAME="twars-url2md-local-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)"
fi

cp "$BINARY_PATH" "builds/$BUILD_NAME"
echo "📦 Binary copied to: builds/$BUILD_NAME"

echo "🎉 Build completed successfully!"