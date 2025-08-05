# twars-url2md Documentation

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Downloads](https://img.shields.io/crates/d/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Documentation](https://docs.rs/twars-url2md/badge.svg)](https://docs.rs/twars-url2md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/twardoch/twars-url2md/actions/workflows/ci.yml/badge.svg)](https://github.com/twardoch/twars-url2md/actions)

## TL;DR

**`twars-url2md`** is a blazingly fast Rust CLI tool that converts web pages into clean, readable Markdown files. It handles batch processing, works with modern CDNs, and produces high-quality output perfect for documentation, archiving, or content conversion workflows.

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

<div class="feature-grid">
<div class="feature-card">
<h3>🚀 High Performance</h3>
<p>Async processing with CPU-adaptive concurrency for batch operations</p>
</div>

<div class="feature-card">
<h3>🧹 Clean Output</h3>
<p>Advanced HTML cleaning removes ads, scripts, and clutter</p>
</div>

<div class="feature-card">
<h3>🔄 Robust</h3>
<p>Automatic retries, panic recovery, and graceful error handling</p>
</div>

<div class="feature-card">
<h3>🛡️ CDN Compatible</h3>
<p>Works with Cloudflare, Fastly, Akamai and other modern CDNs</p>
</div>

<div class="feature-card">
<h3>📂 Flexible I/O</h3>
<p>Multiple input sources, structured or packed output options</p>
</div>

<div class="feature-card">
<h3>🖥️ Cross-Platform</h3>
<p>Native binaries for Linux, macOS, and Windows</p>
</div>
</div>

## What It Does

`twars-url2md` transforms web pages into clean Markdown through a sophisticated pipeline:

1. **Fetches** web content with robust HTTP client handling
2. **Cleans** HTML by removing scripts, ads, and non-essential elements  
3. **Converts** to well-structured Markdown preserving semantic meaning
4. **Organizes** output with flexible file structure options

## Performance Metrics

<div style="text-align: center; margin: 2rem 0;">
<span class="performance-metric">⚡ 100+ URLs/min</span>
<span class="performance-metric">🔄 CPU-adaptive concurrency</span>
<span class="performance-metric">📦 Single 8MB binary</span>
<span class="performance-metric">🚫 Zero runtime dependencies</span>
</div>

## Documentation Table of Contents

This documentation is organized into the following sections:

### Getting Started
- **[Installation](installation.md)** - Multiple installation methods including pre-compiled binaries, Cargo, and building from source
- **[Quick Start](quickstart.md)** - Essential examples to get you productive immediately

### User Guide  
- **[Basic Usage](usage.md)** - Command-line interface, input formats, and common workflows
- **[Advanced Features](advanced.md)** - Packed output, URL extraction, local file processing, and complex scenarios
- **[Configuration](configuration.md)** - Customization options, logging, retries, and environment variables

### Development
- **[Architecture](architecture.md)** - Technical deep dive into design, components, and performance optimizations
- **[Contributing](contributing.md)** - Development setup, coding standards, and contribution guidelines  
- **[Testing](testing.md)** - Test suite overview, running tests, and benchmarking
- **[API Reference](api.md)** - Library usage, Rust API documentation, and integration examples

## Key Use Cases

=== "Content Archiving"
    Perfect for researchers, academics, or knowledge workers who need to preserve web content for offline access or citation.
    
    ```bash
    # Archive a list of research papers
    twars-url2md -i research_urls.txt -o archive/ -v
    ```

=== "Documentation Migration"
    Convert existing web-based documentation to Markdown for integration with static site generators.
    
    ```bash
    # Convert documentation sites to markdown
    echo "https://old-docs.example.com" | twars-url2md --stdin --pack new-docs.md
    ```

=== "Content Curation"
    Build curated collections by combining multiple web sources into organized Markdown files.
    
    ```bash
    # Curate articles into a single document
    twars-url2md -i article_list.txt --pack weekly-digest.md
    ```

=== "Build Pipeline Integration"
    Integrate into CI/CD workflows or build systems for automated content processing.
    
    ```bash
    # Process in CI environment  
    twars-url2md -i $INPUT_FILE -o $OUTPUT_DIR --verbose
    ```

## Quick Navigation

| Section | Description | Best For |
|---------|-------------|----------|
| [Installation](installation.md) | Get up and running | New users |
| [Usage](usage.md) | Core functionality | Daily users |
| [Advanced Features](advanced.md) | Power user features | Complex workflows |
| [Architecture](architecture.md) | Technical details | Developers |
| [API Reference](api.md) | Library integration | Rust developers |

---

!!! tip "Need Help?"
    - Check the [troubleshooting section](usage.md#troubleshooting) for common issues
    - Visit the [GitHub repository](https://github.com/twardoch/twars-url2md) for bug reports and feature requests
    - Review the [API documentation](https://docs.rs/twars-url2md) for library usage

*Built with ❤️ in Rust by [Adam Twardoch](https://github.com/twardoch)*