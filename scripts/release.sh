#!/bin/bash
# this_file: scripts/release.sh

set -euo pipefail

# Release script for twars-url2md
# This script handles the complete release process including tagging and publishing

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Color output functions
red() { echo -e "\033[31m$1\033[0m"; }
green() { echo -e "\033[32m$1\033[0m"; }
yellow() { echo -e "\033[33m$1\033[0m"; }
blue() { echo -e "\033[34m$1\033[0m"; }

# Default options
DRY_RUN=false
SKIP_TESTS=false
SKIP_BUILD=false
FORCE=false
VERSION=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --version VERSION  Specify version to release (e.g., 1.2.3)"
            echo "  --dry-run          Show what would be done without making changes"
            echo "  --skip-tests       Skip running tests"
            echo "  --skip-build       Skip building the project"
            echo "  --force            Force release even if working directory is dirty"
            echo "  --help, -h         Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --version 1.2.3"
            echo "  $0 --version 1.2.3 --dry-run"
            echo "  $0 --version 1.2.3 --skip-tests"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validation
if [ -z "$VERSION" ]; then
    red "❌ Version is required. Use --version to specify it."
    echo "Example: $0 --version 1.2.3"
    exit 1
fi

# Validate version format (semantic versioning)
if ! echo "$VERSION" | grep -E '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$' > /dev/null; then
    red "❌ Invalid version format. Use semantic versioning (e.g., 1.2.3 or 1.2.3-beta.1)"
    exit 1
fi

# Check if we're in a git repository
if [ ! -d .git ]; then
    red "❌ Not in a git repository"
    exit 1
fi

# Check if working directory is clean
if [ "$FORCE" = false ] && [ -n "$(git status --porcelain)" ]; then
    red "❌ Working directory is not clean. Commit your changes first or use --force."
    git status --short
    exit 1
fi

# Check if we're on the main branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$FORCE" = false ]; then
    red "❌ Not on main branch (currently on $CURRENT_BRANCH). Use --force to override."
    exit 1
fi

# Check if tag already exists
if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    red "❌ Tag v$VERSION already exists"
    exit 1
fi

echo "🚀 Preparing release v$VERSION..."
echo "   Current branch: $CURRENT_BRANCH"
echo "   Dry run: $DRY_RUN"

# Function to execute command with dry run support
execute() {
    local cmd="$1"
    local desc="$2"
    
    echo "📋 $desc"
    if [ "$DRY_RUN" = true ]; then
        blue "   [DRY RUN] Would execute: $cmd"
    else
        echo "   Executing: $cmd"
        eval "$cmd"
    fi
}

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
if [ "$DRY_RUN" = true ]; then
    blue "   [DRY RUN] Would update version to $VERSION in Cargo.toml"
else
    # Update version - keeping it as 0.0.0 since we use git tags
    echo "   Version will be set from git tag during build"
fi

# Run tests
if [ "$SKIP_TESTS" = false ]; then
    echo "🧪 Running tests..."
    if [ "$DRY_RUN" = true ]; then
        blue "   [DRY RUN] Would run: ./scripts/test.sh"
    else
        if [ -f "./scripts/test.sh" ]; then
            ./scripts/test.sh
        else
            cargo test --all-features
        fi
    fi
fi

# Build the project
if [ "$SKIP_BUILD" = false ]; then
    echo "🏗️  Building project..."
    if [ "$DRY_RUN" = true ]; then
        blue "   [DRY RUN] Would run: ./scripts/build.sh"
    else
        if [ -f "./scripts/build.sh" ]; then
            ./scripts/build.sh
        else
            cargo build --release
        fi
    fi
fi

# Create git tag
execute "git tag -a v$VERSION -m 'Release version $VERSION'" "Creating git tag v$VERSION"

# Push tag to remote
execute "git push origin v$VERSION" "Pushing tag to remote"

# Push any pending commits
execute "git push origin $CURRENT_BRANCH" "Pushing current branch to remote"

# Create release notes
RELEASE_NOTES_FILE="release-notes-v$VERSION.md"
echo "📝 Creating release notes..."
if [ "$DRY_RUN" = true ]; then
    blue "   [DRY RUN] Would create release notes in $RELEASE_NOTES_FILE"
else
    cat > "$RELEASE_NOTES_FILE" << EOF
# Release Notes v$VERSION

## Changes

<!-- Add your changes here -->

## Installation

### From GitHub Releases

Download the appropriate binary for your platform from the [releases page](https://github.com/twardoch/twars-url2md/releases/tag/v$VERSION).

### From Source

\`\`\`bash
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md
git checkout v$VERSION
cargo build --release
\`\`\`

### From crates.io

\`\`\`bash
cargo install twars-url2md
\`\`\`

## Checksums

<!-- Checksums will be added by CI -->

EOF
    echo "   Created: $RELEASE_NOTES_FILE"
fi

# Show next steps
echo ""
green "🎉 Release v$VERSION prepared successfully!"
echo ""
echo "Next steps:"
echo "1. Edit $RELEASE_NOTES_FILE to add changelog information"
echo "2. The GitHub Actions workflow will automatically:"
echo "   - Build multi-platform binaries"
echo "   - Create a GitHub release"
echo "   - Publish to crates.io"
echo "   - Upload release artifacts"
echo ""
echo "Monitor the progress at:"
echo "https://github.com/twardoch/twars-url2md/actions"
echo ""
echo "Once the release is published, you can:"
echo "- Announce the release"
echo "- Update documentation"
echo "- Notify users"

if [ "$DRY_RUN" = true ]; then
    echo ""
    yellow "⚠️  This was a dry run. No actual changes were made."
    echo "Run without --dry-run to execute the release."
fi