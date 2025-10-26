# Contributing

We welcome contributions to `twars-url2md`. This guide explains how to set up your environment, understand the codebase, and submit changes.

## Getting Started

### Development Environment Setup

1. **Prerequisites**
   ```bash
   # Install Rust (1.70.0 or later)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Clone and Build**
   ```bash
   git clone https://github.com/twardoch/twars-url2md.git
   cd twars-url2md
   
   # Build in debug mode
   cargo build
   
   # Run tests
   cargo test --all-features
   
   # Check formatting and linting
   cargo fmt --check
   cargo clippy --all-targets --all-features
   ```

3. **Development Tools**
   ```bash
   # Install additional tools
   rustup component add rustfmt clippy
   cargo install cargo-audit cargo-watch
   
   # For continuous testing during development
   cargo watch -x test
   ```

### Project Structure

```
src/
├── main.rs          # Entry point with panic handling
├── lib.rs           # Core library and orchestration
├── cli.rs           # Command-line interface
├── url.rs           # URL extraction and validation
├── html.rs          # HTTP client and HTML fetching
├── markdown.rs      # HTML to Markdown conversion
└── tests.rs         # Unit tests

tests/
├── integration/     # Integration tests
├── fixtures/        # Test data and expected outputs
└── benchmarks/      # Performance benchmarks

.github/
└── workflows/       # CI/CD configuration

scripts/             # Build and release scripts
docs/                # Generated documentation
```

## Development Workflow

### Making Changes

1. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-description
   ```

2. **Make Your Changes**
   - Follow coding standards below
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   # Run all tests
   cargo test --all-features
   
   # Run specific test
   cargo test test_url_extraction
   
   # Run integration tests
   cargo test --test integration
   
   # Check formatting
   cargo fmt --check
   
   # Run linter
   cargo clippy --all-targets --all-features
   ```

4. **Commit and Push**
   ```bash
   git add .
   git commit -m "feat: add new URL extraction feature"
   git push origin feature/your-feature-name
   ```

5. **Create Pull Request**
   - Use GitHub web interface
   - Fill out PR template
   - Link related issues

### Commit Message Convention

Use conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes  
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: add support for custom User-Agent headers
fix: handle malformed URLs gracefully
docs: update installation instructions
test: add integration tests for packed output
```

## Coding Standards

### Rust Code Style

Follow standard Rust conventions:

1. **Formatting**
   ```bash
   cargo fmt
   ```

2. **Naming Conventions**
   ```rust
   // Functions and variables: snake_case
   fn process_urls() {}
   let url_count = 0;
   
   // Types and traits: PascalCase
   struct UrlProcessor {}
   trait ContentConverter {}
   
   // Constants: SCREAMING_SNAKE_CASE
   const MAX_RETRIES: u32 = 3;
   ```

3. **Error Handling**
   ```rust
   use anyhow::{Result, Context};
   
   fn process_url(url: &str) -> Result<String> {
       fetch_content(url)
           .with_context(|| format!("Failed to fetch URL: {}", url))?;
       // ...
   }
   ```

