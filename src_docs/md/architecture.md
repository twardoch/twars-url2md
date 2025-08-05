# Architecture

Deep dive into the technical architecture, design decisions, and implementation details of `twars-url2md`.

## System Overview

`twars-url2md` is built with a modular, async-first architecture in Rust, emphasizing performance, reliability, and maintainability.

<div class="arch-diagram">
```mermaid
graph TB
    CLI[CLI Interface<br/>src/cli.rs] --> URLExt[URL Extractor<br/>src/url.rs]
    CLI --> Lib[Core Library<br/>src/lib.rs]
    
    URLExt --> Valid[URL Validation<br/>& Normalization]
    Valid --> HTTP[HTTP Client<br/>src/html.rs]
    
    HTTP --> Mono[Monolith<br/>HTML Cleaner]
    Mono --> Conv[Markdown Converter<br/>src/markdown.rs]
    
    Conv --> Output[Output Writer<br/>File System]
    
    Lib --> Async[Tokio Runtime<br/>Async Orchestration]
    Async --> Conc[Concurrent Processing<br/>Buffer Unordered]
    
    Error[Error Handler<br/>anyhow + tracing] --> All[All Components]
    
    style CLI fill:#e1f5fe
    style HTTP fill:#f3e5f5
    style Mono fill:#e8f5e8
    style Conv fill:#fff3e0
    style Output fill:#fce4ec
```
</div>

## Core Components

### 1. CLI Interface (`src/cli.rs`)

Command-line argument parsing and user interaction layer.

**Key Responsibilities:**
- Argument parsing with `clap` derive macros
- Input validation and error handling
- Configuration setup and environment detection
- User-friendly error messages

**Design Patterns:**
```rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "URLs to process")]
    pub urls: Vec<String>,
    
    #[arg(short, long, help = "Input file containing URLs")]
    pub input: Option<PathBuf>,
    
    #[arg(short, long, help = "Output directory or file")]
    pub output: Option<PathBuf>,
    
    // ... additional arguments
}
```

### 2. URL Processing (`src/url.rs`)

URL extraction, validation, and normalization engine.

**Architecture Features:**
- **Multi-format URL extraction** using `linkify` crate
- **Base URL resolution** for relative links
- **URL validation and deduplication**
- **Output path generation** from URL structure

**Processing Pipeline:**
```rust
pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String> {
    // 1. Extract URLs using linkify
    let mut urls = extract_raw_urls(text);
    
    // 2. Extract from HTML/Markdown if present
    urls.extend(extract_from_html(text));
    urls.extend(extract_from_markdown(text));
    
    // 3. Resolve relative URLs
    if let Some(base) = base_url {
        urls = resolve_relative_urls(urls, base);
    }
    
    // 4. Validate and deduplicate
    validate_and_deduplicate(urls)
}
```

### 3. HTTP Client (`src/html.rs`)

Robust HTTP client built on `curl` for maximum compatibility.

**CDN Compatibility Features:**
- **Browser-like User-Agent** to avoid bot detection
- **HTTP/2 auto-negotiation** preferred by modern CDNs
- **Comprehensive header set** mimicking real browsers
- **Connection pooling** for efficient resource usage

**Request Configuration:**
```rust
const USER_AGENT_STRING: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

fn configure_request(easy: &mut Easy) -> Result<()> {
    easy.useragent(USER_AGENT_STRING)?;
    easy.http_version(HttpVersion::V2)?;
    easy.follow_location(true)?;
    easy.max_redirections(10)?;
    easy.timeout(Duration::from_secs(60))?;
    easy.connect_timeout(Duration::from_secs(20))?;
    
    // Browser-like headers
    let headers = vec![
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept-Language: en-US,en;q=0.5",
        "Accept-Encoding: gzip, deflate",
        "Sec-Fetch-Dest: document",
        "Sec-Fetch-Mode: navigate",
        "Sec-Fetch-Site: none",
        "Upgrade-Insecure-Requests: 1",
    ];
    
    easy.http_headers(headers)?;
    Ok(())
}
```

### 4. HTML Processing Pipeline

Two-stage HTML cleaning and conversion process.

#### Stage 1: Monolith Integration
- **Content Extraction**: Removes scripts, styles, ads
- **Resource Handling**: Processes images and links
- **Panic Recovery**: Catches Monolith panics to prevent crashes

