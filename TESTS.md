# Test Plan for twars-url2md

## Overview

This document describes the test strategy for `twars-url2md`, a Rust CLI tool that downloads URLs and converts them to clean Markdown files. The goal is to ensure correctness, reliability, and performance across all components.

## Test Structure

### 1. Unit Tests

#### 1.1 URL Module (`src/url.rs`)
- **URL Extraction**
  - Extract URLs from plain text
  - Extract URLs from Markdown links
  - Extract URLs from HTML href attributes
  - Handle mixed format inputs
  - Process URLs with special characters and encodings
  - Handle empty and malformed inputs

- **URL Validation**
  - Validate HTTP/HTTPS URLs
  - Reject invalid protocols
  - Resolve relative URLs with base URL
  - Normalize URLs (trailing slashes, fragments)
  - Handle internationalized domain names (IDN)

- **URL Processing**
  - Deduplicate URLs
  - Preserve URL ordering
  - Handle query parameters and fragments

#### 1.2 HTML Module (`src/html.rs`)
- **HTML Fetching**
  - Mock HTTP requests for status codes (200, 404, 500)
  - Handle timeouts
  - Follow redirects (301, 302)
  - Process large HTML documents
  - Validate content types

- **Monolith Integration**
  - Clean HTML with various configurations
  - Process JavaScript-heavy pages
  - Inline CSS
  - Handle images (base64 vs. external)
  - Process iframe and embedded content

- **Error Recovery**
  - Recover from Monolith crashes
  - Fallback for non-HTML content
  - Retry transient failures

#### 1.3 Markdown Module (`src/markdown.rs`)
- **HTML to Markdown Conversion**
  - Convert basic HTML elements (p, div, span)
  - Convert headings (h1-h6)
  - Convert lists (ul, ol, nested)
  - Preserve links
  - Handle image alt text and URLs
  - Convert code blocks and inline code
  - Convert tables
  - Handle blockquotes
  - Process malformed HTML

- **Content Cleaning**
  - Remove script and style tags
  - Preserve semantic structure
  - Handle special characters and entities
  - Process Unicode

#### 1.4 CLI Module (`src/cli.rs`)
- **Argument Parsing**
  - Test all CLI flags and options
  - Validate mutually exclusive options
  - Test default values
  - Verify help text
  - Test argument validation and error messages

- **Input Handling**
  - Read from stdin
  - Read from files
  - Handle multiple input sources
  - Expand glob patterns
  - Handle non-existent files

#### 1.5 Library Module (`src/lib.rs`)
- **Orchestration Logic**
  - Test sequential vs. concurrent modes
  - Verify CPU core detection
  - Test progress reporting
  - Verify output organization

- **Path Generation**
  - Convert URLs to filesystem paths
  - Handle special characters in paths
  - Handle path collisions
  - Create directories
  - Test flat and hierarchical output modes

### 2. Integration Tests

#### 2.1 End-to-End Workflows
- **Single URL Processing**
  - Download and convert simple HTML
  - Process complex pages with images/styles
  - Handle non-HTML resources (PDF, images)
  - Process local files

- **Batch Processing**
  - Process multiple URLs from file
  - Test concurrent processing limits
  - Verify output file organization
  - Test packed mode

- **Error Scenarios**
  - Network timeouts
  - Invalid URLs
  - Server errors
  - Disk write failures
  - Insufficient permissions

#### 2.2 Monolith Integration
- Test full Monolith pipeline with real HTML
- Verify CSS and JavaScript handling
- Test resource embedding options
- Confirm panic recovery

### 3. Performance Tests

#### 3.1 Concurrency Testing
- **Load Testing**
  - Process 100+ URLs concurrently
  - Measure memory usage
  - Test connection pool limits
  - Verify rate limiting

- **Resource Management**
  - Test file descriptor limits
  - Monitor thread pool usage
  - Verify temporary resource cleanup

#### 3.2 Benchmarks
- Benchmark URL extraction
- Benchmark HTML to Markdown conversion
- Measure concurrent processing overhead
- Profile memory allocation

### 4. Property-Based Tests

Using `proptest`:
- Generate random URL patterns
- Test arbitrary HTML structures
- Fuzz input parsing
- Verify invariants under random inputs

### 5. Regression Tests

- Test cases for reported bugs
- Production edge cases
- Platform-specific issues (Windows paths)

## Test Data and Fixtures

### Mock Data
- HTML samples of varying complexity
- HTTP responses for different scenarios
- Test URLs covering various patterns
- Malformed inputs for error cases

### Test Servers
- Mock HTTP server for integration tests
- Configurable response delays
- Error injection
- Redirect chain testing

## Testing Infrastructure

### Continuous Integration
- Run full test suite on PR
- Test on Linux, macOS, Windows
- Verify Rust version compatibility
- Run dependency audits

### Code Coverage
- Minimum 80% line coverage
- 100% coverage for critical paths
- Generate coverage reports in CI

### Test Organization
```
tests/
├── unit/
│   ├── url_tests.rs
│   ├── html_tests.rs
│   ├── markdown_tests.rs
│   └── cli_tests.rs
├── integration/
│   ├── e2e_tests.rs
│   ├── monolith_tests.rs
│   └── concurrent_tests.rs
├── fixtures/
│   ├── html/
│   ├── urls/
│   └── expected/
└── common/
    ├── mod.rs
    └── helpers.rs
```

## Testing Commands

```bash
# Run all tests
cargo test --all-features

# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench

# Run specific test module
cargo test url::tests

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Test Implementation Priority

1. **Phase 1: Core Functionality**
   - URL extraction and validation
   - HTML to Markdown conversion
   - CLI argument parsing

2. **Phase 2: Integration**
   - End-to-end workflows
   - Monolith integration
   - Error handling

3. **Phase 3: Robustness**
   - Concurrent processing
   - Property-based tests
   - Performance benchmarks

4. **Phase 4: Polish**
   - Platform-specific tests
   - Regression suite
   - Documentation tests

## Success Criteria

- All CI tests pass
- No flaky tests
- Test execution < 60 seconds
- Clear test names and documentation
- Easy to add new test cases
- Complete error scenario coverage