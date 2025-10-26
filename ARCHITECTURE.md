# Architecture Documentation

## Overview

`twars-url2md` is a concurrent web scraping and conversion tool. The architecture emphasizes modularity, error resilience, and scalability.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                          CLI Interface                          │
│                        (src/main.rs)                           │
└───────────────────────────────┬─────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────┐
│                      Core Library (src/lib.rs)                  │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐   │
│  │ URL Extractor│  │ HTTP Client  │  │ Output Manager    │   │
│  │ (src/url.rs) │  │(src/html.rs) │  │ (src/lib.rs)     │   │
│  └──────────────┘  └──────────────┘  └───────────────────┘   │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐   │
│  │ HTML Cleaner │  │ MD Converter │  │ Error Handler     │   │
│  │ (Monolith)   │  │ (htmd)       │  │ (anyhow)         │   │
│  └──────────────┘  └──────────────┘  └───────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. CLI Interface (`src/cli.rs`, `src/main.rs`)

**Responsibilities:**
- Parse command-line arguments using Clap
- Handle input from files or stdin
- Configure logging and verbosity
- Implement panic recovery

**Features:**
- Flexible input methods
- Custom error messages
- Panic hook for malformed HTML

### 2. URL Processing (`src/url.rs`)

**Responsibilities:**
- Extract URLs from text
- Validate and normalize URLs
- Create filesystem paths from URL structure
- Resolve relative URLs

**Implementation:**
- `linkify` for URL detection
- `html5ever` for HTML parsing
- Support for local file paths (file://)
- Automatic deduplication

### 3. HTML Processing (`src/html.rs`)

**Responsibilities:**
- Fetch HTML content
- Clean HTML with Monolith
- Handle timeouts and retries
- Configure HTTP client

**Features:**
- Curl-based HTTP client
- Panic recovery for Monolith
- Exponential backoff retries
- Connection pooling

**HTTP Configuration:**
- HTTP/2 preferred
- Chrome 120 User-Agent on macOS
- Sec-Ch-Ua headers for CDN compatibility
- 20-second connection timeout, 60-second total

### 4. Markdown Conversion (`src/markdown.rs`)

**Responsibilities:**
- Convert HTML to Markdown
- Preserve document structure
- Handle edge cases

**Implementation:**
- Wrapper around `htmd`
- Fallback for conversion failures
- Semantic structure preservation

### 5. Core Library (`src/lib.rs`)

**Responsibilities:**
- Orchestrate processing pipeline
- Manage concurrent operations
- Generate output files
- Track progress

**Concurrency:**
- Worker pool based on CPU cores
- Semaphore limiting
- Tokio async/await runtime
- Progress bars

## Data Flow

```
Input (File/Stdin)
       │
       ▼
URL Extraction ──────► URL Validation
       │                     │
       │                     ▼
       │              URL Normalization
       │                     │
       └─────────────────────┘
                 │
                 ▼
         Concurrent Processing
         ┌───────┴───────┐
         ▼               ▼
    HTTP Fetch      Local File Read
         │               │
         ▼               ▼
    HTML Cleaning   Pass-through
         │               │
         ▼               ▼
    MD Conversion   MD Conversion
         │               │
         └───────┬───────┘
                 │
                 ▼
           Output Writing
         ┌───────┴───────┐
         ▼               ▼
    Directory        Packed File
    Structure          Output
```

## Error Handling

### 1. Graceful Degradation
- Monolith panic → Basic HTML cleanup
- HTTP timeout → Curl retry
- Conversion failure → Text extraction

### 2. Retry Mechanism
```rust
Retry Logic:
- Initial attempt
- Retry 1: 1 second delay
- Retry 2: 2 second delay  
- Retry 3: 4 second delay
- Report failure
```

### 3. Error Propagation
- `anyhow` for error context
- Collect all errors in batch mode
- Continue processing after failures

## Performance

### 1. Concurrency
- Worker pool: `min(CPU_COUNT * 2, 16)`
- Prevents resource exhaustion
- Balances throughput and system load

### 2. Memory
- Streaming for large documents
- Connection reuse
- Efficient string operations

### 3. I/O
- Async file operations
- Buffered writing
- Batched directory creation

## Security

### 1. Content
- No JavaScript execution
- No external resource loading
- CSS and images stripped
- iframe content ignored

### 2. Network
- Spoofed User-Agent
- Configurable SSL/TLS verification
- Timeout protection

### 3. File System
- Path sanitization
- No directory traversal
- Safe special character handling

## Extension Points

### 1. URL Extractors
Add new extractors by:
- Implementing pattern matching
- Adding to `extract_urls_from_text`
- Following validation patterns

### 2. Output Formats
Potential additions:
- JSON
- EPUB
- Plain text

### 3. Processing Plugins
Options include:
- Custom HTML processors
- Content filters
- Metadata extractors

## Testing

### Unit Tests
- 40+ module-level tests
- Mock HTTP with curl
- URL parsing edge cases
- HTML structure fixtures

### Integration Tests
- 6+ end-to-end workflows
- Local file processing
- Concurrency verification
- Output mode testing

### Performance Tests
- 100+ URL load testing
- Memory profiling
- Bottleneck analysis
- Timeout/retry validation

### Issue Verification
- `issues/issuetest.py` test suite
- Validates bug fixes
- CLI and output testing
- Regression prevention

## Future Work

### 1. Streaming
- Process without full memory load
- Progressive output
- Real-time pipeline

### 2. Distributed Processing
- Job queue system
- Horizontal scaling
- Result aggregation

### 3. Smart Caching
- Content deduplication
- Incremental updates
- ETa/Last-Modified support

## CDN Compatibility

### Compatible CDNs
- **Cloudflare**: Bot detection bypass
- **Fastly**: HTTP/2 support
- **Akamai**: Edge case handling
- **Adobe CDN**: Timeout fixes

### Compatibility Features
1. HTTP version negotiation (ALPN)
2. Modern browser headers
3. Browser-like TLS behavior
4. Realistic User-Agent

### Headers
- Chrome 120 User-Agent
- HTML Accept headers
- Sec-Ch-Ua Client Hints
- Sec-Fetch metadata
- Cache-Control/Pragma

## Dependencies

### Core
- `tokio`: Async runtime
- `curl`: HTTP client
- `monolith`: HTML cleaning
- `htmd`: Markdown conversion
- `clap`: CLI parsing
- `anyhow`: Error handling

### Rationale
- Tokio: Standard async runtime
- Curl: Better CDN compatibility than pure Rust clients
- Monolith: Superior HTML cleaning
- htmd: Fast, accurate conversion

## Deployment

### Binary Size
- Optimized release builds
- Stripped symbols
- Static linking trade-offs

### Platforms
- Native binaries for major platforms
- Cross-compilation support
- CI/CD automated builds

### Configuration
- Environment variables
- No config files
- Self-contained operation