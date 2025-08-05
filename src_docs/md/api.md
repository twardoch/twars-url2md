# API Reference

Complete API documentation for using `twars-url2md` as a Rust library in your applications.

## Getting Started

Add `twars-url2md` to your `Cargo.toml`:

```toml
[dependencies]
twars-url2md = "0.3.0"  # Check crates.io for latest version
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

## Core API

### Main Processing Function

#### `process_urls`

The primary function for processing multiple URLs:

```rust
pub async fn process_urls(
    urls: Vec<String>, 
    config: Config
) -> Result<Vec<(String, anyhow::Error)>>
```

**Parameters:**
- `urls`: Vector of URLs to process
- `config`: Processing configuration

**Returns:**
- `Ok(Vec<(String, Error)>)`: List of failed URLs with their errors
- `Err(anyhow::Error)`: Critical error that stopped processing

**Example:**
```rust
use twars_url2md::{process_urls, Config};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://crates.io".to_string(),
    ];
    
    let config = Config {
        output_base: PathBuf::from("./output"),
        verbose: true,
        max_retries: 3,
        single_file: false,
        has_output: true,
        pack_file: None,
    };
    
    match process_urls(urls, config).await {
        Ok(errors) => {
            if errors.is_empty() {
                println!("All URLs processed successfully!");
            } else {
                eprintln!("Some URLs failed:");
                for (url, error) in errors {
                    eprintln!("  {}: {}", url, error);
                }
            }
        }
        Err(e) => {
            eprintln!("Critical error: {}", e);
        }
    }
    
    Ok(())
}
```

### Configuration

#### `Config` Struct

Configuration for URL processing:

```rust
pub struct Config {
    pub verbose: bool,
    pub max_retries: u32,
    pub output_base: PathBuf,
    pub single_file: bool,
    pub has_output: bool,
    pub pack_file: Option<PathBuf>,
}
```

**Fields:**
- `verbose`: Enable detailed logging
- `max_retries`: Maximum retry attempts for failed requests
- `output_base`: Base path for output files
- `single_file`: Whether output_base is a single file
- `has_output`: Whether to write individual files
- `pack_file`: Optional path for packed output file

**Default Implementation:**
```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            max_retries: 2,
            output_base: PathBuf::from("./output"),
            single_file: false,
            has_output: true,
            pack_file: None,
        }
    }
}
```

## URL Processing

### URL Extraction

#### `extract_urls_from_text`

Extract URLs from various text formats:

```rust
pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String>
```

**Parameters:**
- `text`: Input text containing URLs
- `base_url`: Base URL for resolving relative links

**Returns:**
- Vector of extracted and validated URLs

**Example:**
```rust
use twars_url2md::url::extract_urls_from_text;

// Extract from plain text
let text = "Visit https://example.com and https://rust-lang.org";
let urls = extract_urls_from_text(text, None);
assert_eq!(urls.len(), 2);

// Extract from HTML with base URL
let html = r#"<a href="/docs">Documentation</a> <a href="https://external.com">External</a>"#;
let urls = extract_urls_from_text(html, Some("https://mysite.com"));
// Results: ["https://mysite.com/docs", "https://external.com"]
```

#### `validate_url`

Validate a single URL:

```rust
pub fn validate_url(url: &str) -> Result<String, anyhow::Error>
```

**Example:**
```rust
use twars_url2md::url::validate_url;

assert!(validate_url("https://example.com").is_ok());
assert!(validate_url("invalid-url").is_err());
assert!(validate_url("ftp://example.com").is_err()); // Only HTTP/HTTPS allowed
```

### Path Generation

#### `url_to_file_path`

Generate output file path from URL:

```rust
pub fn url_to_file_path(url: &str, base_path: &Path) -> PathBuf
```

**Example:**
```rust
use twars_url2md::url::url_to_file_path;
use std::path::PathBuf;

let url = "https://doc.rust-lang.org/book/ch01-01-installation.html";
let base = PathBuf::from("output");
let path = url_to_file_path(url, &base);

