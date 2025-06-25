# Architecture Documentation

## Overview

`twars-url2md` is designed as a high-performance, concurrent web scraping and conversion tool. The architecture emphasizes modularity, error resilience, and scalability.

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
- Implement panic recovery for robustness

**Key Features:**
- Flexible input methods
- Custom error messages for better UX
- Panic hook to prevent crashes from malformed HTML

### 2. URL Processing (`src/url.rs`)

**Responsibilities:**
- Extract URLs from various text formats
- Validate and normalize URLs
- Create filesystem paths from URL structure
- Handle relative URL resolution

**Design Decisions:**
- Uses `linkify` for robust URL detection
- Supports HTML parsing with `html5ever`
- Handles local file paths (file:// protocol)
- Deduplicates URLs automatically

### 3. HTML Processing (`src/html.rs`)

**Responsibilities:**
- Fetch HTML content from URLs
- Clean HTML using Monolith
- Handle timeouts and retries
- Manage HTTP client configuration

**Architecture Highlights:**
- Dual HTTP engine design (curl fallback)
- Panic recovery for Monolith operations
- Configurable retry mechanism with exponential backoff
- Resource cleanup and connection pooling

### 4. Markdown Conversion (`src/markdown.rs`)

**Responsibilities:**
- Convert cleaned HTML to Markdown
- Preserve document structure
- Handle edge cases gracefully

**Implementation:**
- Thin wrapper around `htmd` library
- Fallback for conversion failures
- Preserves semantic structure

### 5. Core Library (`src/lib.rs`)

**Responsibilities:**
- Orchestrate URL processing pipeline
- Manage concurrent operations
- Handle output file generation
- Progress tracking for batch operations

**Concurrency Model:**
- Adaptive worker pool based on CPU cores
- Semaphore-based concurrency limiting
- Async/await with Tokio runtime
- Progress bars for user feedback

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

## Error Handling Strategy

### 1. Graceful Degradation
- Monolith panic → Simple HTML cleanup fallback
- HTTP timeout → Curl fallback
- Conversion failure → Basic text extraction

### 2. Retry Mechanism
```rust
Retry Logic:
- Initial attempt
- Retry 1: Wait 1 second
- Retry 2: Wait 2 seconds  
- Retry 3: Wait 4 seconds
- Report failure
```

### 3. Error Propagation
- Uses `anyhow` for rich error context
- Collects all errors for batch reporting
- Non-blocking: one failure doesn't stop others

## Performance Optimizations

### 1. Concurrency
- Worker pool size: `min(CPU_COUNT * 2, 16)`
- Prevents resource exhaustion
- Balances throughput and system load

### 2. Memory Management
- Streaming for large documents
- Reuse of HTTP client connections
- Efficient string operations

### 3. I/O Optimization
- Async file operations
- Buffered writing
- Directory creation batching

## Security Considerations

### 1. Content Security
- JavaScript execution disabled
- No external resource loading
- CSS and images stripped by default
- iframe content ignored

### 2. Network Security
- User agent spoofing for compatibility
- SSL/TLS verification (configurable)
- Timeout protection against slow servers

### 3. File System Security
- Path sanitization for output files
- No directory traversal attacks
- Safe handling of special characters

## Extension Points

### 1. URL Extractors
New extractors can be added by:
- Implementing pattern matching logic
- Adding to `extract_urls_from_text`
- Following existing validation patterns

### 2. Output Formats
Additional formats could include:
- JSON structured data
- EPUB for e-readers
- Plain text extraction

### 3. Processing Plugins
Potential plugin points:
- Custom HTML processors
- Content filters
- Metadata extractors

## Testing Strategy

### Unit Tests
- Module-level testing
- Mock HTTP responses
- Edge case coverage

### Integration Tests
- End-to-end workflows
- Real HTML processing
- Concurrent operation testing

### Performance Tests
- Load testing with 100+ URLs
- Memory usage profiling
- Bottleneck identification

## Future Enhancements

### 1. Streaming Architecture
- Process large documents without full memory load
- Progressive output generation
- Real-time processing pipeline

### 2. Distributed Processing
- Job queue for URL processing
- Horizontal scaling capability
- Result aggregation service

### 3. Smart Caching
- Content deduplication
- Incremental updates
- ETa/Last-Modified support

## Dependencies

### Core Dependencies
- `tokio`: Async runtime
- `reqwest`: HTTP client (being phased out)
- `curl`: HTTP client (primary)
- `monolith`: HTML cleaning
- `htmd`: Markdown conversion
- `clap`: CLI parsing
- `anyhow`: Error handling

### Design Rationale
- **Tokio**: Industry standard async runtime
- **Curl**: Better compatibility than pure Rust clients
- **Monolith**: Best-in-class HTML cleaning
- **htmd**: Fast, accurate MD conversion

## Deployment Considerations

### Binary Size
- Release builds with optimization
- Strip symbols for smaller size
- Consider static linking trade-offs

### Platform Support
- Native binaries for major platforms
- Cross-compilation setup
- CI/CD for automated builds

### Configuration
- Environment variables for runtime config
- No configuration files needed
- Self-contained operation