```rust
pub async fn fetch_and_clean_html(url: &str) -> Result<String> {
    // Fetch HTML content
    let html_content = fetch_html_content(url).await?;
    
    // Clean with Monolith (with panic recovery)
    let cleaned_html = std::panic::catch_unwind(|| {
        monolith::html::clean(&html_content, &url, &monolith_options())
    }).map_err(|_| anyhow!("HTML cleaning panicked"))?;
    
    Ok(cleaned_html)
}
```

#### Stage 2: Markdown Conversion
- **Semantic Preservation**: Maintains heading hierarchy
- **Link Processing**: Preserves and normalizes links
- **Structure Retention**: Tables, lists, code blocks

### 5. Async Processing Engine (`src/lib.rs`)

High-performance concurrent processing using Tokio.

**Concurrency Architecture:**
```rust
pub async fn process_urls(urls: Vec<String>, config: Config) -> Result<Vec<(String, Error)>> {
    let concurrency_limit = determine_optimal_concurrency();
    
    let results = futures::stream::iter(urls)
        .map(|url| process_single_url(url, &config))
        .buffer_unordered(concurrency_limit)
        .collect::<Vec<_>>()
        .await;
    
    handle_results(results)
}

fn determine_optimal_concurrency() -> usize {
    let cpu_count = num_cpus::get();
    std::cmp::min(cpu_count * 2, 16) // I/O bound, cap at 16
}
```

**Error Aggregation:**
- Individual URL failures don't stop batch processing
- Comprehensive error reporting at completion
- Retry logic with exponential backoff

### 6. Output Management

Flexible output system supporting multiple formats and structures.

**Output Modes:**
- **Directory Structure**: Mirrors URL hierarchy
- **Single File**: Concatenated content
- **Packed Format**: URL-separated sections

```rust
pub enum OutputMode {
    Directory { base_path: PathBuf },
    SingleFile { file_path: PathBuf },
    Packed { file_path: PathBuf },
    Combined { dir_path: PathBuf, pack_path: PathBuf },
}

impl OutputMode {
    pub async fn write_content(&self, url: &str, content: &str) -> Result<()> {
        match self {
            Self::Directory { base_path } => {
                let file_path = url_to_file_path(url, base_path);
                write_file_with_dirs(&file_path, content).await
            },
            Self::Packed { file_path } => {
                append_with_header(file_path, url, content).await
            },
            // ... other modes
        }
    }
}
```

## Performance Optimizations

### 1. Adaptive Concurrency

Dynamic concurrency adjustment based on system resources:
- **CPU Detection**: Uses `num_cpus` crate for core count
- **I/O Optimization**: Higher concurrency for I/O-bound operations
- **Resource Limits**: Caps maximum concurrent operations

### 2. Connection Reuse

HTTP client optimizations:
- **Connection Pooling**: Reuses connections for same domains
- **Keep-Alive**: Maintains connections between requests
- **HTTP/2 Multiplexing**: Multiple requests per connection

### 3. Memory Management

Efficient memory usage patterns:
- **Streaming Processing**: Avoids loading all content in memory
- **Lazy Evaluation**: Processes URLs on-demand
- **Resource Cleanup**: Proper cleanup of temporary resources

### 4. Build Optimizations

Compile-time optimizations for release builds:
```toml
[profile.release]
lto = true                # Link-time optimization
codegen-units = 1         # Single codegen unit for better optimization
panic = "unwind"          # Stack unwinding for panic recovery
strip = true              # Strip debug symbols
opt-level = 3             # Maximum optimization
```

## Error Handling Strategy

### Multi-Layer Error Handling

1. **Network Layer**: Connection timeouts, DNS failures
2. **Protocol Layer**: HTTP errors, redirects
3. **Content Layer**: HTML parsing, encoding issues
4. **File System Layer**: Permission errors, disk space

### Error Recovery Mechanisms

```rust
#[derive(Debug)]
pub enum ProcessingError {
    NetworkError { url: String, cause: String, retry_count: u32 },
    ParsingError { url: String, stage: String, cause: String },
    FileSystemError { path: PathBuf, operation: String, cause: String },
    ContentError { url: String, issue: String },
}

impl ProcessingError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, 
            Self::NetworkError { retry_count, .. } if *retry_count < MAX_RETRIES
        )
    }
    
    pub fn should_skip(&self) -> bool {
        matches!(self, 
            Self::ContentError { .. } | 
            Self::ParsingError { .. }
        )
    }
}
```