// Result: output/doc.rust-lang.org/book/ch01-01-installation.md
assert_eq!(
    path,
    PathBuf::from("output/doc.rust-lang.org/book/ch01-01-installation.md")
);
```

## HTML Processing

### Content Fetching

#### `fetch_and_clean_html`

Fetch and clean HTML content from URL:

```rust
pub async fn fetch_and_clean_html(url: &str) -> Result<String, anyhow::Error>
```

**Example:**
```rust
use twars_url2md::html::fetch_and_clean_html;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cleaned_html = fetch_and_clean_html("https://example.com").await?;
    println!("Cleaned HTML length: {}", cleaned_html.len());
    Ok(())
}
```

#### `fetch_html_content`

Fetch raw HTML content:

```rust
pub async fn fetch_html_content(url: &str) -> Result<String, anyhow::Error>
```

### Local File Processing

#### `process_local_file`

Process local HTML file:

```rust
pub async fn process_local_file(file_path: &str) -> Result<String, anyhow::Error>
```

**Example:**
```rust
use twars_url2md::html::process_local_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = process_local_file("/path/to/document.html").await?;
    println!("Processed content: {}", content);
    Ok(())
}
```

## Markdown Conversion

### HTML to Markdown

#### `html_to_markdown`

Convert HTML to Markdown:

```rust
pub fn html_to_markdown(html: &str) -> Result<String, anyhow::Error>
```

**Example:**
```rust
use twars_url2md::markdown::html_to_markdown;

let html = r#"
    <h1>Title</h1>
    <p>This is <strong>bold</strong> text with a 
    <a href="https://example.com">link</a>.</p>
    <ul><li>Item 1</li><li>Item 2</li></ul>
"#;

