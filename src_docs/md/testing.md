# Testing

Testing strategy and guidelines for `twars-url2md`. Covers unit tests, integration tests, benchmarks, and quality assurance.

## Test Architecture

### Test Organization

```
tests/
├── unit/                    # Component-specific tests
│   ├── mod.rs              # Test module organization
│   └── url_tests.rs        # URL processing tests
├── integration/            # End-to-end workflow tests
│   ├── e2e_tests.rs       # Complete processing workflows
│   └── mod.rs             # Integration test utilities
├── fixtures/              # Test data and expected outputs
│   ├── html/              # Sample HTML files
│   │   ├── simple.html    # Basic HTML structure
│   │   └── complex.html   # Complex HTML with edge cases
│   ├── expected/          # Expected conversion outputs
│   │   └── simple_output.md
│   └── urls/              # URL test lists
│       ├── test_urls.txt  # Basic URL list
│       └── mixed_content.txt # URLs with mixed content
├── benchmarks/            # Performance benchmarks
│   └── benchmarks.rs      # Performance test suite
├── common/                # Shared test utilities
│   └── mod.rs            # Common test functions and setup
└── tests.rs              # Main test runner
```

### Test Categories

1. **Unit Tests** - Individual component functionality
2. **Integration Tests** - End-to-end workflows
3. **Performance Tests** - Benchmarks and profiling
4. **Compatibility Tests** - Edge cases and different inputs
5. **Security Tests** - Input validation and safety

## Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test --all-features

# Run specific test module
cargo test url_tests --all-features

# Run integration tests only
cargo test --test integration --all-features

# Run with output (for debugging)
cargo test --all-features -- --nocapture

# Run single test function
cargo test test_url_extraction --all-features
```

### Advanced Test Execution

```bash
# Run tests with verbose output
cargo test --all-features --verbose

# Run tests in single thread (for debugging)
cargo test --all-features -- --test-threads=1

# Run ignored tests (marked with #[ignore])
cargo test --all-features -- --ignored

# Show test timing
cargo test --all-features -- --report-time

# Filter tests by name pattern  
cargo test url --all-features  # Runs tests with "url" in name
```

### Test Configuration

```bash
# Set environment variables for tests
RUST_LOG=debug cargo test --all-features

# Test with specific configuration
TEST_TIMEOUT=30 cargo test integration --all-features

# Test with network access disabled
OFFLINE_MODE=1 cargo test unit --all-features
```

## Unit Tests

### URL Processing Tests

```rust
#[cfg(test)]
mod url_tests {
    use super::*;
    
    #[test]
    fn test_url_extraction_from_plain_text() {
        let text = "Check out https://example.com and https://rust-lang.org";
        let urls = extract_urls_from_text(text, None);
        
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"https://rust-lang.org".to_string()));
    }
    
    #[test]
    fn test_url_extraction_from_html() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com">Example</a>
                    <a href="/relative/path">Relative</a>
                </body>
            </html>
        "#;
        
        let urls = extract_urls_from_text(html, Some("https://base.com"));
        
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"https://base.com/relative/path".to_string()));
    }
    
    #[test]
    fn test_url_validation() {
        let invalid_urls = vec![
            "not-a-url",
            "ftp://invalid-scheme.com",
            "mailto:test@example.com",
            "",
        ];
        
        for url in invalid_urls {
            let result = validate_url(url);
            assert!(result.is_err(), "URL should be invalid: {}", url);
        }
    }
    
    #[test]
    fn test_path_generation() {
        let url = "https://doc.rust-lang.org/book/ch01-01-installation.html";
        let base_path = PathBuf::from("output");
        
        let result = url_to_file_path(url, &base_path);
        let expected = PathBuf::from("output/doc.rust-lang.org/book/ch01-01-installation.md");
        
        assert_eq!(result, expected);
    }
}
```

### HTML Processing Tests

```rust
#[cfg(test)]
mod html_tests {
    use super::*;
    use mockito::Server;
    
    #[tokio::test]
    async fn test_html_fetching() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>Test content</body></html>")
            .create_async()
            .await;
        
        let url = format!("{}/test", server.url());
        let result = fetch_html_content(&url).await;
        
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("Test content"));
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_retry_logic() {
        let mut server = Server::new_async().await;
        
        // First request fails, second succeeds
        let mock_fail = server
            .mock("GET", "/retry-test")
            .with_status(500)
            .create_async()
            .await;
        
        let mock_success = server
            .mock("GET", "/retry-test")
            .with_status(200)
            .with_body("Success")
            .create_async()
            .await;
        
        let url = format!("{}/retry-test", server.url());
        let result = fetch_with_retry(&url, 3).await;
        
        assert!(result.is_ok());
        mock_fail.assert_async().await;
        mock_success.assert_async().await;
    }
}
```

### Markdown Conversion Tests

```rust
#[cfg(test)]
mod markdown_tests {
    use super::*;
    
