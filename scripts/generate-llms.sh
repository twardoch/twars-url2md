#!/bin/bash
# this_file: scripts/generate-llms.sh
# Generate llms.txt snapshot for AI tools using repomix

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

# Parse command line arguments
OUTPUT_FILE="llms.txt"
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Generate llms.txt snapshot for AI tools using repomix"
            echo ""
            echo "OPTIONS:"
            echo "  -o, --output FILE  Output file path (default: llms.txt)"
            echo "  --force            Force regeneration even if file exists"
            echo "  --help, -h         Show this help message"
            echo ""
            echo "Prerequisites:"
            echo "  - Node.js and npx must be installed"
            echo "  - repomix package will be installed via npx"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if output file exists and skip if not forcing
if [ -f "$OUTPUT_FILE" ] && [ "$FORCE" = false ]; then
    print_warning "Output file '$OUTPUT_FILE' already exists"
    print_status "Use --force to regenerate, or specify different output with -o"
    exit 0
fi

# Check if npx is available
if ! command -v npx >/dev/null 2>&1; then
    print_error "npx is not installed"
    print_status "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Check if repomix is available
print_status "Checking repomix availability..."
if ! npx -y repomix --version >/dev/null 2>&1; then
    print_error "Failed to run repomix"
    print_status "Make sure you have internet connection for npx to download repomix"
    exit 1
fi

# Generate llms.txt
print_status "Generating $OUTPUT_FILE with repomix..."
if npx repomix -o "$OUTPUT_FILE" . 2>&1; then
    print_success "Generated $OUTPUT_FILE successfully"

    # Show file size
    if command -v du >/dev/null 2>&1; then
        FILE_SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
        print_status "File size: $FILE_SIZE"
    fi

    # Show line count
    if command -v wc >/dev/null 2>&1; then
        LINE_COUNT=$(wc -l < "$OUTPUT_FILE")
        print_status "Lines: $LINE_COUNT"
    fi
else
    print_error "Failed to generate $OUTPUT_FILE"
    exit 1
fi
