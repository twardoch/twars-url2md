#!/bin/bash
# this_file: build.sh
# Build script for twars-url2md - A powerful CLI tool for converting web pages to Markdown

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."

    if ! command_exists cargo; then
        print_error "cargo is not installed. Please install Rust toolchain first."
        exit 1
    fi

    if ! command_exists rustc; then
        print_error "rustc is not installed. Please install Rust toolchain first."
        exit 1
    fi

    print_success "Prerequisites check passed"
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Build script for twars-url2md"
    echo ""
    echo "OPTIONS:"
    echo "  -h, --help       Show this help message"
    echo "  -f, --format     Format code only"
    echo "  -l, --lint       Run linter only"
    echo "  -t, --test       Run tests only"
    echo "  -b, --build      Build release binary only"
    echo "  -d, --dev        Build development binary only"
    echo "  -c, --clean      Clean build artifacts"
    echo "  -p, --package    Package for publishing"
    echo "  -a, --all        Run all steps (format, lint, test, build) [default]"
    echo "  --skip-format    Skip formatting step"
    echo "  --skip-lint      Skip linting step"
    echo "  --skip-test      Skip testing step"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all steps"
    echo "  $0 --format          # Format code only"
    echo "  $0 --test            # Run tests only"
    echo "  $0 --all --skip-test # Run all except tests"
}

# Default values
FORMAT=false
LINT=false
TEST=false
BUILD=false
DEV_BUILD=false
CLEAN=false
PACKAGE=false
ALL=true
SKIP_FORMAT=false
SKIP_LINT=false
SKIP_TEST=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
    -h | --help)
        show_help
        exit 0
        ;;
    -f | --format)
        FORMAT=true
        ALL=false
        ;;
    -l | --lint)
        LINT=true
        ALL=false
        ;;
    -t | --test)
        TEST=true
        ALL=false
        ;;
    -b | --build)
        BUILD=true
        ALL=false
        ;;
    -d | --dev)
        DEV_BUILD=true
        ALL=false
        ;;
    -c | --clean)
        CLEAN=true
        ALL=false
        ;;
    -p | --package)
        PACKAGE=true
        ALL=false
        ;;
    -a | --all)
        ALL=true
        ;;
    --skip-format)
        SKIP_FORMAT=true
        ;;
    --skip-lint)
        SKIP_LINT=true
        ;;
    --skip-test)
        SKIP_TEST=true
        ;;
    *)
        print_error "Unknown option: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
    esac
    shift
done

# Main execution
main() {
    print_status "Starting build process for twars-url2md..."
    check_prerequisites

    # Clean if requested
    if [[ $CLEAN == true ]]; then
        print_status "Cleaning build artifacts..."
        cargo clean
        print_success "Clean completed"
        return 0
    fi

    # Package if requested
    if [[ $PACKAGE == true ]]; then
        print_status "Packaging for publishing..."
        cargo package
        print_success "Package completed"
        return 0
    fi

    # Format code
    if [[ ($ALL == true && $SKIP_FORMAT == false) || $FORMAT == true ]]; then
        print_status "Formatting code..."
        if cargo fmt -- --check >/dev/null 2>&1; then
            print_success "Code is already formatted"
        else
            print_warning "Code needs formatting, applying..."
            cargo fmt
            print_success "Code formatting completed"
        fi
    fi

    # Run linter
    if [[ ($ALL == true && $SKIP_LINT == false) || $LINT == true ]]; then
        print_status "Running linter (clippy)..."
        cargo clippy --all-targets --all-features -- -D warnings
        print_success "Linting completed"
    fi

    # Run tests
    if [[ ($ALL == true && $SKIP_TEST == false) || $TEST == true ]]; then
        print_status "Running tests..."
        cargo test --all-features
        print_success "Tests completed"
    fi

    # Build development binary
    if [[ $DEV_BUILD == true ]]; then
        print_status "Building development binary..."
        cargo build
        print_success "Development build completed"
        print_status "Binary location: target/debug/twars-url2md"
    fi

    # Build release binary
    if [[ ($ALL == true) || $BUILD == true ]]; then
        print_status "Building release binary..."
        cargo build --release
        print_success "Release build completed"
        print_status "Binary location: target/release/twars-url2md"

        # Show binary info
        if [[ -f "target/release/twars-url2md" ]]; then
            BINARY_SIZE=$(du -h target/release/twars-url2md | cut -f1)
            print_status "Binary size: $BINARY_SIZE"
        fi
    fi

    print_success "Build process completed successfully!"
}

# Run main function
main
