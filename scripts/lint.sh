#!/bin/bash
# this_file: scripts/lint.sh
# Run formatters and linters for code quality checks

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

# Check prerequisites
if ! command -v cargo >/dev/null 2>&1; then
    print_error "cargo is not installed. Please install Rust toolchain first."
    exit 1
fi

# Parse command line arguments
FIX=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --fix)
            FIX=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Run formatters and linters for code quality checks"
            echo ""
            echo "OPTIONS:"
            echo "  --fix        Apply automatic fixes (format code, auto-fix clippy warnings)"
            echo "  --verbose    Enable verbose output"
            echo "  --help, -h   Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Set verbosity flags
if [ "$VERBOSE" = true ]; then
    CARGO_VERBOSE_FLAG="--verbose"
else
    CARGO_VERBOSE_FLAG=""
fi

print_status "Running code quality checks..."

# Format checking/fixing
if [ "$FIX" = true ]; then
    print_status "Formatting code..."
    cargo fmt $CARGO_VERBOSE_FLAG
    print_success "Code formatted"
else
    print_status "Checking code format..."
    if cargo fmt --check $CARGO_VERBOSE_FLAG; then
        print_success "Code format check passed"
    else
        print_error "Code format check failed"
        print_warning "Run with --fix to format code automatically"
        exit 1
    fi
fi

# Clippy linting
print_status "Running clippy..."
if [ "$FIX" = true ]; then
    # Try to auto-fix what clippy can fix
    if cargo clippy --fix --allow-dirty --all-targets --all-features $CARGO_VERBOSE_FLAG -- -D warnings 2>/dev/null; then
        print_success "Clippy passed (with auto-fixes applied)"
    else
        # If auto-fix fails, just run regular clippy
        cargo clippy --all-targets --all-features $CARGO_VERBOSE_FLAG -- -D warnings
        print_success "Clippy passed"
    fi
else
    cargo clippy --all-targets --all-features $CARGO_VERBOSE_FLAG -- -D warnings
    print_success "Clippy passed"
fi

print_success "All code quality checks passed!"
