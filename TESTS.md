# Comprehensive Test Plan for twars-url2md

## Overview

This document outlines the comprehensive test strategy for the `twars-url2md` project, a Rust CLI application that downloads URLs and converts them to clean, readable Markdown files. The test suite aims to ensure reliability, correctness, and robustness across all components.

## Test Structure

### 1. Unit Tests

#### 1.1 URL Module (`src/url.rs`)
- **URL Extraction**
  - Extract URLs from plain text
  - Extract URLs from Markdown links
  - Extract URLs from HTML href attributes
  - Handle mixed format inputs
  - Handle URLs with special characters and encodings
  - Test empty and malformed inputs

- **URL Validation**
  - Validate HTTP/HTTPS URLs
  - Reject invalid protocols
  - Handle relative URLs with base URL resolution
  - Test URL normalization (trailing slashes, fragments)
  - Handle internationalized domain names (IDN)

- **URL Processing**
  - Deduplicate URLs correctly
  - Preserve URL ordering when required
  - Handle query parameters and fragments appropriately

#### 1.2 HTML Module (`src/html.rs`)
- **HTML Fetching**
  - Mock HTTP requests for different status codes (200, 404, 500, etc.)
  - Test timeout handling
  - Test redirect following (301, 302)
  - Handle large HTML documents
  - Test content-type validation

- **Monolith Integration**
  - Test HTML cleaning with various configurations
  - Handle JavaScript-heavy pages
  - Test CSS inlining behavior
  - Verify image handling (base64 encoding vs. external)
  - Test iframe and embedded content handling

- **Error Recovery**
  - Test panic recovery from Monolith crashes
  - Verify fallback mechanisms for non-HTML content
  - Test retry logic for transient failures

#### 1.3 Markdown Module (`src/markdown.rs`)
- **HTML to Markdown Conversion**
  - Test basic HTML elements (p, div, span)
  - Test headings (h1-h6) conversion
  - Test list conversion (ul, ol, nested lists)
  - Test link preservation
  - Test image alt text and URLs
  - Test code blocks and inline code
  - Test table conversion
  - Test blockquote handling
  - Handle malformed HTML gracefully

- **Content Cleaning**
  - Remove script and style tags
  - Preserve semantic structure
  - Handle special characters and entities
  - Test Unicode handling

#### 1.4 CLI Module (`src/cli.rs`)
- **Argument Parsing**
  - Test all CLI flags and options
  - Validate mutually exclusive options
  - Test default values
  - Verify help text accuracy
  - Test argument validation and error messages

- **Input Handling**
  - Read from stdin
  - Read from files
  - Handle multiple input sources
  - Test glob pattern expansion
  - Handle non-existent files gracefully

#### 1.5 Library Module (`src/lib.rs`)
- **Orchestration Logic**
  - Test sequential vs. concurrent processing modes
  - Verify CPU core detection and adaptation
  - Test progress reporting
  - Verify output organization logic

- **Path Generation**
  - Test URL-to-filesystem path conversion
  - Handle special characters in paths
  - Test collision handling
  - Verify directory creation
  - Test both flat and hierarchical output modes

### 2. Integration Tests

#### 2.1 End-to-End Workflows
- **Single URL Processing**
  - Download and convert a simple HTML page
  - Process a complex web page with images and styles
  - Handle a non-HTML resource (PDF, image)
  - Test local file processing

- **Batch Processing**
  - Process multiple URLs from a file
  - Test concurrent processing limits
  - Verify output file organization
  - Test packed mode output

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
- Confirm panic recovery in real scenarios

### 3. Performance Tests

#### 3.1 Concurrency Testing
- **Load Testing**
  - Process 100+ URLs concurrently
  - Measure memory usage under load
  - Test connection pool limits
  - Verify rate limiting behavior

- **Resource Management**
  - Test file descriptor limits
  - Monitor thread pool usage
  - Verify cleanup of temporary resources

#### 3.2 Benchmarks
- Benchmark URL extraction performance
- Benchmark HTML to Markdown conversion
- Measure overhead of concurrent processing
- Profile memory allocation patterns

### 4. Property-Based Tests

Using `proptest` or similar:
- Generate random URL patterns
- Test with arbitrary HTML structures
- Fuzz input parsing
- Verify invariants hold under random inputs

### 5. Regression Tests

- Specific test cases for reported bugs
- Edge cases discovered in production
- Platform-specific issues (Windows paths, etc.)

## Test Data and Fixtures

### Mock Data
- Sample HTML files of varying complexity
- Mock HTTP responses for different scenarios
- Test URLs covering various patterns
- Malformed inputs for error testing

### Test Servers
- Mock HTTP server for integration tests
- Configurable response delays
- Error injection capabilities
- Redirect chain testing

## Testing Infrastructure

### Continuous Integration
- Run full test suite on PR
- Platform matrix (Linux, macOS, Windows)
- Rust version compatibility testing
- Dependency audit

### Code Coverage
- Target: 80% line coverage minimum
- 100% coverage for critical paths
- Coverage reports in CI

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

# Run unit tests only
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
   - Basic HTML to Markdown conversion
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
   - Regression test suite
   - Documentation tests

## Success Criteria

- All tests pass in CI
- No flaky tests
- Test execution < 60 seconds
- Clear test names and documentation
- Easy to add new test cases
- Comprehensive error scenario coverage