    #[test]
    fn test_html_to_markdown_conversion() {
        let html = r#"
            <html>
                <head><title>Test Page</title></head>
                <body>
                    <h1>Main Heading</h1>
                    <p>This is a <strong>bold</strong> paragraph with a 
                       <a href="https://example.com">link</a>.</p>
                    <ul>
                        <li>First item</li>
                        <li>Second item</li>
                    </ul>
                </body>
            </html>
        "#;
        
        let markdown = html_to_markdown(html).unwrap();
        
        assert!(markdown.contains("# Main Heading"));
        assert!(markdown.contains("**bold**"));
        assert!(markdown.contains("[link](https://example.com)"));
        assert!(markdown.contains("- First item"));
        assert!(markdown.contains("- Second item"));
    }
    
    #[test]
    fn test_complex_html_structures() {
        let html = include_str!("../fixtures/html/complex.html");
        let result = html_to_markdown(html);
        
        assert!(result.is_ok());
        let markdown = result.unwrap();
        
        // Verify specific structures are preserved
        assert!(markdown.contains("# "));  // Headers preserved
        assert!(markdown.contains("| "));  // Tables preserved
        assert!(markdown.contains("```")); // Code blocks preserved
    }
}
```

## Integration Tests

### End-to-End Workflow Tests

```rust
// tests/integration/e2e_tests.rs
use twars_url2md::{process_urls, Config};
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_single_url_processing() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        output_base: temp_dir.path().to_path_buf(),
        verbose: false,
        max_retries: 2,
        single_file: false,
        has_output: true,
        pack_file: None,
    };
    
    let urls = vec!["https://httpbin.org/html".to_string()];
    let result = process_urls(urls, config).await;
    
    assert!(result.is_ok());
    
    // Verify output file was created
    let output_files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    
    assert!(!output_files.is_empty());
    assert!(output_files.iter().any(|p| p.extension().unwrap() == "md"));
}

#[tokio::test]
async fn test_packed_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let pack_file = temp_dir.path().join("packed.md");
    
    let config = Config {
        output_base: temp_dir.path().to_path_buf(),
        verbose: false,
        max_retries: 2,
        single_file: false,
        has_output: false,
        pack_file: Some(pack_file.clone()),
    };
    
    let urls = vec![
        "https://httpbin.org/html".to_string(),
        "https://httpbin.org/json".to_string(),
    ];
    
    let result = process_urls(urls, config).await;
    assert!(result.is_ok());
    
    // Verify packed file was created with proper format
    let content = fs::read_to_string(&pack_file).unwrap();
    assert!(content.contains("# https://httpbin.org/html"));
    assert!(content.contains("# https://httpbin.org/json"));
}

#[tokio::test]
async fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        output_base: temp_dir.path().to_path_buf(),
        verbose: false,
        max_retries: 1,
        single_file: false,
        has_output: true,
        pack_file: None,
    };
    
    let urls = vec![
        "https://httpbin.org/html".to_string(),      // Valid URL
        "https://invalid-domain-12345.com".to_string(), // Invalid URL
        "https://httpbin.org/status/404".to_string(),   // 404 error
    ];
    
    let result = process_urls(urls, config).await;
    
    // Should complete but with some errors
    assert!(result.is_ok());
    let errors = result.unwrap();
    assert!(!errors.is_empty()); // Should have errors for invalid URLs
}
```

### File Input/Output Tests

```rust
#[tokio::test]
async fn test_file_input_processing() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create input file with URLs
    let input_file = temp_dir.path().join("urls.txt");
    fs::write(&input_file, "https://httpbin.org/html\nhttps://httpbin.org/json").unwrap();
    
    // Process using file input
    let output = Command::new("cargo")
        .args(&["run", "--", "-i"])
        .arg(&input_file)
        .arg("-o")
        .arg(temp_dir.path().join("output"))
        .output()
        .await
        .unwrap();
    
    assert!(output.status.success());
    
    // Verify output files
    let output_dir = temp_dir.path().join("output");
    let entries: Vec<_> = fs::read_dir(output_dir).unwrap().collect();
    assert!(!entries.is_empty());
}

#[tokio::test]
async fn test_stdin_input() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--stdin", "-o"])
        .arg(temp_dir.path().join("output"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    
    // Send URLs via stdin
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"https://httpbin.org/html\n").await.unwrap();
    stdin.flush().await.unwrap();
    drop(child.stdin.take());
    
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
}
```

## Performance Testing

### Benchmark Tests

```rust
// tests/benchmarks/benchmarks.rs
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use twars_url2md::*;

fn benchmark_url_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_extraction");
    
    let test_cases = vec![
        ("small", include_str!("../fixtures/html/simple.html")),
        ("medium", include_str!("../fixtures/html/medium.html")),
        ("large", include_str!("../fixtures/html/complex.html")),
    ];
    
    for (name, content) in test_cases {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("extract_urls", name),
            content,
            |b, content| {
                b.iter(|| extract_urls_from_text(content, None))
            },
        );
    }
    
    group.finish();
}

