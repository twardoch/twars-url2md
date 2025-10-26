# twars-url2md

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Downloads](https://img.shields.io/crates/d/twars-url2md)](https://crates.io/crates/twars-url2md)
[![Documentation](https://docs.rs/twars-url2md/badge.svg)](https://docs.rs/twars-url2md)
[![GitHub Release](https://img.shields.io/github/v/release/twardoch/twars-url2md)](https://github.com/twardoch/twars-url2md/releases/latest)
![GitHub Release Date](https://img.shields.io/github/release-date/twardoch/twars-url2md)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/twardoch/twars-url2md/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/twardoch/twars-url2md/actions/workflows/ci.yml/badge.svg)](https://github.com/twardoch/twars-url2md/actions)
[![Security audit](https://img.shields.io/badge/security-audit-green)](https://github.com/twardoch/twars-url2md/actions)
[![Multi-platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue)](https://github.com/twardoch/twars-url2md/releases)

**`twars-url2md`** fetches web pages, cleans up their HTML content, and converts them into clean, readable Markdown files. It's designed for high-performance batch processing, making it ideal for archiving, research, content conversion, or any task requiring structured text from web sources.

## Table of Contents

- [What it Does](#what-it-does)
- [Who It's For](#who-its-for)
- [Why It's Useful](#why-its-useful)
- [Installation](#installation)
  - [Pre-compiled Binaries (Recommended)](#pre-compiled-binaries-recommended)
  - [Install via Cargo](#install-via-cargo)
  - [Build from Source](#build-from-source)
- [Usage](#usage)
  - [Command-Line Interface (CLI)](#command-line-interface-cli)
  - [Library Usage](#library-usage)
- [Technical Details](#technical-details)
  - [Architecture Overview](#architecture-overview)
  - [Key Technical Features](#key-technical-features)
  - [HTTP Client Details & CDN Compatibility](#http-client-details--cdn-compatibility)
  - [Output Structure Options](#output-structure-options)
  - [Logging Framework](#logging-framework)
  - [Build Metadata](#build-metadata)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Getting Started](#getting-started)
  - [Building](#building)
  - [Code Quality & Formatting](#code-quality--formatting)
  - [Testing](#testing)
  - [Documentation](#documentation)
  - [Security Audit](#security-audit)
  - [Publishing (for maintainers)](#publishing-for-maintainers)
- [Contributing](#contributing)
  - [Areas for Contribution](#areas-for-contribution)
- [Troubleshooting](#troubleshooting)
  - [Common Issues & Solutions](#common-issues--solutions)
  - [Debugging with Verbose Logging](#debugging-with-verbose-logging)
- [License](#license)
- [Author](#author)
- [Acknowledgments](#acknowledgments)

## What it Does

`twars-url2md` takes one or more URLs (or local HTML files) as input. For each URL, it:

1.  **Fetches** the web page content using libcurl, handling various network conditions and CDN behaviors with browser-like headers.
2.  **Converts** the HTML into well-formatted Markdown using [htmd](https://github.com/letmutex/htmd).
3.  **Saves** the Markdown to your local filesystem, either as individual files organized by URL structure or packed into a single file.

## Who It's For

This tool is valuable for:

*   **Developers:** Integrating web content conversion into applications or build processes.
*   **Researchers & Academics:** Archiving web pages for citation, analysis, or offline reading.
*   **Content Creators & Curators:** Converting articles or blog posts into Markdown for easier editing and republishing.
*   **Knowledge Workers:** Building personal knowledge bases from online resources.
*   **Anyone** needing to quickly and reliably transform web pages into a clean, portable text format.

## Why It's Useful

`twars-url2md` stands out due to its combination of speed, reliability, and the quality of its output:

*   **High-Performance Processing**: Leverages asynchronous operations and adaptive concurrency (scaling with your CPU cores) for rapid batch processing of hundreds of URLs.
*   **Clean & Readable Markdown**: Produces well-structured Markdown that accurately represents the original content's hierarchy.
*   **Advanced HTML Cleaning**: Goes beyond simple conversion by first pruning irrelevant HTML elements, resulting in focused and clutter-free Markdown.
*   **Robust Error Handling**: Features automatic retries with exponential backoff for network issues and graceful recovery from HTML parsing errors, ensuring maximum success during large jobs.
*   **Intelligent URL Handling**:
    *   Extracts URLs from plain text, HTML, and Markdown input.
    *   Resolves relative links if a base URL is provided.
    *   Supports processing local HTML files (`/path/to/file.html` or `file:///path/to/file.html`).
    *   Validates URLs and filters out non-HTTP(S) schemes.
*   **CDN Compatibility**: Designed to work smoothly with modern CDNs (like Cloudflare, Fastly, Akamai), employing browser-like headers and HTTP/2 support to avoid bot detection and connection issues.
*   **Flexible Input/Output**:
    *   Accepts URLs via command-line arguments, input files, or standard input (stdin).
    *   Organizes output into a logical directory structure mirroring the source URLs or creates a single `.md` file.
    *   Can "pack" content from multiple URLs into one consolidated Markdown file, with original URLs as headers.
*   **Cross-Platform**: Works consistently on Windows, macOS, and Linux.
*   **Configurable**: Offers options for verbose logging, custom output paths, and retry limits.

## Installation

You can install `twars-url2md` using pre-compiled binaries (recommended for most users) or by building it from source using Cargo.

### Pre-compiled Binaries (Recommended)

#### Quick Installation (One-liner)

**Linux and macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash
```

**Or with custom install directory:**
```bash
curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash -s -- --install-dir ~/.local/bin
```

#### Manual Installation

Download the latest release for your platform from the [GitHub Releases page](https://github.com/twardoch/twars-url2md/releases/latest).

**macOS:**

```bash
# Intel x86_64
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-macos-x86_64.tar.gz | tar xz
# Apple Silicon (M1/M2)
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-macos-aarch64.tar.gz | tar xz
# Move to a directory in your PATH
sudo mv twars-url2md /usr/local/bin/
```

**Linux:**

```bash
# x86_64
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-x86_64.tar.gz | tar xz
# ARM64 (aarch64)
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-aarch64.tar.gz | tar xz
# Static binary (musl)
curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-x86_64-musl.tar.gz | tar xz
# Move to a directory in your PATH
sudo mv twars-url2md /usr/local/bin/
```

**Windows:**

```powershell
# Download
Invoke-WebRequest -Uri https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-windows-x86_64.zip -OutFile twars-url2md.zip
# Extract
Expand-Archive twars-url2md.zip -DestinationPath .
# Move twars-url2md.exe to a directory in your system's PATH
# For example, move it to C:\Windows\System32 or add its current directory to PATH
```
*Note: For Windows, ensure the directory where you place `twars-url2md.exe` is included in your `PATH` environment variable to run it from any command prompt.*

### Install via Cargo

If you have Rust installed (version 1.70.0 or later), you can install `twars-url2md` directly from Crates.io:

```bash
cargo install twars-url2md
```

### Build from Source

To build `twars-url2md` from the source code:

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/twardoch/twars-url2md.git
    cd twars-url2md
    ```
2.  **Build in release mode:**
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/twars-url2md`.
3.  **Install (optional):**
    You can install it to your Cargo binary path (`~/.cargo/bin/`) by running:
    ```bash
    cargo install --path .
    ```

After installation, verify it by running:
```bash
twars-url2md --version
```

## Usage

`twars-url2md` can be used as a command-line tool or as a library in your Rust projects.

### Command-Line Interface (CLI)

The basic syntax for the CLI is:

```bash
twars-url2md [OPTIONS] [URLS...]
```

If URLs are provided directly as arguments, they will be processed. Otherwise, use `--input` or `--stdin`.

**CLI Options:**

You can view all options by running `twars-url2md --help`. Here are the main ones:

*   `-i, --input <FILE>`: Input file containing URLs (one per line, or text with extractable URLs).
*   `-o, --output <PATH>`: Output directory for Markdown files. If `<PATH>` ends with `.md`, all content will be saved into this single file instead of a directory structure (unless `--pack` is also used).
*   `--stdin`: Read URLs from standard input.
*   `--base-url <URL>`: Base URL for resolving relative links found in the input content (e.g., if parsing URLs from an HTML page).
*   `-p, --pack <FILE.md>`: Pack all converted Markdown content into a single specified `.md` file. Each URL's content will be headed by its original URL.
*   `-v, --verbose`: Enable verbose output with detailed logging (INFO and DEBUG levels).
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

**Input Formats:**

The tool can extract URLs from various input sources when using `-i` or `--stdin`:

1.  **Plain URLs:** A list of URLs, one per line.
    ```
    https://example.com
    https://another-site.com/page
    ```
2.  **HTML Content:** Text containing HTML with links (e.g., `<a href="https://example.com">Example</a>`).
3.  **Markdown Content:** Text containing Markdown links (e.g., `[Example](https://example.com)`).
4.  **Local Files:** Paths to local HTML files.
    ```
    /path/to/your/file.html
    file:///absolute/path/to/another/file.html
    ```
    *Note: For local files, the content is read and converted to Markdown directly.*

**CLI Examples:**

1.  **Process a single URL, output to console (default if no -o or --pack):**
    ```bash
    twars-url2md https://www.rust-lang.org
    ```
    *(Note: This will print Markdown to stdout. For saving, use `-o` or `--pack`)*

2.  **Process multiple URLs, save to default directory structure (`./output/<domain>/...`):**
    ```bash
    twars-url2md https://www.rust-lang.org https://crates.io -o ./output
    ```

3.  **Process URLs from a file, save to a custom directory:**
    ```bash
    # urls.txt contains one URL per line
    twars-url2md -i urls.txt -o ./markdown_files
    ```

4.  **Process URLs from stdin, with verbose logging:**
    ```bash
    echo "https://example.com" | twars-url2md --stdin -o ./output -v
    ```

5.  **Extract URLs from a webpage and process them (using `curl` as an example source):**
    ```bash
    curl -s https://news.ycombinator.com | \
      twars-url2md --stdin --base-url https://news.ycombinator.com -o ./hn_articles
    ```

6.  **Process local HTML files (using `find` to supply file paths):**
    ```bash
    find . -name "*.html" | twars-url2md --stdin -o ./local_markdown
    ```

7.  **Create a single combined Markdown file from multiple URLs (`--pack`):**
    ```bash
    twars-url2md -i urls.txt --pack combined_report.md
    ```
    *Each URL's content in `combined_report.md` will be preceded by a header like `# https://example.com/some/page`.*

8.  **Output to a single `.md` file (alternative to directory structure):**
    ```bash
    # This is useful if you have one primary URL or want a simpler output than --pack
    twars-url2md https://example.com/main_article -o article.md
    ```
    *If multiple URLs are processed and output is a single file (not using `--pack`), their content will be concatenated.*

9.  **Use both individual file output and packed output:**
    ```bash
    twars-url2md -i urls.txt -o ./individual_files --pack all_content.md
    ```

### Library Usage

`twars-url2md` can also be used as a Rust library to integrate its functionality into your own projects.

Add it to your `Cargo.toml`:

```toml
[dependencies]
twars-url2md = "0.3.0" # Replace with the latest version from crates.io
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```
*(Check [Crates.io](https://crates.io/crates/twars-url2md) for the most current version number.)*

**Example:**

```rust
use twars_url2md::{process_urls, Config, url::extract_urls_from_text};
use std::path::PathBuf;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Example text from which to extract URLs
    let text_with_urls = "Check out https://www.rust-lang.org and also see https://crates.io for packages.";
    
    // Extract URLs. A base_url (Option<&str>) can be provided if needed.
    let urls_to_process = extract_urls_from_text(text_with_urls, None);

    if urls_to_process.is_empty() {
        println!("No URLs found to process.");
        return Ok(());
    }

    // Configure the processing task
    let config = Config {
        verbose: true,        // Enable detailed internal logging (uses `tracing` crate)
        max_retries: 3,       // Max *additional* retries after the initial attempt (so, 3 means up to 4 total attempts)
        output_base: PathBuf::from("./my_markdown_output"), // Base path for output
        single_file: false,   // False: create directory structure; True: if output_base is a file.md, save all to it
        has_output: true,     // True if output_base is a path to save to, false if just processing (e.g. for pack_file only)
        pack_file: Some(PathBuf::from("./packed_documentation.md")), // Optional: combine all into one .md file
    };

    // Ensure the output directory exists if not using pack_file primarily or single_file mode
     if config.has_output && !config.single_file && config.output_base.extension().is_none() {
        if !config.output_base.exists() {
            tokio::fs::create_dir_all(&config.output_base).await?;
        }
    } else if config.has_output && config.single_file { // Output is a single file
        if let Some(parent) = config.output_base.parent() { // Ensure parent directory for the single file exists
            if !parent.exists() {
                 tokio::fs::create_dir_all(parent).await?;
            }
        }
    }
    
    // Process the URLs
    // `process_urls` returns a Result containing a list of (String, anyhow::Error) for failed URLs.
    match process_urls(urls_to_process, config).await {
        Ok(errors) => {
            if errors.is_empty() {
                println!("All URLs processed successfully!");
            } else {
                eprintln!("Some URLs failed to process:");
                for (url, error) in errors {
                    eprintln!("- {}: {}", url, error);
                }
            }
        }
        Err(e) => {
            eprintln!("A critical error occurred during processing: {}", e);
        }
    }
    
    Ok(())
}
```

For more detailed information on the library API, please refer to the [official documentation on docs.rs](https://docs.rs/twars-url2md).

## Technical Details

This section provides a deeper dive into the architecture and inner workings of `twars-url2md`.

### Architecture Overview

`twars-url2md` is built with a modular design in Rust, emphasizing performance, concurrency, and resilience.

**Core Components:**

```
┌──────────────────┐     ┌───────────────────┐     ┌──────────────────┐
│   CLI / Library  │────▶│   URL Extractor   │────▶│    HTTP Client   │
│  (Input Handler) │     │  (src/url.rs)     │     │  (src/html.rs)   │
└──────────────────┘     └───────────────────┘     └─────────┬────────┘
                                                            │
                                                            ▼
┌──────────────────┐     ┌───────────────────┐     ┌─────────┴────────┐
│  Output Writer   │◀────│ Markdown Converter│◀────│   HTML Cleaner   │
│ (File System)    │     │ (src/markdown.rs) │     │ (Monolith Lib)   │
└──────────────────┘     └───────────────────┘     └──────────────────┘
```

1.  **CLI / Library Interface (`src/cli.rs`, `src/lib.rs`):**
    *   Handles command-line argument parsing (using `clap`) or library function calls.
    *   Manages input sources (files, stdin, direct arguments).
    *   Orchestrates the overall processing flow.

2.  **URL Extractor & Validator (`src/url.rs`):**
    *   Extracts URLs from various text formats (plain text, HTML, Markdown) using libraries like `linkify`.
    *   Validates, normalizes, and deduplicates URLs.
    *   Resolves relative URLs against a provided base URL.
    *   Generates structured output paths based on URL components.

3.  **HTTP Client (`src/html.rs`):**
    *   Fetches web content using a robust, `curl`-based HTTP client (`curl_rust` crate).
    *   Configured to mimic browser behavior to enhance compatibility with modern websites and CDNs (see "HTTP Client Details & CDN Compatibility" below).
    *   Implements retry logic with exponential backoff for transient network errors.

4.  **HTML Cleaner (Monolith Integration):**
    *   Utilizes the [Monolith library](https://github.com/Y2Z/monolith) to process and clean raw HTML.
    *   Strips scripts, styles, iframes, and other non-essential elements.
    *   Aims to isolate the main article or content body of the page.
    *   Includes panic recovery: if Monolith encounters an issue with severely malformed HTML, `twars-url2md` attempts a fallback to basic HTML processing or skips the URL, logging an error and allowing the batch job to continue.

5.  **Markdown Converter (`src/markdown.rs`):**
    *   Converts the cleaned HTML into Markdown using the [htmd library](https://github.com/letmutex/htmd).
    *   Preserves semantic structure (headings, lists, links, etc.).

6.  **Output Writer (`src/lib.rs`):**
    *   Manages writing the Markdown content to the filesystem.
    *   Supports creating a directory structure based on URL paths, outputting to a single specified file, or packing all content into one file.

### Key Technical Features

*   **Concurrent Processing:**
    *   Leverages Tokio's asynchronous runtime for non-blocking I/O.
    *   Processes multiple URLs concurrently using an adaptive worker pool. The number of concurrent workers is typically based on available CPU cores (e.g., `min(CPU_COUNT * 2, 16)`), optimizing throughput without overloading the system.
    *   Uses `futures::stream::StreamExt::buffer_unordered` for managing concurrent tasks.

*   **Robust Error Handling:**
    *   Uses the `anyhow` crate for flexible and context-rich error reporting.
    *   **Retry Mechanism:** For network-related issues during fetching, `twars-url2md` automatically retries. The `Config.max_retries` field (default `2` for CLI, configurable for library use) specifies the number of *additional* attempts after the first one. So, `max_retries: 2` means up to 3 total attempts. Retries use exponential backoff.
    *   **Panic Recovery:** Specifically for the Monolith HTML cleaning stage, if Monolith panics (e.g., due to extremely malformed HTML), the application catches the panic for that specific URL, logs an error, and attempts to fall back to a simpler HTML processing path or skips the URL, allowing the batch job to continue.

### HTTP Client Details & CDN Compatibility
The built-in HTTP client is carefully configured to maximize compatibility with various web servers and Content Delivery Networks (CDNs):
*   **Underlying Engine:** Uses `curl` via the `curl_rust` crate, known for its robustness and wide protocol support.
*   **User-Agent:** Sends a common browser User-Agent string (e.g., `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36`) to appear like a standard browser. This is defined in `src/lib.rs` as `USER_AGENT_STRING`.
*   **HTTP/2 Support:** Auto-negotiates HTTP version, preferring HTTP/2 where available for better performance and compatibility with CDNs that might otherwise challenge or block older clients.
*   **Browser-like Headers:** Sends a comprehensive set of HTTP headers (e.g., `Accept`, `Accept-Language`, `Sec-Ch-Ua`, `Sec-Fetch-Site`, etc.) to further mimic legitimate browser traffic, reducing the likelihood of being blocked by bot detection systems.
*   **Timeouts:** Implements connection and total request timeouts (e.g., 20s for connection, 60s total by default in `curl_http_client`) to prevent indefinite hangs.

### Output Structure Options
*   **Directory Structure (default with `-o <directory>`):** Creates a hierarchy like `output_dir/example.com/path/to/page.md`.
*   **Single File (with `-o <filename>.md`):** Concatenates all Markdown content into the specified file. If multiple URLs are processed, their content is simply joined. This mode is active if the path given to `-o` ends with `.md` and is not a directory.
*   **Packed File (with `--pack <filename>.md`):** Combines Markdown from all URLs into a single file. Each URL's content is clearly demarcated by a heading like `# <URL>`. This mode also preserves the original input order of URLs in the output file.

### Logging Framework
*   Utilizes the `tracing` crate for structured and configurable logging.
*   Log levels (from most to least severe): `ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`.
*   Verbosity is controlled by the `-v, --verbose` flag (enables INFO and DEBUG for `twars_url2md` modules by setting `RUST_LOG=info,twars_url2md=debug`).
*   Finer-grained control is available via the `RUST_LOG` environment variable (e.g., `RUST_LOG=twars_url2md=trace` for maximum detail from this application, or `RUST_LOG=info` for general info level). See `src/main.rs` for initialization logic.

### Build Metadata
*   The build process (via `build.rs`) embeds build time, target architecture, and profile (debug/release) into the binary. This information is accessible via the `twars-url2md --version` command (see `src/lib.rs` `version()` function).

## Development

This section provides guidance for setting up a development environment, building the project, and running tests.

### Prerequisites

*   **Rust:** Version 1.70.0 or later. You can install Rust via [rustup](https://rustup.rs/).
*   **Cargo:** The Rust package manager (installed with Rust).
*   **Git:** For cloning the repository.
*   **(Optional) Python 3:** For running the issue verification suite (`issues/issuetest.py`).

### Getting Started

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/twardoch/twars-url2md.git
    cd twars-url2md
    ```

### Building

The project includes convenient build scripts for different scenarios:

*   **Quick build (fastest, no checks):**
    ```bash
    ./build.sh --quick
    ```
    Skips formatting, linting, and tests. Just builds the release binary (~2-3 minutes).

*   **Full build (recommended for development):**
    ```bash
    ./build.sh
    ```
    Runs formatting, linting, tests, and builds release binary (default behavior).

*   **CI/CD build:**
    ```bash
    ./build.sh --ci
    ```
    Clean build with tests, ideal for continuous integration.

*   **Production release:**
    ```bash
    ./build.sh --release
    ```
    Clean build with binary stripping for smallest size.

*   **Manual cargo commands:**
    - Debug build: `cargo build` (output: `target/debug/twars-url2md`)
    - Release build: `cargo build --release` (output: `target/release/twars-url2md`)
    - Run directly: `cargo run -- -i urls.txt -o ./output`

Run `./build.sh --help` to see all available options and modes.

### Code Quality & Formatting

Consistent code style is maintained using `rustfmt`, and `clippy` is used for linting.

*   **Run code quality checks:**
    ```bash
    ./scripts/lint.sh
    ```
    Checks formatting and runs clippy linter.

*   **Auto-fix issues:**
    ```bash
    ./scripts/lint.sh --fix
    ```
    Automatically formats code and applies clippy auto-fixes.

*   **Manual cargo commands:**
    - Format code: `cargo fmt`
    - Check formatting: `cargo fmt --check`
    - Run linter: `cargo clippy --all-targets --all-features -- -D warnings`

### Testing

The project has a suite of unit and integration tests.

*   **Run all tests:**
    ```bash
    cargo test --all-features
    ```

*   **Run a specific test function:**
    ```bash
    # Example: cargo test test_url_extraction --all-features
    cargo test <TEST_FUNCTION_NAME> --all-features
    ```
    *(Replace `<TEST_FUNCTION_NAME>` with the actual test function name.)*

*   **Run tests with output (e.g., for debugging print statements):**
    ```bash
    cargo test --all-features -- --nocapture
    ```

*   **Run only integration tests:**
    Integration tests are typically located in the `tests/` directory (e.g., `tests/integration/e2e_tests.rs`).
    ```bash
    # Example: cargo test --test e2e_tests --all-features
    cargo test --test <INTEGRATION_TEST_FILENAME_WITHOUT_RS> --all-features
    ```

*   **Issue Verification Suite:**
    The project includes an issue verification script to test various CLI functionalities and confirm fixes for reported issues.
    ```bash
    python3 issues/issuetest.py
    ```
    *(Ensure you have Python 3 installed and any dependencies listed in or for that script.)*

### Documentation

*   **Generate and open local documentation:**
    ```bash
    cargo doc --no-deps --open
    ```

### Security Audit

*   **Check for known vulnerabilities in dependencies:**
    ```bash
    cargo audit
    ```
    *(You may need to install `cargo-audit` first: `cargo install cargo-audit`)*

### Publishing (for maintainers)

*   **Verify package before publishing:**
    ```bash
    cargo package
    ```
*   **Publish to Crates.io:**
    ```bash
    cargo publish
    ```

## Contributing

Contributions are welcome and greatly appreciated! Whether it's bug reports, feature suggestions, documentation improvements, or code contributions, your help makes `twars-url2md` better.

Please see the [**CONTRIBUTING.md**](CONTRIBUTING.md) file for detailed guidelines on how to contribute to the project, including information on reporting issues, submitting pull requests, and the code of conduct.

### Areas for Contribution

If you're looking for ways to contribute, here are some areas where help would be valuable:

*   **Additional Output Formats:** Implementing converters for formats like AsciiDoc, reStructuredText, or others.
*   **Performance Optimizations:** Identifying and improving bottlenecks in processing speed or memory usage.
*   **Enhanced Error Messages:** Making error messages even more user-friendly and actionable.
*   **Expanded Test Coverage:** Adding more unit, integration, or end-to-end tests, especially for edge cases.
*   **Documentation:** Improving the README, API documentation, or adding usage examples.
*   **New Features:** Proposing and implementing new functionalities that align with the tool's purpose.

Before starting significant work, it's a good idea to open an issue to discuss your proposed changes.

## Troubleshooting

If you encounter issues while using `twars-url2md`, this section may help.

### Common Issues & Solutions

*   **SSL/TLS Certificate Errors:**
    *   **Issue:** You might see errors related to SSL/TLS certificate validation, especially with sites using self-signed certificates or older TLS configurations.
    *   **Details:** `twars-url2md` uses `curl` which typically relies on the system's certificate store. Ensure your system's CA certificates are up-to-date. For specific problematic sites, this can be complex. The tool aims for secure defaults.
    *   **Action:** Check verbose output (`-v`) for more details on the TLS handshake. If it's a corporate environment with a custom CA, ensure that CA is trusted by your system.

*   **CDN-Protected Sites (e.g., Cloudflare, Akamai, Adobe):**
    *   **Issue:** Previously, some sites behind aggressive CDNs might have blocked requests or timed out.
    *   **Solution:** Recent versions of `twars-url2md` have significantly improved CDN compatibility by:
        *   Auto-negotiating HTTP/2, which is preferred by many CDNs.
        *   Sending a comprehensive set of browser-like HTTP headers (including User-Agent, Sec-CH-UA, etc.) to avoid simplistic bot detection.
        *   Using `curl` as the underlying HTTP client, which has a network stack more aligned with browsers.
    *   **Action:** Ensure you are using the latest version of `twars-url2md`. If you still encounter issues, verbose logging (`-v` or `RUST_LOG`) can provide clues.

*   **Timeouts on Large or Slow Pages:**
    *   **Issue:** The tool might time out when processing very large pages or pages from slow-responding servers.
    *   **Details:** Default timeouts are generally generous (e.g., 60 seconds for the entire request).
    *   **Action:** Check your network connection. If the page is exceptionally large or the server is consistently slow, this might be unavoidable with default settings. Verbose output can confirm if a timeout is the cause.

*   **Monolith Panics or Poor Conversion on Specific Pages:**
    *   **Issue:** The Monolith library (for HTML cleaning) or the `htmd` library (for Markdown conversion) might struggle with extremely complex, malformed, or unusual HTML structures.
    *   **Details:** `twars-url2md` includes panic recovery for Monolith. If Monolith panics, the tool logs an error and attempts to fall back to a more basic HTML processing step or skips the URL. This prevents the entire batch from failing.
    *   **Action:** If a specific page consistently fails to convert well, it might be due to its unique structure. You can report such pages as issues, providing the URL.

### Debugging with Verbose Logging

For more detailed insight into what the tool is doing, especially when troubleshooting, use verbose logging.

*   **Using the `-v` flag:**
    The simplest way to get more logs is to add the `-v` or `--verbose` flag to your command. This typically sets the log level to show `INFO` messages from all crates and `DEBUG` messages from `twars_url2md` itself (`RUST_LOG=info,twars_url2md=debug`).
    ```bash
    twars-url2md -i urls.txt -o output -v
    ```

*   **Using the `RUST_LOG` Environment Variable:**
    For more fine-grained control, you can use the `RUST_LOG` environment variable. `twars-url2md` uses the `tracing` library.

    **Syntax:** `RUST_LOG="target[span{field=value}]=level"` (simplified: `RUST_LOG="crate_name=level,another_crate=level"`)

    **Examples:**
    *   Enable `DEBUG` level for `twars_url2md` and `INFO` for everything else (similar to `-v`):
        ```bash
        RUST_LOG=info,twars_url2md=debug twars-url2md -i urls.txt -o output
        ```
    *   Enable `TRACE` level for `twars_url2md` (very detailed, for deep debugging):
        ```bash
        RUST_LOG=twars_url2md=trace twars-url2md -i urls.txt -o output
        ```
    *   Enable `DEBUG` for the HTML processing module specifically:
        ```bash
        RUST_LOG=twars_url2md::html=debug twars-url2md -i urls.txt -o output
        ```
    *   Show all `DEBUG` level messages from all crates:
        ```bash
        RUST_LOG=debug twars-url2md -i urls.txt -o output
        ```

    **Logging Levels (most to least verbose):**
    *   `TRACE`: Extremely detailed information, typically for fine-grained debugging.
    *   `DEBUG`: Detailed information useful for debugging.
    *   `INFO`: Informational messages about the progress of the application.
    *   `WARN`: Warnings about potential issues that don't stop execution.
    *   `ERROR`: Errors that prevent a specific operation (e.g., processing one URL) but don't crash the application.

If you consistently encounter an issue not covered here, consider opening an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues) with detailed information, including the command you ran, the output, and logs if possible.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Adam Twardoch ([@twardoch](https://github.com/twardoch))

## Acknowledgments

`twars-url2md` builds upon the excellent work of others:

*   [Monolith](https://github.com/Y2Z/monolith) for robust HTML cleaning and resource embedding.
*   [htmd](https://github.com/letmutex/htmd) for efficient HTML-to-Markdown conversion.
*   The Rust community and the creators of the many high-quality crates used in this project.

---

For bug reports, feature requests, or questions, please open an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues).