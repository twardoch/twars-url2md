# Development Guide

This guide explains how to work with the `twars-url2md` codebase. It covers setup, structure, building, testing, and contributing.

## Table of Contents

- [Development Environment Setup](#development-environment-setup)
- [Project Structure](#project-structure)
- [Building and Testing](#building-and-testing)
- [Scripts](#scripts)
- [Release Process](#release-process)
- [CI/CD Pipeline](#cicd-pipeline)
- [Contributing](#contributing)

## Development Environment Setup

### Prerequisites

- **Rust**: Version 1.70.0 or later
- **Git**: For version control
- **curl**: Required by dependencies for HTTP requests

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/twardoch/twars-url2md.git
   cd twars-url2md
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

## Project Structure

```
twars-url2md/
├── src/
│   ├── main.rs         # Entry point and panic handling
│   ├── lib.rs          # Core processing logic
│   ├── cli.rs          # Command-line interface
│   ├── url.rs          # URL extraction and validation
│   ├── html.rs         # HTML fetching and processing
│   ├── markdown.rs     # Markdown conversion
│   └── tests.rs        # Unit tests
├── tests/
│   ├── unit/           # Unit tests
│   ├── integration/    # Integration tests
│   └── benchmarks.rs   # Performance benchmarks
├── scripts/
│   ├── build.sh        # Build script
│   ├── test.sh         # Test script
│   └── release.sh      # Release script
├── .github/
│   └── workflows/
│       └── ci.yml      # GitHub Actions CI/CD
├── build.rs            # Build script for version handling
├── install.sh          # Installation script
├── Cargo.toml          # Dependencies and metadata
├── README.md           # User documentation
├── CHANGELOG.md        # Version history
└── DEVELOPMENT.md      # This file
```

## Building and Testing

### Quick Development Commands

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_extract_urls_from_text

# Run clippy linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Using the Build Scripts

Scripts automate common tasks:

```bash
# Build the project
./scripts/build.sh

# Run comprehensive tests
./scripts/test.sh

# Run only unit tests
./scripts/test.sh --unit-only

# Run with benchmarks
./scripts/test.sh --benchmark

# Prepare a release (dry run)
./scripts/release.sh --version 1.5.0 --dry-run
```

## Scripts

### build.sh

Builds the project and extracts version information from git tags.

**Usage:**
```bash
./scripts/build.sh
```

**Features:**
- Cleans previous builds
- Extracts git version information
- Builds optimized release binary
- Copies binary to `builds/` directory with version info

### test.sh

Runs the complete test suite with various options.

**Usage:**
```bash
./scripts/test.sh [OPTIONS]
```

**Options:**
- `--unit-only`: Run only unit tests
- `--integration-only`: Run only integration tests  
- `--benchmark`: Run benchmark tests
- `--no-clippy`: Skip clippy linting
- `--no-format`: Skip format checking
- `--verbose`: Enable verbose output

### release.sh

Handles the complete release process including git tagging.

**Usage:**
```bash
./scripts/release.sh --version 1.5.0 [OPTIONS]
```

**Options:**
- `--version VERSION`: Version to release (required)
- `--dry-run`: Show what would be done without making changes
- `--skip-tests`: Skip running tests
- `--skip-build`: Skip building the project
- `--force`: Force release even if working directory is dirty

## Release Process

### Semantic Versioning

We follow [Semantic Versioning (semver)](https://semver.org/):

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): New features, backwards compatible
- **Patch** (0.0.X): Bug fixes, backwards compatible

### Version Management

Versions are automatically determined from git tags:

1. **Tagged releases**: Version comes from git tag (e.g., `v1.5.0` → `1.5.0`)
2. **Development builds**: Version includes commit info (e.g., `1.4.2-dev.5.g1234567`)
3. **Dirty builds**: Version includes `-dirty` suffix

### Creating a Release

1. **Update CHANGELOG.md** with new version information
2. **Run tests** to ensure everything works:
   ```bash
   ./scripts/test.sh
   ```
3. **Create release** (dry run first):
   ```bash
   ./scripts/release.sh --version 1.5.0 --dry-run
   ./scripts/release.sh --version 1.5.0
   ```
4. **Push to GitHub** (done automatically by script)
5. **Monitor CI/CD** pipeline at [GitHub Actions](https://github.com/twardoch/twars-url2md/actions)

## CI/CD Pipeline

### GitHub Actions Workflow

Our CI/CD pipeline (`.github/workflows/ci.yml`) includes:

1. **Test Suite**: Runs on Linux, macOS, and Windows with stable and beta Rust
2. **Security Audit**: Automated vulnerability scanning with `cargo-audit`
3. **Code Coverage**: Coverage reporting with `tarpaulin`
4. **Multi-platform Builds**: Creates binaries for:
   - Linux: x86_64, aarch64, musl
   - macOS: x86_64, aarch64
   - Windows: x86_64
5. **Release Automation**: Publishes to GitHub Releases and crates.io

### Pipeline Triggers

- **Pull Requests**: Run tests and security audit
- **Main Branch**: Run full test suite
- **Tags** (v*): Run tests, build releases, and publish

### Secrets Configuration

The following secrets must be configured in GitHub:

- `CRATES_IO_TOKEN`: For publishing to crates.io
- `GITHUB_TOKEN`: Automatically provided by GitHub

## Contributing

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write comprehensive tests for new features
- Update documentation for user-facing changes

### Testing Guidelines

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test complete workflows
- **Benchmark tests**: Measure performance for critical paths
- **Error handling**: Test error conditions and edge cases

### Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes with tests
4. **Run** the test suite: `./scripts/test.sh`
5. **Update** documentation if needed
6. **Submit** a pull request

### Development Best Practices

- Keep commits focused and atomic
- Write descriptive commit messages
- Test edge cases and error conditions
- Consider performance implications
- Document public APIs thoroughly
- Follow security best practices

## Architecture Notes

### Core Components

1. **URL Processing** (`url.rs`): Extract and validate URLs from various formats
2. **HTML Fetching** (`html.rs`): Robust HTTP client with retry logic
3. **Markdown Conversion** (`markdown.rs`): Clean HTML-to-Markdown conversion
4. **CLI Interface** (`cli.rs`): User-friendly command-line interface

### Design Principles

- **Robustness**: Handle network failures and malformed input gracefully
- **Performance**: Leverage async/await and concurrency for speed
- **Usability**: Provide clear error messages and helpful defaults
- **Modularity**: Keep components focused and testable

### Error Handling

- Use `anyhow` for error propagation with context
- Implement retry logic for transient failures
- Provide user-friendly error messages
- Log detailed information for debugging

## Support

For questions or issues:

1. **Check** the [README](README.md) for usage information
2. **Review** existing [GitHub Issues](https://github.com/twardoch/twars-url2md/issues)
3. **Create** a new issue for bugs or feature requests
4. **Join** discussions in the repository

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details