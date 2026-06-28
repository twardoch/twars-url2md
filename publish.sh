#!/usr/bin/env bash
# this_file: publish.sh
# Build, bump version, and publish twars-url2md to crates.io.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> Building..."
bash "$SCRIPT_DIR/build.sh"

echo "==> Bumping version..."
uvx gitnextver@latest .

echo "==> Publishing to crates.io..."
cargo publish

echo "Published."
