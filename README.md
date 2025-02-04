# twars-url2md

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
![GitHub Release Date](https://img.shields.io/github/release-date/twardoch/twars-url2md)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/twardoch/twars-url2md/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**`twars-url2md`** is a fast and robust command-line tool written in Rust that fetches web pages, cleans up their HTML content, and converts them into clean Markdown.

You can drop a text that contains URLs onto the app, and it will find all the URLs and save Markdown versions of the pages in a logical folder structure. The output is not perfect, but the tool is fast and robust.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration & Retry Mechanism](#configuration--retry-mechanism)
- [Development and Testing](#development-and-testing)
- [CI/CD and Release Process](#cicd-and-release-process)
- [Contributing](#contributing)
- [How It Works](#how-it-works)
- [License](#license)
- [Author](#author)

## Features

### Powerful Web Content Conversion

- Extracts clean web content using Monolith
- Converts web pages to Markdown efficiently
- Handles complex URL and encoding scenarios

### Smart URL Handling

- Extracts URLs from various text formats
- Resolves and validates URLs intelligently
- Supports base URL and relative link processing

### Flexible Input & Output**

- Multiple input methods (file, stdin, CLI)
- Organized Markdown file generation
- Cross-platform compatibility

### Advanced Processing

  - Parallel URL processing
  - Robust error handling

## Install CLI app

#### ☛ [Download CLI app](https://github.com/twardoch/twars-url2md/releases) for Mac, Windows or Linux

Pre-compiled binary builds for macOS (Apple/Intel), Windows (x86_64), and Linux (x86_64) are on the [releases page](https://github.com/twardoch/twars-url2md/releases).

## Other ways to install

### From Crates.io

Make sure you have [Rust](https://www.rust-lang.org/tools/install) (MSRV: **1.70.0** or later) installed, then run:

```bash
cargo install twars-url2md
```

### From Source

Clone the repository and install locally:

```bash
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md
cargo install --path .
```

## Usage

The tool accepts URLs via a file or standard input and converts each page into a Markdown file. It also supports a base URL for resolving relative links.

### Input Options

- **`--input <FILE>`**
  Read URLs from a specified file (one URL per line)
- **`--stdin`**
  Read URLs from standard input
- **`--base_url <URL>`**
  Base URL for resolving relative links
- **Note:** Do not use both `--input` and `--stdin` simultaneously

### Output Options

- **`--output <DIR>`**
  Specify output directory for Markdown files
- If no output directory is specified, content is printed to stdout

### Output Organization

The tool organizes the output into a directory structure based on the URLs.

- Organizes files by host and path components
- URLs ending in `/` or with no path use `index.md`
- Other URLs use the last path component with `.md` extension

Example structure:

```
output/
├── example.com/
│   ├── index.md
│   └── articles/
│       └── page.md
└── another-site.com/
    └── post/
        └── article.md
```

### Examples

```bash
# Process a single URL and print to stdout
twars-url2md --stdin <<< "https://example.com"

# Process URLs from a file with specific output directory
twars-url2md --input urls.txt --output ./markdown_output

# Process piped URLs with base URL for relative links
cat urls.txt | twars-url2md --stdin --base_url "https://example.com" --output ./output

# Show verbose output (disabled by default)
twars-url2md --input urls.txt --output ./output --verbose
```

#### Batch work

```bash
# This scans the page for links, and downloads all 260+ pages linked from that page, in about 15 seconds
curl "https://en.wikipedia.org/wiki/Rust_(programming_language)" | twars-url2md --stdin --output work/

# This downloads 15k+ pages (links from all the files downloaded previously) in 8 minutes
cat $(fd -e md --search-path work) | twars-url2md --stdin --output work/
```

## Development and Testing

`twars-url2md` efficiently processes web content through several optimized steps. It starts by extracting valid _http(s)_ URLs using the `linkify` crate, filtering out malformed links from stdin, files, or command-line inputs.

For each URL, `twars-url2md`:

- Spawns an asynchronous task with `tokio`, scaling concurrent tasks to available CPU cores
- Uses `monolith` to fetch and clean HTML, removing scripts, styles, and media while preserving document structure
- Processes HTML with a custom `html5ever` parser that maintains document hierarchy and handles character encoding
- Converts content to Markdown via `htmd`, preserving headings, links, and basic formatting
- Implements an exponential backoff retry mechanism for failed requests
- Creates output directories based on URL domain and path using cross-platform `PathBuf`

The tool provides comprehensive error handling, catches potential panics, and generates meaningful error messages. It tracks progress for multiple URLs with `indicatif` and uses `rayon` for parallel processing of large HTML documents. It processes files in chunks and uses pre-allocated data structures with estimated capacities to achieve memory efficiency.

### Running Tests

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test test_name
```

### Code Quality Tools

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy --all-targets --all-features`
- **Pre-commit Hooks**: Runs formatting, clippy, and basic checks



## CI/CD and Release Process

GitHub Actions workflow includes:

- Automated testing on pull requests
- Code quality checks (clippy, fmt)
- Release creation for version tags
- Binary builds for multiple platforms
- Automatic crates.io publishing



## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

Adam Twardoch ([@twardoch](https://github.com/twardoch))

---

For bug reports, feature requests, or general questions, please open an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues).
