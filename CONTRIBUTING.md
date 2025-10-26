# Contributing to twars-url2md

Thanks for your interest in contributing to twars-url2md. This document explains how to contribute effectively.

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

By participating, you agree to:

- Be respectful and inclusive
- Help newcomers get started
- Give constructive criticism
- Accept feedback gracefully
- Act in the community's best interest

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/twars-url2md.git
   cd twars-url2md
   ```
3. Add upstream remote:
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
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md

# Build
cargo build

# Test
cargo test

# Run
cargo run -- --help
```

### Recommended Tools

- **rustfmt**: Code formatting
  ```bash
  rustup component add rustfmt
  ```
- **clippy**: Linting
  ```bash
  rustup component add clippy
  ```
- **cargo-edit**: Dependency management
  ```bash
  cargo install cargo-edit
  ```

## Making Contributions

### Types of Contributions

- Bug fixes
- New features (discuss major ones first)
- Documentation improvements
- Test coverage increases
- Performance optimizations
- Code refactoring

### Contribution Workflow

1. Create an issue (if none exists) describing your plan
2. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make changes following coding standards
4. Write/update tests
5. Update documentation if needed
6. Commit with clear messages
7. Push to your fork and submit a pull request

### Issue Resolution Process

For fixing reported issues:

1. **Analyze**:
   - Read the issue carefully
   - Reproduce locally
   - Document findings

2. **Create verification tests**:
   - Add tests to `issues/issuetest.py`
   - Ensure tests fail before the fix
   - Cover edge cases

3. **Implement fix**:
   - Make minimal changes
   - Follow existing patterns
   - Add debug logging if useful

4. **Verify**:
   ```bash
   python3 issues/issuetest.py
   cargo test --all-features
   ```

5. **Document**:
   - Update CHANGELOG.md
   - Move issue file to `issues/resolved/`
   - Update affected documentation

6. **Example**:
   ```python
   def test_issue_104():
       """Test Issue #104: Adobe CDN timeout fix"""
       result = subprocess.run(
           ['cargo', 'run', '--', '--stdin'],
           input='https://helpx.adobe.com/pdf/illustrator_reference.pdf\n',
           capture_output=True,
           text=True,
           timeout=10
       )
       assert result.returncode == 0
       assert 'illustrator_reference.md' in result.stdout
   ```

### Commit Messages

Use conventional commits:

```
type(scope): subject

body

footer
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Refactoring
- `test`: Tests
- `chore`: Build/tools

Example:
```
feat(url): add support for custom URL patterns

- Add regex-based URL extraction
- Support configurable patterns
- Update documentation

Closes #123
```

## Coding Standards

### Rust Style

1. Format with rustfmt:
   ```bash
   cargo fmt
   ```

2. Lint with clippy:
   ```bash
   cargo clippy --all-targets --all-features
   ```

3. Naming conventions:
   - `snake_case` for functions/variables
   - `CamelCase` for types/traits
   - `SCREAMING_SNAKE_CASE` for constants

4. Error handling:
   - Use `Result<T, E>`
   - Add context with `anyhow::Context`
   - Avoid `unwrap()` outside tests

5. Documentation:
   - Document public APIs
   - Include examples in doc comments
   - Use `///` for public, `//` for internal

### Code Structure

```
src/
├── lib.rs          # Library root
├── main.rs         # CLI entry point
├── cli.rs          # Argument parsing
├── url.rs          # URL extraction/validation
├── html.rs         # HTML fetching/processing
└── markdown.rs     # Markdown conversion

tests/
├── unit/           # Unit tests
├── integration/    # Integration tests
└── fixtures/       # Test data
```

## Testing

### Running Tests

```bash
cargo test                    # All tests
cargo test --all-features      # With all features
cargo test test_name          # Specific test
cargo test -- --nocapture     # With output
cargo bench                   # Benchmarks
python3 issues/issuetest.py   # Issue verification
```

### Writing Tests

- Unit tests: In `#[cfg(test)]` modules
- Integration tests: In `tests/` directory
- Issue tests: In `issues/issuetest.py`
- Aim for >80% coverage
- Use descriptive names: `test_extract_urls_from_html_with_base_url`

Example:
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

### Code Docs

1. Module documentation:
   ```rust
   //! Handles URL extraction and validation.
   //!
   //! Provides functions to extract URLs from text
   //! and validate them according to rules.
   ```

2. Function documentation:
   ```rust
   /// Extract URLs from text.
   ///
   /// # Arguments
   ///
   /// * `text` - Text to extract URLs from
   /// * `base_url` - Base URL for relative URLs
   ///
   /// # Examples
   ///
   /// ```
   /// let urls = extract_urls_from_text("Visit https://example.com", None);
   /// assert_eq!(urls.len(), 1);
   /// ```
   pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String> {
   ```

3. Generate docs:
   ```bash
   cargo doc --no-deps --open
   ```

### User Docs

- Update README.md for user-facing changes
- Add examples for new features
- Update CLI help text in clap attributes

## Submitting Changes

### Pull Request Process

1. Update your branch:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Run quality checks:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test
   cargo doc --no-deps
   ```

3. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

4. Create PR with:
   - Clear title
   - Description of changes
   - Related issue references
   - Examples/screenshots if needed

### Review Process

1. Maintainers review your PR
2. Address feedback
3. Maintainers merge approved PRs

## Release Process

Maintainers handle releases:

1. Bump version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag:
   ```bash
   git tag -a v1.2.3 -m "Release v1.2.3"
   git push origin v1.2.3
   ```
4. GitHub Actions automatically:
   - Runs tests
   - Creates release
   - Builds binaries
   - Publishes to crates.io

## Getting Help

- Questions: GitHub Discussion
- Bugs: GitHub Issue with reproduction steps
- Security: Private email
- Issue status: Check `issues/resolved/`

## Recognition

Contributors appear in:
- GitHub contributors list
- Release notes
- Project documentation

Thanks for contributing.