4. **Documentation**
   ```rust
   /// Processes a URL and converts it to Markdown.
   /// 
   /// # Arguments
   /// 
   /// * `url` - The URL to process
   /// * `config` - Processing configuration
   /// 
   /// # Returns
   /// 
   /// The converted Markdown content or an error if processing fails.
   /// 
   /// # Example
   /// 
   /// ```
   /// let markdown = process_url("https://example.com", &config)?;
   /// ```
   pub fn process_url(url: &str, config: &Config) -> Result<String> {
       // Implementation
   }
   ```

### Code Organization

1. **Module Structure**
   - Keep modules focused
   - Use `pub(crate)` for internal APIs
   - Document public APIs

2. **Error Types**
   ```rust
   use thiserror::Error;
   
   #[derive(Error, Debug)]
   pub enum ProcessingError {
       #[error("Network error: {0}")]
       Network(#[from] reqwest::Error),
       
       #[error("Invalid URL: {url}")]
       InvalidUrl { url: String },
   }
   ```

3. **Async Code**
   ```rust
   pub async fn fetch_multiple_urls(urls: Vec<String>) -> Result<Vec<String>> {
       let futures = urls.into_iter()
           .map(|url| fetch_single_url(url));
       
       let results = futures::future::try_join_all(futures).await?;
       Ok(results)
   }
   ```

## Testing Guidelines

### Test Categories

1. **Unit Tests** (`src/tests.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_url_extraction() {
           let text = "Visit https://example.com for more info";
           let urls = extract_urls_from_text(text, None);
           assert_eq!(urls.len(), 1);
           assert_eq!(urls[0], "https://example.com");
       }
       
       #[tokio::test]
       async fn test_async_processing() {
           let result = process_url("https://httpbin.org/html").await;
           assert!(result.is_ok());
       }
   }
   ```

2. **Integration Tests** (`tests/integration/`)
   ```rust
   use twars_url2md::{process_urls, Config};
   use tempfile::tempdir;
   
   #[tokio::test]
   async fn test_end_to_end_processing() {
       let temp_dir = tempdir().unwrap();
       let config = Config {
           output_base: temp_dir.path().to_path_buf(),
           verbose: false,
           // ... other config
       };
       
       let urls = vec!["https://httpbin.org/html".to_string()];
       let result = process_urls(urls, config).await;
       
       assert!(result.is_ok());
       // Verify output files exist
   }
   ```

3. **Benchmark Tests** (`tests/benchmarks/`)
   ```rust
   use criterion::{criterion_group, criterion_main, Criterion};
   
   fn benchmark_url_extraction(c: &mut Criterion) {
       let text = include_str!("../fixtures/large_html_file.html");
       
       c.bench_function("extract_urls", |b| {
           b.iter(|| extract_urls_from_text(text, None))
       });
   }
   
   criterion_group!(benches, benchmark_url_extraction);
   criterion_main!(benches);
   ```

### Testing Best Practices

- Test success and failure cases
- Use realistic test data
- Mock external dependencies when appropriate
- Write descriptive test names
- Keep tests fast and independent

## Contributing Areas

### High-Priority Areas

1. **Performance Optimizations**
   - Memory usage improvements
   - Faster HTML processing
   - Better concurrency patterns

2. **Content Processing**
   - Enhanced HTML cleaning rules
   - Better Markdown conversion quality
   - Support for more content types

3. **Error Handling**
   - More specific error messages
   - Better recovery strategies
   - Improved debugging information

4. **Platform Support**
   - Additional architecture support
   - Package manager integrations
   - Container optimizations

### Medium-Priority Areas

1. **Output Formats**
   - Additional export formats (AsciiDoc, reStructuredText)
   - Custom output templates
   - Metadata extraction and embedding

2. **Configuration**
   - Configuration file support
   - More granular control options
   - Environment-specific presets

3. **Monitoring and Observability**
   - Progress reporting improvements
   - Metrics collection
   - Better logging structured data

### Documentation Improvements

1. **User Documentation**
   - More usage examples
   - Troubleshooting guides
   - Video tutorials

2. **Developer Documentation**
   - Architecture deep dives
   - API documentation
   - Integration examples

3. **Community Resources**
   - FAQ compilation
   - Community showcase
   - Best practices guide

## Pull Request Process

### PR Checklist

Before submitting your PR, ensure:

- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Linting passes (`cargo clippy --all-targets --all-features`)
- [ ] Documentation is updated
- [ ] New functionality includes tests
- [ ] Commit messages follow convention
- [ ] PR description explains changes clearly

### PR Template

```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to change)
- [ ] Documentation update

## Testing
Describe how you tested your changes:
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated  
- [ ] Manual testing performed

## Related Issues
Closes #123, Addresses #456

## Additional Notes
Any additional context or considerations.
```

### Review Process

1. **Automated Checks**: CI runs tests and linting
2. **Code Review**: Maintainers review for quality and design
3. **Discussion**: Address feedback and make necessary changes
4. **Approval**: Once approved, PR will be merged

### After Your PR is Merged

- Your contribution will be included in the next release
- You'll be added to the contributors list
- Consider helping with code review for other contributors

## Release Process

### Version Management

We use semantic versioning (SemVer):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Workflow

1. **Prepare Release**
   ```bash
   # Update version in Cargo.toml
   # Update CHANGELOG.md
   # Create release commit
   git commit -m "chore: release v1.2.3"
   ```

2. **Create Tag**
   ```bash
   git tag -a v1.2.3 -m "Release v1.2.3"
   git push origin v1.2.3
   ```

3. **Automated Release**
   - GitHub Actions builds binaries
   - Creates GitHub release
   - Publishes to crates.io

## Community Guidelines

### Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct):
- Be friendly and welcoming
- Be patient and constructive
- Be respectful of different viewpoints
- Focus on what's best for the community

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Pull Requests**: Code contributions and reviews

### Getting Help

- **Documentation**: Check docs for common questions
- **Issues**: Search existing issues first
- **Discussions**: Ask questions in GitHub Discussions
- **Code Review**: Request review from maintainers

---

**First-Time Contributors**: Look for issues labeled `good first issue` or `help wanted`. Don't hesitate to ask questions.

**Recognition**: All contributors are recognized in our README and release notes. Every contribution counts.