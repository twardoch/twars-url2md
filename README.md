# twars-url2md

[![Crates.io](https://img.shields.io/crates/v/twars-url2md)](https://crates.io/crates/twars-url2md)
![GitHub Release Date](https://img.shields.io/github/release-date/twardoch/twars-url2md)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/twardoch/twars-url2md/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**`twars-url2md`** is a fast and robust command-line tool written in Rust that fetches web pages, cleans up their HTML content, and converts them into clean Markdown.

You can drop a text that contains URLs onto the app, and it will find all the URLs and save Markdown versions of the pages in a logical folder structure. The output is not perfect, but the tool is fast and robust.

## 1. Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Development](#development)
- [License](#license)
- [Author](#author)

## 2. Features

### 2.1. Powerful Web Content Conversion

- Extracts clean web content using Monolith
- Converts web pages to Markdown efficiently
- Handles complex URL and encoding scenarios

### 2.2. Smart URL Handling

- Extracts URLs from various text formats
- Resolves and validates URLs intelligently
- Supports base URL and relative link processing
- **NEW**: Processes local HTML files in addition to remote URLs

### 2.3. Flexible Input & Output

- Multiple input methods (file, stdin, CLI)
- Organized Markdown file generation
- Cross-platform compatibility
- **NEW**: Option to pack all Markdown outputs into a single combined file

### 2.4. Advanced Processing

- Parallel URL processing
- Robust error handling
- Exponential backoff retry mechanism for network requests

## 3. Installation

### 3.1. Download Pre-compiled Binaries

The easiest way to get started is to download the pre-compiled binary for your platform.

1. Visit the [releases page](https://github.com/twardoch/twars-url2md/releases)
2. Download the appropriate file for your system:
   - **macOS**: `twars-url2md-macos-universal.tar.gz` (works on both Intel and Apple Silicon)
   - **Windows**: `twars-url2md-windows-x86_64.exe.zip`
   - **Linux**: `twars-url2md-linux-x86_64.tar.gz`
3. Extract the archive:
   - **macOS/Linux**: `tar -xzf twars-url2md-*.tar.gz`
   - **Windows**: Extract the zip file using Explorer or any archive utility
4. Make the binary executable (macOS/Linux only): `chmod +x twars-url2md`
5. Move the binary to a location in your PATH:
   - **macOS/Linux**: `sudo mv twars-url2md /usr/local/bin/` or `mv twars-url2md ~/.local/bin/`
   - **Windows**: Move to a folder in your PATH or add the folder to your PATH

### 3.2. Install from Crates.io

If you have Rust installed (version 1.70.0 or later), you can install directly from crates.io:

```bash
cargo install twars-url2md
```

### 3.3. Build from Source

For the latest version or to customize the build:

```bash
# Clone the repository
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md

# Build and install
cargo build --release
mv target/release/twars-url2md /usr/local/bin/  # or any location in your PATH
```

## 4. Usage

### 4.1. Command Line Options

```
Usage: twars-url2md [OPTIONS]

Options:
  -i, --input <FILE>       Input file containing URLs or local file paths (one per line)
  -o, --output <DIR>       Output directory for markdown files
      --stdin              Read URLs from standard input
      --base-url <URL>     Base URL for resolving relative links
  -p, --pack <FILE>        Output file to pack all markdown files together
  -v, --verbose            Enable verbose output
  -h, --help               Print help
  -V, --version            Print version
```

### 4.2. Input Options

The tool accepts URLs and local file paths from:

- A file specified with `--input`
- Standard input with `--stdin`
- **Note:** Either `--input` or `--stdin` must be specified

### 4.3. Output Options

- `--output <DIR>`: Create individual Markdown files in this directory
- `--pack <FILE>`: Combine all Markdown files into a single output file
- You can use both options together

### 4.4. Processing Local Files

You can now include local HTML files in your input:

- Absolute paths: `/path/to/file.html`
- File URLs: `file:///path/to/file.html`
- Mix of local files and remote URLs in the same input

## 5. Examples

### 5.1. Basic Usage

```bash
# Process a single URL and print to stdout
echo "https://example.com" | twars-url2md --stdin

# Process URLs from a file with specific output directory
twars-url2md --input urls.txt --output ./markdown_output

# Process piped URLs with base URL for relative links
cat urls.txt | twars-url2md --stdin --base-url "https://example.com" --output ./output

# Show verbose output
twars-url2md --input urls.txt --output ./output --verbose
```

### 5.2. Using the Pack Option

```bash
# Process URLs and create a combined Markdown file
twars-url2md --input urls.txt --pack combined.md

# Both individual files and a combined file
twars-url2md --input urls.txt --output ./output --pack combined.md
```

### 5.3. Processing Local Files

```bash
# Create a test HTML file
echo "<html><body><h1>Test</h1><p>Content</p></body></html>" > test.html

# Process a local HTML file
echo "$PWD/test.html" > local_paths.txt
twars-url2md --input local_paths.txt --output ./output

# Mix local and remote content
cat > mixed.txt << EOF
https://example.com
file://$PWD/test.html
EOF
twars-url2md --input mixed.txt --pack combined.md
```

### 5.4. Batch Processing

```bash
# Extract and process links from a webpage
curl "https://en.wikipedia.org/wiki/Rust_(programming_language)" | twars-url2md --stdin --output rust_wiki/

# Process multiple files
find ./html_files -name "*.html" > files_to_process.txt
twars-url2md --input files_to_process.txt --output ./markdown_output --pack all_content.md
```

## 6. Output Organization

The tool organizes output into a directory structure based on the URLs:

```
output/
├── example.com/
│   ├── index.md       # from https://example.com/
│   └── articles/
│       └── page.md    # from https://example.com/articles/page
└── another-site.com/
    └── post/
        └── article.md # from https://another-site.com/post/article
```

For local files, the directory structure mirrors the file path.

## 7. Development

### 7.1. Running Tests

```bash
# Run all tests
cargo test

# Run with specific features
cargo test --all-features

# Run specific test
cargo test test_name
```

### 7.2. Code Quality Tools

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy --all-targets --all-features`

### 7.3. Publishing

To publish a new release of twars-url2md:

#### 7.3.1. Prepare for Release

```bash
# Update version in Cargo.toml (e.g. from 1.3.6 to 1.3.7)
# Ensure everything works
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
```

#### 7.3.2. Build Locally

```bash
# Build in release mode
cargo build --release

# Test the binary
./target/release/twars-url2md --help
```

#### 7.3.3. Publish to Crates.io

```bash
# Login to crates.io (if not already logged in)
cargo login

# Verify the package
cargo package

# Publish
cargo publish
```

#### 7.3.4. Create GitHub Release

```bash
# Create and push a tag matching your version
git tag -a v1.3.7 -m "Release v1.3.7"
git push origin v1.3.7
```

The configured GitHub Actions workflow (`.github/workflows/ci.yml`) will automatically:
- Run tests on the tag
- Create a GitHub Release
- Build binaries for macOS, Windows, and Linux
- Upload the binaries to the release
- Publish to crates.io

#### 7.3.5. Manual Release (Alternative)

If GitHub Actions fails, you can create the release manually:

1. Go to GitHub repository → Releases → Create a new release
2. Select your tag
3. Build platform-specific binaries:

```bash
# macOS universal binary
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo "target/x86_64-apple-darwin/release/twars-url2md" "target/aarch64-apple-darwin/release/twars-url2md" -create -output "target/twars-url2md"
tar czf twars-url2md-macos-universal.tar.gz -C target twars-url2md

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
tar czf twars-url2md-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release twars-url2md

# Windows
cargo build --release --target x86_64-pc-windows-msvc
cd target/x86_64-pc-windows-msvc/release
7z a ../../../twars-url2md-windows-x86_64.zip twars-url2md.exe
```

4. Upload these files to your GitHub release

#### 7.3.6. Verify the Release

- Check that the release appears on GitHub
- Verify that binary files are attached to the release
- Confirm the new version appears on crates.io
- Try installing the new version: `cargo install twars-url2md`

## 8. License

MIT License - see [LICENSE](LICENSE) for details.

## 9. Author

Adam Twardoch ([@twardoch](https://github.com/twardoch))

---

For bug reports, feature requests, or general questions, please open an issue on the [GitHub repository](https://github.com/twardoch/twars-url2md/issues).
