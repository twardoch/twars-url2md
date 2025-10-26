#!/bin/bash
# this_file: scripts/build.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

: "${CARGO_TERM_COLOR:=always}"
MODE="dev"

green() { printf '\033[32m%s\033[0m\n' "$1"; }
yellow() { printf '\033[33m%s\033[0m\n' "$1"; }
red() { printf '\033[31m%s\033[0m\n' "$1"; }
info() { printf '\033[34m[build]\033[0m %s\n' "$1"; }

die() {
    red "[error] $1"
    exit 1
}

usage() {
    cat <<'USAGE'
Usage: ./scripts/build.sh [MODE]

Modes:
  --quick     Build release binary only (skip fmt/lint/tests)
  --dev       Format, lint, test, and build (default)
  --ci        Clean workspace, lint/test, and build
  --release   Clean, lint/test, build, strip, and copy artifact to builds/

Environment variables:
  TWARS_BUILD_SKIP_CARGO=1   Print steps without executing commands (useful for tests)
USAGE
}

run_step() {
    local label="$1"
    shift
    info "$label"
    if [[ "${TWARS_BUILD_SKIP_CARGO:-0}" == "1" ]]; then
        printf '  (skip) %s\n' "$*"
        return 0
    fi
    "$@"
}

ensure_prereqs() {
    command -v cargo >/dev/null 2>&1 || die "cargo is required"
    command -v rustc >/dev/null 2>&1 || die "rustc is required"
}

collect_git_metadata() {
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        GIT_VERSION="$(git describe --tags --always --dirty 2>/dev/null || echo unknown)"
        GIT_COMMIT="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
        GIT_BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
    else
        GIT_VERSION="local"
        GIT_COMMIT="unknown"
        GIT_BRANCH="unknown"
    fi
}

strip_binary() {
    local binary_path="$1"
    if [[ ! -f "$binary_path" ]]; then
        yellow "Binary not found at $binary_path, skipping strip"
        return 0
    fi
    if command -v strip >/dev/null 2>&1 && [[ "${TWARS_BUILD_SKIP_CARGO:-0}" != "1" ]]; then
        info "Stripping binary"
        strip "$binary_path" || yellow "strip failed, continuing with unstripped binary"
    fi
}

copy_artifact() {
    local binary_path="$1"
    [[ -f "$binary_path" ]] || die "Expected binary at $binary_path"
    mkdir -p builds
    local os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    local arch="$(uname -m)"
    local dest="builds/twars-url2md-${GIT_VERSION}-${os}-${arch}"
    run_step "Copying artifact to $dest" cp "$binary_path" "$dest"
}

run_quick() {
    ensure_prereqs
    run_step "Building release binary" cargo build --release --all-features
}

run_dev() {
    ensure_prereqs
    run_step "Checking formatting" cargo fmt --check
    run_step "Running clippy" cargo clippy --all-targets --all-features -- -D warnings
    run_step "Running tests" cargo test --all-features
    run_step "Running doc tests" cargo test --doc --all-features
    run_step "Building release binary" cargo build --release --all-features
}

run_ci() {
    ensure_prereqs
    run_step "Cleaning workspace" cargo clean
    run_dev
}

run_release() {
    ensure_prereqs
    run_step "Cleaning workspace" cargo clean
    run_dev
    collect_git_metadata
    local binary="target/release/twars-url2md"
    strip_binary "$binary"
    copy_artifact "$binary"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --quick)
            MODE="quick"
            ;;
        --dev)
            MODE="dev"
            ;;
        --ci)
            MODE="ci"
            ;;
        --release)
            MODE="release"
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage
            die "Unknown option: $1"
            ;;
    esac
    shift
done

case "$MODE" in
    quick) run_quick ;;
    dev) run_dev ;;
    ci) run_ci ;;
    release) run_release ;;
    *) die "Unsupported mode: $MODE" ;;
esac

green "Build script completed (${MODE} mode)."
