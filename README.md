# twars-url2md

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Documentation](https://docs.rs/twars-url2md/badge.svg)](https://docs.rs/twars-url2md)
![GitHub Release Date](https://img.shields.io/github/release-date/twardoch/twars-url2md)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/twardoch/twars-url2md/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/twardoch/twars-url2md/actions/workflows/ci.yml/badge.svg)](https://github.com/twardoch/twars-url2md/actions)

**`twars-url2md`** is a fast and robust command-line tool written in Rust that fetches web pages, cleans up their HTML content, and converts them into clean Markdown files. It's designed for high-performance batch processing with intelligent error handling and recovery.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Library Usage](#library-usage)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Development](#development)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)
- [License](#license)
- [Author](#author)

## Features

### Core Functionality

- 🚀 **High-Performance Processing**: Concurrent URL processing with adaptive CPU utilization
- 📝 **Clean Markdown Output**: Converts complex HTML to readable Markdown using `htmd`
- 🧹 **Advanced HTML Cleaning**: Uses Monolith for removing scripts, styles, and unnecessary elements
- 🔄 **Robust Error Recovery**: Automatic retries with exponential backoff and panic recovery
- 📊 **Progress Tracking**: Real-time progress bars for batch operations

### URL Handling

- 🔍 **Smart URL Extraction**: Finds URLs in plain text, HTML, and Markdown formats
- 🔗 **Intelligent Resolution**: Handles relative URLs with base URL support
- 📁 **Local File Support**: Process local HTML files alongside remote URLs
- ✅ **URL Validation**: Filters out invalid and non-HTTP(S) URLs automatically

### Input/Output Options

- 📥 **Multiple Input Methods**: File input, stdin, or direct command-line arguments
- 📂 **Organized Output**: Creates logical directory structure based on URL paths
- 📦 **Pack Mode**: Combine multiple pages into a single Markdown file
- 🖥️ **Cross-Platform**: Works on Windows, macOS, and Linux

### Advanced Features

- 🌐 **Dual HTTP Engines**: Primary `reqwest` client with `curl` fallback for compatibility
- 🛡️ **Security**: Skips JavaScript, prevents XSS, and handles malformed content safely
- 📈 **Scalable**: Processes hundreds of URLs efficiently with connection pooling
- 🔧 **Configurable**: Verbose logging, custom output paths, and retry settings

## Installation

### Pre-compiled Binaries (Recommended)

Download the latest release for your platform:

```bash
# macOS (Universal binary for Intel and Apple Silicon)
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-macos-universal.tar.gz | tar xz
sudo mv twars-url2md /usr/local/bin/

# Linux x86_64
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-x86_64.tar.gz | tar xz
sudo mv twars-url2md /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-windows-x86_64.zip -OutFile twars-url2md.zip
Expand-Archive twars-url2md.zip -DestinationPath .
```

### Install via Cargo

```bash
# Requires Rust 1.70.0 or later
cargo install twars-url2md
```

### Build from Source

```bash
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md
cargo build --release
cargo install --path .
```

## Usage

### Command Line Interface

```
twars-url2md [OPTIONS]

Options:
  -i, --input <FILE>       Input file containing URLs (one per line)
  -o, --output <PATH>      Output directory or file (.md for single file output)
      --stdin              Read URLs from standard input
      --base-url <URL>     Base URL for resolving relative links
  -p, --pack <FILE>        Pack all content into a single Markdown file
  -v, --verbose            Enable verbose output with detailed logging
  -h, --help               Print help information
  -V, --version            Print version information
```

### Input Formats

The tool accepts various input formats:

1. **Plain URLs**:
   ```
   https://example.com
   https://another-site.com/page
   ```

2. **HTML with links**:
   ```html
   <a href="https://example.com">Example</a>
   <img src="https://example.com/image.jpg">
   ```

3. **Markdown with links**:
   ```markdown
   [Example](https://example.com)
   ![Image](https://example.com/image.jpg)
   ```

4. **Local files**:
   ```
   /path/to/file.html
   file:///absolute/path/file.html
   ```

## Examples

### Basic Usage

```bash
# Process a single URL
echo "https://rust-lang.org" | twars-url2md --stdin

# Process URLs from a file
twars-url2md -i urls.txt -o ./output

# Process with verbose logging
twars-url2md -i urls.txt -o ./output -v
```

### Advanced Usage

```bash
# Extract URLs from a webpage and process them
curl -s https://news.ycombinator.com | \
  twars-url2md --stdin --base-url https://news.ycombinator.com -o ./hn-articles

# Process local HTML files
find . -name "*.html" | twars-url2md --stdin -o ./markdown

# Create a single combined Markdown file
twars-url2md -i urls.txt --pack combined.md

# Use both individual files and packed output
twars-url2md -i urls.txt -o ./individual --pack all-content.md
```

### Single File Output

```bash
# Output to a single .md file instead of directory structure
twars-url2md -i urls.txt -o output.md
```

## Library Usage

`twars-url2md` can also be used as a Rust library:

```rust
use twars_url2md::{process_urls, Config, url::extract_urls_from_text};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Extract URLs from text
    let text = "Check out https://rust-lang.org and https://crates.io";
    let urls = extract_urls_from_text(text, None);
    
    // Configure processing
    let config = Config {
        verbose: true,
        max_retries: 3,
        output_base: std::path::PathBuf::from("./output"),
        single_file: false,
        has_output: true,
        pack_file: None,
    };
    
    // Process URLs
    let errors = process_urls(urls, config).await?;
    
    // Handle any errors
    for (url, error) in errors {
        eprintln!("Failed to process {}: {}", url, error);
    }
    
    Ok(())
}
```

See the [API documentation](https://docs.rs/twars-url2md) for more details.

## Architecture

### Component Overview

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   CLI/API   │────▶│ URL Extractor│────▶│   HTTP      │
│   Input     │     │   & Validator│     │   Client    │
└─────────────┘     └──────────────┘     └─────────────┘
                                                │
                                                ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Output    │◀────│   Markdown   │◀────│  Monolith   │
│   Writer    │     │   Converter  │     │   Cleaner   │
└─────────────┘     └──────────────┘     └─────────────┘
```

### Key Components

1. **URL Processing** (`src/url.rs`)
   - URL extraction from various text formats
   - URL validation and normalization
   - Relative URL resolution

2. **HTML Processing** (`src/html.rs`)
   - HTTP client with retry logic
   - Monolith integration for HTML cleaning
   - Panic recovery mechanism

3. **Markdown Conversion** (`src/markdown.rs`)
   - HTML to Markdown transformation
   - Structure preservation

4. **CLI Interface** (`src/cli.rs`)
   - Argument parsing
   - Input/output handling
   - Configuration management

## Configuration

### Environment Variables

- `RUST_LOG`: Control logging level (e.g., `RUST_LOG=debug twars-url2md -v`)
- `HTTP_PROXY`/`HTTPS_PROXY`: Configure proxy settings

### Output Structure

```
output/
├── example.com/
│   ├── index.md           # from https://example.com/
│   └── blog/
│       ├── post1.md       # from https://example.com/blog/post1
│       └── post2.md       # from https://example.com/blog/post2
└── docs.rust-lang.org/
    └── book/
        └── ch01-01.md     # from https://docs.rust-lang.org/book/ch01-01
```

## Development

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Check documentation
cargo doc --no-deps --open

# Security audit
cargo audit
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_url_extraction

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- Additional output formats (AsciiDoc, reStructuredText)
- Performance optimizations
- Enhanced error messages
- Additional test coverage
- Documentation improvements

## Troubleshooting

### Common Issues

**Issue**: SSL certificate errors
```bash
# Solution: The tool uses native TLS by default. For self-signed certificates:
twars-url2md -i urls.txt -o output --verbose
```

**Issue**: Timeout on large pages
```bash
# Solution: The tool has generous timeouts, but some pages may still timeout
# Check verbose output for details
```

**Issue**: Monolith panics on certain pages
```bash
# Solution: The tool includes panic recovery and will fall back to basic HTML processing
```

### Debug Mode

Enable detailed logging for troubleshooting:

```bash
RUST_LOG=debug twars-url2md -i urls.txt -o output -v
```

## Performance

- Concurrent processing with adaptive worker count
- Connection pooling for efficient HTTP requests
- Optimized for batch processing of hundreds of URLs
- Memory-efficient streaming for large documents

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

Adam Twardoch ([@twardoch](https://github.com/twardoch))

## Acknowledgments

- [Monolith](https://github.com/Y2Z/monolith) for HTML cleaning
- [htmd](https://github.com/letmutex/htmd) for Markdown conversion
- The Rust community for excellent libraries

---

For bug reports, feature requests, or questions, please open an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues).