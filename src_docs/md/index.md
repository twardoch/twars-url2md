# twars-url2md Documentation

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Downloads](https://img.shields.io/crates/d/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Documentation](https://docs.rs/twars-url2md/badge.svg)](https://docs.rs/twars-url2md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/twardoch/twars-url2md/actions/workflows/ci.yml/badge.svg)](https://github.com/twardoch/twars-url2md/actions)

## TL;DR

**`twars-url2md`** converts web pages to clean Markdown. It processes URLs in batches, handles modern CDN challenges, and outputs structured content suitable for documentation, archiving, or automated workflows.

!!! example "Quick Start"
    ```bash
    # Install via pre-compiled binary (recommended)
    curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash
    
    # Convert a single URL
    twars-url2md https://example.com -o output/
    
    # Process multiple URLs from a file
    twars-url2md -i urls.txt -o markdown_files/
    
    # Pack multiple URLs into one file
    twars-url2md -i urls.txt --pack combined.md
    ```

## Features

- **High Performance**: Async processing with adaptive concurrency
- **Clean Output**: Strips scripts, ads, and clutter from HTML
- **Robust Operation**: Retries failed requests, recovers from panics
- **CDN Compatible**: Handles Cloudflare, Fastly, Akamai, etc.
- **Flexible I/O**: Supports various input sources and output formats
- **Cross-Platform**: Binaries available for Linux, macOS, Windows

## What It Does

`twars-url2md` transforms web pages into Markdown using this pipeline:

1. **Fetches** content with a resilient HTTP client
2. **Cleans** HTML by removing non-essential elements
3. **Converts** cleaned HTML to semantic Markdown
4. **Organizes** output as individual files or combined documents

## Performance

<div style="text-align: center; margin: 2rem 0;">
<span class="performance-metric">⚡ 100+ URLs/min</span>
<span class="performance-metric">🔄 CPU-adaptive concurrency</span>
<span class="performance-metric">📦 Single 8MB binary</span>
<span class="performance-metric">🚫 Zero runtime dependencies</span>
</div>

## Documentation Contents

### Getting Started
- **[Installation](installation.md)** – Binaries, Cargo, or building from source
- **[Quick Start](quickstart.md)** – Basic usage examples

### User Guide  
- **[Basic Usage](usage.md)** – CLI syntax, input formats, common workflows
- **[Advanced Features](advanced.md)** – Packing output, URL extraction, local file handling
- **[Configuration](configuration.md)** – Settings, logging, retries, environment variables

### Development
- **[Architecture](architecture.md)** – Design and performance details
- **[Contributing](contributing.md)** – Development setup and guidelines  
- **[Testing](testing.md)** – Running tests and benchmarks
- **[API Reference](api.md)** – Library integration and Rust API docs

## Use Cases

=== "Content Archiving"
    Preserve web pages for offline use or citation.
    
    ```bash
    twars-url2md -i research_urls.txt -o archive/ -v
    ```

=== "Documentation Migration"
    Pull online docs into Markdown for static site generators.
    
    ```bash
    echo "https://old-docs.example.com" | twars-url2md --stdin --pack new-docs.md
    ```

=== "Content Curation"
    Combine sources into organized Markdown collections.
    
    ```bash
    twars-url2md -i article_list.txt --pack weekly-digest.md
    ```

=== "Build Pipeline Integration"
    Automate content processing in CI/CD systems.
    
    ```bash
    twars-url2md -i $INPUT_FILE -o $OUTPUT_DIR --verbose
    ```

## Quick Navigation

| Section | Description | Best For |
|---------|-------------|----------|
| [Installation](installation.md) | Setup instructions | New users |
| [Usage](usage.md) | Core commands | Daily users |
| [Advanced Features](advanced.md) | Extended options | Complex tasks |
| [Architecture](architecture.md) | Technical design | Developers |
| [API Reference](api.md) | Library docs | Rust developers |

---

!!! tip "Need Help?"
    - [Troubleshooting](usage.md#troubleshooting) – Common problems and fixes
    - [GitHub repo](https://github.com/twardoch/twars-url2md) – Bugs and feature requests
    - [API docs](https://docs.rs/twars-url2md) – Library usage details

*Built in Rust by [Adam Twardoch](https://github.com/twardoch)*