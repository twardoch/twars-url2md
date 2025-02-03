# twars-url2md

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
![GitHub Release Date](https://img.shields.io/github/release-date/twardoch/twars-url2md)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/twardoch/twars-url2md/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**twars-url2md** is a robust command-line tool written in Rust that fetches web pages, cleans up their HTML content, and converts them into clean Markdown. It leverages [Monolith](https://github.com/Y2Z/monolith) for content extraction and [htmd](https://crates.io/crates/htmd) for the conversion process, ensuring that the resulting Markdown preserves the document's logical structure.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Input Options](#input-options)
  - [Output Organization](#output-organization)
  - [Examples](#examples)
- [Configuration & Retry Mechanism](#configuration--retry-mechanism)
- [Development and Testing](#development-and-testing)
- [CI/CD and Release Process](#cicd-and-release-process)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)

## Features

- **Powerful Content Extraction**
  - Uses Monolith to fetch and process web content
  - Strips unwanted assets (CSS, JavaScript, images, videos, fonts)
  - Preserves essential HTML structure
  - Handles proper character encoding detection

- **Smart URL Processing**
  - Extracts and validates URLs from plain text, HTML, and Markdown
  - Supports relative URL resolution with base URL
  - Filters out invalid URLs and duplicates
  - Handles special characters and complex URL structures

- **Flexible Input Options**
  - File input (one URL per line)
  - Standard input (pipe URLs)
  - Command-line arguments
  - Base URL specification for relative links

- **Robust Output Management**
  - Organized directory hierarchy based on URL structure
  - Smart filename generation (`index.md` for root/trailing slash)
  - Proper handling of special characters
  - Optional single file or directory-based output

- **Advanced Processing Features**
  - Parallel URL processing with progress indication
  - Exponential backoff retry mechanism
  - Comprehensive error reporting
  - Cross-platform compatibility

## Installation

### From Crates.io

Make sure you have [Rust](https://www.rust-lang.org/tools/install) (MSRV: **1.70.0** or later) installed, then run:

```bash
cargo install twars-url2md
```

### From Binary Releases

Pre-built binaries are available for:

- Linux (x86_64)
- macOS (Universal binary for Intel and Apple Silicon)
- Windows (x86_64)

Download from the [Releases page](https://github.com/twardoch/twars-url2md/releases).

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

For URLs like `scheme://username:password@host:port/path?query#fragment`:

- Username, password, port, query parameters, and fragments are ignored
- Files are organized by host and path components
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

# Show verbose output (enabled by default)
twars-url2md --input urls.txt --output ./output
```

## Configuration & Retry Mechanism

- **Parallel Processing**: Uses tokio for concurrent URL processing
- **Progress Tracking**: Displays progress bar for multiple URLs
- **Retry Logic**:
  - Up to 2 retries per URL
  - Exponential backoff between attempts
  - Detailed error reporting for failed URLs
- **Verbose Mode**: Enabled by default for processing information

## Development and Testing

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

### Dependencies

Key crates used:

- `monolith`: Web content extraction
- `htmd`: HTML to Markdown conversion
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `linkify`: URL detection
- `clap`: CLI argument parsing
- `indicatif`: Progress bars
- `html5ever`: HTML parsing

## CI/CD and Release Process

GitHub Actions workflow includes:

- Automated testing on pull requests
- Code quality checks (clippy, fmt)
- Release creation for version tags
- Binary builds for multiple platforms
- Automatic crates.io publishing

## Contributing

1. Fork the repository
2. Create a feature branch
3. Install pre-commit hooks: `pre-commit install`
4. Make your changes
5. Ensure tests pass: `cargo test`
6. Submit a pull request

Please follow:

- Rust coding conventions
- Comprehensive test coverage
- Clear commit messages
- Documentation updates

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

Adam Twardoch ([@twardoch](https://github.com/twardoch))

---

For bug reports, feature requests, or general questions, please open an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues).
