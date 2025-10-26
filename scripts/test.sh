#!/bin/bash
# this_file: scripts/test.sh

set -euo pipefail

# Test script for twars-url2md
# This script runs the complete test suite including unit, integration, and benchmark tests

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "🧪 Running twars-url2md test suite..."

# Default test options
RUN_UNIT_TESTS=true
RUN_INTEGRATION_TESTS=true
RUN_BENCHMARK_TESTS=false
RUN_CLIPPY=true
RUN_FORMAT_CHECK=true
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --unit-only)
            RUN_INTEGRATION_TESTS=false
            RUN_BENCHMARK_TESTS=false
            shift
            ;;
        --integration-only)
            RUN_UNIT_TESTS=false
            RUN_BENCHMARK_TESTS=false
            shift
            ;;
        --benchmark|--bench)
            RUN_BENCHMARK_TESTS=true
            shift
            ;;
        --no-clippy)
            RUN_CLIPPY=false
            shift
            ;;
        --no-format)
            RUN_FORMAT_CHECK=false
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --unit-only        Run only unit tests"
            echo "  --integration-only Run only integration tests"
            echo "  --benchmark        Run benchmark tests"
            echo "  --no-clippy        Skip clippy linting"
            echo "  --no-format        Skip format checking"
            echo "  --verbose, -v      Enable verbose output"
            echo "  --help, -h         Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
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

# Color output functions
red() { echo -e "\033[31m$1\033[0m"; }
green() { echo -e "\033[32m$1\033[0m"; }
yellow() { echo -e "\033[33m$1\033[0m"; }
blue() { echo -e "\033[34m$1\033[0m"; }

# Function to run command with status reporting
run_test() {
    local test_name="$1"
    local command="$2"
    
    echo "🔍 Running $test_name..."
    if eval "$command"; then
        green "✅ $test_name passed"
        return 0
    else
        red "❌ $test_name failed"
        return 1
    fi
}

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

# Build script help smoke test (fast, uses stub cargo)
if run_test "build script help" "bash tests/scripts_build_help_test.sh"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Format checking
if [ "$RUN_FORMAT_CHECK" = true ]; then
    if run_test "format check" "cargo fmt --check $CARGO_VERBOSE_FLAG"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
        yellow "💡 Run 'cargo fmt' to fix formatting issues"
    fi
fi

# Clippy linting
if [ "$RUN_CLIPPY" = true ]; then
    if run_test "clippy linting" "cargo clippy --all-targets --all-features $CARGO_VERBOSE_FLAG -- -D warnings"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
        yellow "💡 Fix clippy warnings before proceeding"
    fi
fi

# Unit tests
if [ "$RUN_UNIT_TESTS" = true ]; then
    echo "🧪 Running unit tests..."

    # Test src/ modules
    if run_test "library unit tests" "cargo test --lib $CARGO_VERBOSE_FLAG"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi

    # Test specific unit test modules
    if run_test "URL extraction tests" "cargo test tests::unit::url_tests $CARGO_VERBOSE_FLAG"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi
fi

# Integration tests
if [ "$RUN_INTEGRATION_TESTS" = true ]; then
    echo "🔗 Running integration tests..."

    # Note: Integration tests might be disabled due to mockito version issues
    # Check if integration tests are enabled
    if [ -f "tests/tests.rs" ] && grep -q "mod integration;" tests/tests.rs 2>/dev/null; then
        if run_test "integration tests" "cargo test tests::integration $CARGO_VERBOSE_FLAG"; then
            ((TESTS_PASSED++))
        else
            ((TESTS_FAILED++))
        fi
    else
        yellow "⚠️  Integration tests are disabled (mockito compatibility)"
    fi

    # Run end-to-end tests directly
    if run_test "end-to-end tests" "cargo test e2e_tests $CARGO_VERBOSE_FLAG"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi
fi

# Benchmark tests
if [ "$RUN_BENCHMARK_TESTS" = true ]; then
    echo "⚡ Running benchmark tests..."

    if run_test "benchmark tests" "cargo test benchmark_tests $CARGO_VERBOSE_FLAG"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi
fi

# Documentation tests
echo "📚 Running documentation tests..."
if run_test "doc tests" "cargo test --doc $CARGO_VERBOSE_FLAG"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Test build
echo "🏗️  Testing build..."
if run_test "build test" "cargo build --all-features $CARGO_VERBOSE_FLAG"; then
    ((TESTS_PASSED++))
else
    ((TESTS_FAILED++))
fi

# Summary
echo ""
echo "📊 Test Summary:"
echo "=================="
green "✅ Passed: $TESTS_PASSED"
if [ $TESTS_FAILED -gt 0 ]; then
    red "❌ Failed: $TESTS_FAILED"
else
    green "❌ Failed: $TESTS_FAILED"
fi

echo ""
if [ $TESTS_FAILED -eq 0 ]; then
    green "🎉 All tests passed! The codebase is healthy."
    exit 0
else
    red "💥 Some tests failed. Please fix the issues before proceeding."
    exit 1
fi