let markdown = html_to_markdown(html)?;
println!("{}", markdown);
// Output:
// # Title
// 
// This is **bold** text with a [link](https://example.com).
// 
// - Item 1
// - Item 2
```

## Advanced Usage

### Custom Processing Pipeline

Create a custom processing pipeline:

```rust
use twars_url2md::{url, html, markdown};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: Extract URLs from text
    let text = std::fs::read_to_string("input.html")?;
    let urls = url::extract_urls_from_text(&text, Some("https://base.com"));
    
    // Step 2: Process each URL
    for url in urls {
        // Fetch and clean HTML
        let cleaned_html = html::fetch_and_clean_html(&url).await?;
        
        // Convert to Markdown
        let markdown = markdown::html_to_markdown(&cleaned_html)?;
        
        // Generate output path
        let output_path = url::url_to_file_path(&url, &PathBuf::from("output"));
        
        // Create directory if needed
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Write output
        tokio::fs::write(&output_path, markdown).await?;
        
        println!("Processed: {} -> {}", url, output_path.display());
    }
    
    Ok(())
}
```

### Concurrent Processing with Custom Control

Implement custom concurrency control:

```rust
use twars_url2md::{html, markdown, url};
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let urls = vec![
        "https://example.com/page1".to_string(),
        "https://example.com/page2".to_string(),
        "https://example.com/page3".to_string(),
    ];
    
    // Limit concurrent requests
    let semaphore = Arc::new(Semaphore::new(3));
    
    let results = stream::iter(urls)
        .map(|url| {
            let semaphore = semaphore.clone();
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                process_single_url(&url).await
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;
    
    for result in results {
        match result {
            Ok(path) => println!("Success: {}", path),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}

async fn process_single_url(url: &str) -> anyhow::Result<String> {
    let html = html::fetch_and_clean_html(url).await?;
    let markdown = markdown::html_to_markdown(&html)?;
    
    let output_path = url::url_to_file_path(url, &std::path::PathBuf::from("output"));
    
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    
    tokio::fs::write(&output_path, markdown).await?;
    
    Ok(output_path.to_string_lossy().to_string())
}
```

### Error Handling Patterns

Comprehensive error handling:

```rust
use twars_url2md::{process_urls, Config};
use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug)]
struct ProcessingStats {
    total_urls: usize,
    successful: usize,
    failed: usize,
    errors: Vec<(String, String)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let urls = load_urls_from_file("urls.txt")
        .context("Failed to load URLs from file")?;
    
    let config = Config {
        output_base: PathBuf::from("./output"),
        verbose: true,
        max_retries: 3,
        single_file: false,
        has_output: true,
        pack_file: Some(PathBuf::from("archive.md")),
    };
    
    let stats = process_with_stats(urls, config).await?;
    
    print_processing_report(&stats);
    
    if stats.failed > 0 {
        std::process::exit(1);
    }
    
    Ok(())
}

async fn process_with_stats(urls: Vec<String>, config: Config) -> Result<ProcessingStats> {
    let total_urls = urls.len();
    
    let errors = process_urls(urls, config).await
        .context("Critical error during URL processing")?;
    
    let failed = errors.len();
    let successful = total_urls - failed;
    
    let error_details = errors
        .into_iter()
        .map(|(url, error)| (url, error.to_string()))
        .collect();
    
    Ok(ProcessingStats {
        total_urls,
        successful,
        failed,
        errors: error_details,
    })
}

fn load_urls_from_file(path: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let urls = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect();
    
    Ok(urls)
}

fn print_processing_report(stats: &ProcessingStats) {
    println!("Processing Report:");
    println!("  Total URLs: {}", stats.total_urls);
    println!("  Successful: {}", stats.successful);
    println!("  Failed: {}", stats.failed);
    
    if !stats.errors.is_empty() {
        println!("\nFailed URLs:");
        for (url, error) in &stats.errors {
            println!("  {}: {}", url, error);
        }
    }
}
```

## Library Integration Examples

### Web Server Integration

Using with a web framework:

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use twars_url2md::{process_urls, Config};
use std::path::PathBuf;

#[derive(Deserialize)]
struct ConvertRequest {
    urls: Vec<String>,
    output_dir: Option<String>,
    pack_output: Option<bool>,
}

#[derive(Serialize)]
struct ConvertResponse {
    success: bool,
    message: String,
    failed_urls: Vec<String>,
}

async fn convert_urls(
    Json(request): Json<ConvertRequest>,
) -> Result<ResponseJson<ConvertResponse>, StatusCode> {
    let output_dir = request.output_dir
        .unwrap_or_else(|| format!("output_{}", chrono::Utc::now().timestamp()));
    
    let pack_file = if request.pack_output.unwrap_or(false) {
        Some(PathBuf::from(format!("{}/archive.md", output_dir)))
    } else {
        None
    };
    
    let config = Config {
        output_base: PathBuf::from(&output_dir),
        verbose: false,
        max_retries: 2,
        single_file: false,
        has_output: true,
        pack_file,
    };
    
    match process_urls(request.urls, config).await {
        Ok(errors) => {
            let failed_urls: Vec<String> = errors.into_iter().map(|(url, _)| url).collect();
            let success = failed_urls.is_empty();
            
            Ok(ResponseJson(ConvertResponse {
                success,
                message: if success {
                    format!("All URLs processed successfully. Output: {}", output_dir)
                } else {
                    format!("Some URLs failed. Output: {}", output_dir)
                },
                failed_urls,
            }))
        }
        Err(e) => {
            eprintln!("Critical error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/convert", post(convert_urls));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

### CLI Tool Integration

Building a CLI tool with the library:

```rust
use clap::{Arg, Command};
use twars_url2md::{process_urls, Config, url::extract_urls_from_text};
use std::path::PathBuf;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("my-converter")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input file with URLs"))
        .arg(Arg::new("output")
            .short('o')  
            .long("output")
            .value_name("PATH")
            .help("Output directory"))
        .arg(Arg::new("extract")
            .long("extract")
            .help("Extract URLs from HTML/Markdown content"))
        .get_matches();
    
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("output"));
    
    let content = tokio::fs::read_to_string(input_file).await?;
    
    let urls = if matches.get_flag("extract") {
        extract_urls_from_text(&content, None)
    } else {
        content.lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    };
    
    let config = Config {
        output_base: output_dir,
        verbose: true,
        ..Default::default()
    };
    
    let errors = process_urls(urls, config).await?;
    
    if !errors.is_empty() {
        eprintln!("Some URLs failed to process:");
        for (url, error) in errors {
            eprintln!("  {}: {}", url, error);
        }
        std::process::exit(1);
    }
    
    println!("All URLs processed successfully!");
    Ok(())
}
```

---

!!! tip "API Best Practices"
    - Always handle the `Result` types properly
    - Use structured error handling with `anyhow`
    - Implement proper async patterns with Tokio
    - Consider implementing custom `Config` builders for complex scenarios
    - Use the `verbose` flag during development for debugging

!!! note "Performance Considerations"
    - The library handles concurrency automatically
    - For very large URL lists, consider processing in batches
    - Monitor memory usage with large concurrent operations
    - Use the retry mechanism appropriately for your use case