### Panic Recovery

Specific panic recovery for the Monolith library:
```rust
pub fn safe_clean_html(html: &str, url: &str) -> Result<String> {
    std::panic::catch_unwind(|| {
        monolith::clean(html, url, &get_monolith_options())
    })
    .map_err(|_| anyhow!("HTML cleaning panicked for URL: {}", url))?
    .map_err(|e| anyhow!("Monolith error: {}", e))
}
```

## Logging and Observability

### Structured Logging

Using `tracing` crate for structured, contextual logging:
```rust
#[tracing::instrument(skip(content), fields(url = %url, content_size = content.len()))]
pub async fn process_content(url: &str, content: &str) -> Result<String> {
    tracing::info!("Starting content processing");
    
    let cleaned = clean_html(content).await
        .map_err(|e| {
            tracing::error!(error = %e, "HTML cleaning failed");
            e
        })?;
    
    tracing::debug!(cleaned_size = cleaned.len(), "HTML cleaning completed");
    
    let markdown = convert_to_markdown(&cleaned).await?;
    
    tracing::info!(
        markdown_size = markdown.len(),
        "Content processing completed successfully"
    );
    
    Ok(markdown)
}
```

### Performance Metrics

Built-in performance tracking:
- Request timing and throughput
- Memory usage monitoring  
- Error rate tracking
- Retry attempt statistics

## Security Considerations

### Input Validation

- **URL Validation**: Strict URL format checking
- **Path Traversal Prevention**: Safe output path generation
- **Content Size Limits**: Protection against memory exhaustion

### Network Security

- **TLS Verification**: Certificate validation enabled by default
- **Redirect Limits**: Maximum redirect count to prevent loops
- **Timeout Enforcement**: Request timeouts to prevent hanging

### File System Safety

```rust
pub fn safe_output_path(base: &Path, url: &str) -> Result<PathBuf> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str()
        .ok_or_else(|| anyhow!("Invalid host in URL"))?;
    
    // Sanitize path components
    let path_segments: Vec<String> = parsed_url
        .path_segments()
        .unwrap_or_default()
        .map(|s| sanitize_filename(s))
        .filter(|s| !s.is_empty() && s != "." && s != "..")
        .collect();
    
    let mut output_path = base.join(sanitize_filename(host));
    for segment in path_segments {
        output_path = output_path.join(segment);
    }
    
    // Ensure path is within base directory
    if !output_path.starts_with(base) {
        return Err(anyhow!("Generated path outside base directory"));
    }
    
    Ok(output_path.with_extension("md"))
}
```

## Build System

### Build Metadata Integration

Runtime build information embedded during compilation:
```rust
// build.rs
use built::write_built_file;

fn main() {
    write_built_file().expect("Failed to acquire build info");
}

// src/lib.rs  
pub fn version() -> String {
    format!(
        "{} (built {} for {})",
        built_info::PKG_VERSION,
        built_info::BUILT_TIME_UTC,
        built_info::TARGET
    )
}
```

### Cross-Platform Builds

Multi-target build configuration:
- **Linux**: x86_64, aarch64, musl variants
- **macOS**: Intel and Apple Silicon
- **Windows**: x86_64 MSVC

### Dependency Management

Strategic dependency choices:
- **Core Dependencies**: Minimal, well-maintained crates
- **Feature Flags**: Optional functionality to reduce binary size
- **Version Pinning**: Careful balance between updates and stability

## Testing Architecture

### Test Structure

```
tests/
├── unit/           # Unit tests for individual components
├── integration/    # Integration tests for workflows  
├── fixtures/       # Test data and expected outputs
└── benchmarks/     # Performance benchmarks
```

### Test Categories

1. **Unit Tests**: Individual component functionality
2. **Integration Tests**: End-to-end workflow testing
3. **Performance Tests**: Benchmark critical paths
4. **Compatibility Tests**: Different input formats and edge cases

---

!!! note "Design Philosophy"
    The architecture prioritizes:
    
    - **Reliability**: Graceful error handling and recovery
    - **Performance**: Async processing and efficient resource usage  
    - **Maintainability**: Modular design with clear separation of concerns
    - **Extensibility**: Easy to add new input/output formats
    - **User Experience**: Informative feedback and intuitive behavior