fn benchmark_markdown_conversion(c: &mut Criterion) {
    let html_content = include_str!("../fixtures/html/complex.html");
    
    c.bench_function("html_to_markdown", |b| {
        b.iter(|| html_to_markdown(html_content))
    });
}

fn benchmark_concurrent_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_processing");
    
    for url_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("process_urls", url_count),
            url_count,
            |b, &url_count| {
                let urls: Vec<String> = (0..url_count)
                    .map(|i| format!("https://httpbin.org/html?id={}", i))
                    .collect();
                
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async {
                        // Benchmark implementation
                    });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_url_extraction,
    benchmark_markdown_conversion,
    benchmark_concurrent_processing
);
criterion_main!(benches);
```

### Memory Usage Tests

```rust
#[cfg(test)]
mod memory_tests {
    use super::*;
    
    #[test]
    fn test_memory_usage_large_files() {
        let large_html = "a".repeat(10_000_000); // 10MB string
        
        // Monitor memory usage during processing
        let initial_memory = get_memory_usage();
        let result = html_to_markdown(&large_html);
        let peak_memory = get_memory_usage();
        
        assert!(result.is_ok());
        
        // Memory usage should be reasonable (not loading everything at once)
        let memory_increase = peak_memory - initial_memory;
        assert!(memory_increase < 50_000_000); // Less than 50MB increase
    }
    
    #[tokio::test]
    async fn test_concurrent_memory_usage() {
        let urls: Vec<String> = (0..100)
            .map(|i| format!("https://httpbin.org/html?id={}", i))
            .collect();
        
        let initial_memory = get_memory_usage();
        
        // Process many URLs concurrently
        let _results = process_urls_concurrent(&urls).await;
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        // Memory usage should scale reasonably with concurrent processing
        assert!(memory_increase < 100_000_000); // Less than 100MB
    }
    
    fn get_memory_usage() -> usize {
        // Platform-specific memory usage measurement
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let status = fs::read_to_string("/proc/self/status").unwrap();
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    return parts[1].parse::<usize>().unwrap() * 1024; // Convert KB to bytes
                }
            }
        }
        0
    }
}
```

## Quality Assurance

### Test Data Management

```rust
// tests/common/mod.rs
use std::path::PathBuf;

pub struct TestFixtures;

impl TestFixtures {
    pub fn html_simple() -> &'static str {
        include_str!("../fixtures/html/simple.html")
    }
    
    pub fn html_complex() -> &'static str {
        include_str!("../fixtures/html/complex.html")
    }
    
    pub fn expected_simple_md() -> &'static str {
        include_str!("../fixtures/expected/simple_output.md")
    }
    
    pub fn test_urls() -> Vec<String> {
        include_str!("../fixtures/urls/test_urls.txt")
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect()
    }
}

pub fn setup_test_environment() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

pub async fn create_mock_server() -> mockito::Server {
    mockito::Server::new_async().await
}
```

### Automated Test Execution

```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    name: Tests
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy
      
      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --check
        
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run unit tests
        run: cargo test --lib --all-features
      
      - name: Run integration tests
        run: cargo test --test '*' --all-features
      
      - name: Run doc tests
        run: cargo test --doc --all-features

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Generate coverage report
        run: cargo tarpaulin --out Xml --all-features
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Local Test Scripts

```bash
#!/bin/bash
# scripts/test.sh - Comprehensive local testing

set -euo pipefail

echo "Running comprehensive test suite..."

# Code formatting
echo "Checking code formatting..."
cargo fmt --check

# Linting
echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Unit tests
echo "Running unit tests..."
cargo test --lib --all-features --verbose

# Integration tests
echo "Running integration tests..."
cargo test --test '*' --all-features --verbose

# Doc tests
echo "Running documentation tests..."
cargo test --doc --all-features

# Security audit
echo "Running security audit..."
cargo audit

# Benchmarks (optional)
if [[ "${RUN_BENCHMARKS:-false}" == "true" ]]; then
    echo "Running benchmarks..."
    cargo bench
fi

echo "All tests passed!"
```

### Continuous Monitoring

```bash
# scripts/watch-tests.sh - Development test monitoring
#!/bin/bash

# Install cargo-watch if not present
if ! command -v cargo-watch &> /dev/null; then
    cargo install cargo-watch
fi

# Watch for changes and run tests
cargo watch -x "test --all-features" -x "clippy --all-targets --all-features"
```

---

**Testing Best Practices**
- Write tests first for new features (TDD approach)
- Test edge cases and error conditions
- Use realistic test data from fixtures
- Keep tests fast and independent
- Mock external dependencies appropriately
- Measure and maintain good test coverage

**Performance Testing**
- Run benchmarks regularly to catch performance regressions
- Test with realistic data sizes and network conditions
- Monitor memory usage during development
- Profile critical paths for optimization opportunities