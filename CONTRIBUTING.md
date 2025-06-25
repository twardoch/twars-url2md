# Contributing to twars-url2md

Thank you for your interest in contributing to twars-url2md! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Contributions](#making-contributions)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Accept feedback gracefully
- Prioritize the community's best interests

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/twars-url2md.git
   cd twars-url2md
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/twardoch/twars-url2md.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Cargo
- Git

### Initial Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md

# Build the project
cargo build

# Run tests
cargo test

# Run the tool
cargo run -- --help
```

### Recommended Tools

- **rustfmt**: Code formatting
  ```bash
  rustup component add rustfmt
  ```
- **clippy**: Rust linter
  ```bash
  rustup component add clippy
  ```
- **cargo-edit**: Managing dependencies
  ```bash
  cargo install cargo-edit
  ```

## Making Contributions

### Types of Contributions

We welcome various types of contributions:

1. **Bug Fixes**: Fix issues reported in GitHub Issues
2. **Features**: Add new functionality (discuss major features first)
3. **Documentation**: Improve docs, add examples, fix typos
4. **Tests**: Increase test coverage
5. **Performance**: Optimize code for better performance
6. **Refactoring**: Improve code structure and maintainability

### Contribution Workflow

1. **Create an issue** (if one doesn't exist) describing what you plan to work on
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following our coding standards
4. **Write/update tests** for your changes
5. **Update documentation** if needed
6. **Commit your changes** with clear commit messages
7. **Push to your fork** and submit a pull request

### Commit Message Guidelines

Follow conventional commit format:

```
type(scope): subject

body

footer
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build process or auxiliary tool changes

Example:
```
feat(url): add support for custom URL patterns

- Add regex-based URL extraction
- Support configurable patterns
- Update documentation

Closes #123
```

## Coding Standards

### Rust Style Guide

1. **Format code** with rustfmt:
   ```bash
   cargo fmt
   ```

2. **Check with clippy**:
   ```bash
   cargo clippy --all-targets --all-features
   ```

3. **Naming conventions**:
   - Use `snake_case` for functions and variables
   - Use `CamelCase` for types and traits
   - Use `SCREAMING_SNAKE_CASE` for constants

4. **Error handling**:
   - Use `Result<T, E>` for fallible operations
   - Provide context with `anyhow::Context`
   - Avoid `unwrap()` except in tests

5. **Documentation**:
   - Document all public APIs
   - Include examples in doc comments
   - Use `///` for public items, `//` for internal comments

### Code Organization

```
src/
├── lib.rs          # Library root with public API
├── main.rs         # CLI binary entry point
├── cli.rs          # CLI argument parsing
├── url.rs          # URL extraction and validation
├── html.rs         # HTML fetching and processing
└── markdown.rs     # Markdown conversion

tests/
├── unit/           # Unit tests
├── integration/    # Integration tests
└── fixtures/       # Test data
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Writing Tests

1. **Unit tests**: Place in `#[cfg(test)]` modules in source files
2. **Integration tests**: Place in `tests/` directory
3. **Test coverage**: Aim for >80% coverage
4. **Test naming**: Use descriptive names like `test_extract_urls_from_html_with_base_url`

Example test:
```rust
#[test]
fn test_url_extraction() {
    let text = "Visit https://example.com";
    let urls = extract_urls_from_text(text, None);
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0], "https://example.com");
}
```

## Documentation

### Code Documentation

1. **Module docs**: Add module-level documentation
   ```rust
   //! This module handles URL extraction and validation.
   //!
   //! It provides functions to extract URLs from various text formats
   //! and validate them according to configurable rules.
   ```

2. **Function docs**: Document all public functions
   ```rust
   /// Extract URLs from text content.
   ///
   /// # Arguments
   ///
   /// * `text` - The text to extract URLs from
   /// * `base_url` - Optional base URL for resolving relative URLs
   ///
   /// # Examples
   ///
   /// ```
   /// let urls = extract_urls_from_text("Visit https://example.com", None);
   /// assert_eq!(urls.len(), 1);
   /// ```
   pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String> {
   ```

3. **Generate docs**:
   ```bash
   cargo doc --no-deps --open
   ```

### User Documentation

- Update README.md for user-facing changes
- Add examples for new features
- Update CLI help text in clap attributes

## Submitting Changes

### Pull Request Process

1. **Update your branch** with latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run quality checks**:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test
   cargo doc --no-deps
   ```

3. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create Pull Request** on GitHub with:
   - Clear title describing the change
   - Description of what changed and why
   - Reference to related issues
   - Screenshots/examples if applicable

### Review Process

1. Maintainers will review your PR
2. Address any feedback
3. Once approved, maintainers will merge your PR

## Release Process

Releases are managed by maintainers:

1. **Version bump** in Cargo.toml
2. **Update CHANGELOG.md**
3. **Create git tag**:
   ```bash
   git tag -a v1.2.3 -m "Release v1.2.3"
   git push origin v1.2.3
   ```
4. **GitHub Actions** automatically:
   - Runs tests
   - Creates GitHub release
   - Builds binaries
   - Publishes to crates.io

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open a GitHub Issue
- **Security**: Email security concerns privately

## Recognition

Contributors are recognized in:
- GitHub contributors page
- Release notes
- Project documentation

Thank you for contributing to twars-url2md!