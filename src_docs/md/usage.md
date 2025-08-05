# Basic Usage

This comprehensive guide covers all command-line options, input formats, output modes, and common usage patterns for `twars-url2md`.

## Command Syntax

```bash
twars-url2md [OPTIONS] [URLS...]
```

## Command-Line Options

### Input Options

| Option | Description | Example |
|--------|-------------|---------|
| `[URLS...]` | URLs as command arguments | `twars-url2md https://example.com https://other.com` |
| `-i, --input <FILE>` | Read URLs from file (one per line or extractable text) | `-i urls.txt` |
| `--stdin` | Read URLs from standard input | `echo "https://example.com" \| twars-url2md --stdin` |
| `--base-url <URL>` | Base URL for resolving relative links | `--base-url https://example.com` |

### Output Options

| Option | Description | Example |
|--------|-------------|---------|
| `-o, --output <PATH>` | Output directory or file path | `-o ./markdown_files` or `-o output.md` |
| `-p, --pack <FILE>` | Pack all content into single file with URL headers | `--pack combined.md` |

### Control Options

| Option | Description | Default |
|--------|-------------|---------|
| `-v, --verbose` | Enable verbose logging (INFO/DEBUG levels) | Disabled |
| `-h, --help` | Show help information | - |
| `-V, --version` | Show version and build information | - |

## Input Formats

### Direct URL Arguments

```bash
# Single URL
twars-url2md https://www.rust-lang.org

# Multiple URLs
twars-url2md https://www.rust-lang.org https://crates.io https://doc.rust-lang.org
```

### File Input (`-i, --input`)

Create a file with URLs (one per line):

```txt
https://www.rust-lang.org
https://crates.io
https://doc.rust-lang.org/book/
# Comments are ignored
https://github.com/rust-lang/rust
```

```bash
twars-url2md -i urls.txt -o output/
```

### Standard Input (`--stdin`)

```bash
# From pipeline
echo "https://example.com" | twars-url2md --stdin -o output/

# From here-document
twars-url2md --stdin -o output/ << EOF
https://www.rust-lang.org
https://crates.io
EOF

# From another command
curl -s https://awesome-rust.com | twars-url2md --stdin --base-url https://awesome-rust.com -o awesome/
```

### URL Extraction from Text

`twars-url2md` can extract URLs from various text formats:

=== "HTML Content"
    ```html
    <!-- Input HTML content -->
    <p>Check out <a href="https://www.rust-lang.org">Rust</a> and 
    <a href="/crates">crates.io</a> for packages.</p>
    ```
    
    ```bash
    # Extract and process URLs
    cat content.html | twars-url2md --stdin --base-url https://example.com -o extracted/
    ```

=== "Markdown Content"
    ```markdown
    # Links in Markdown
    Check out [Rust](https://www.rust-lang.org) and [Crates](https://crates.io).
    Also see [relative link](./docs/guide.html).
    ```
    
    ```bash
    twars-url2md -i content.md --base-url https://site.com -o docs/
    ```

=== "Plain Text"
    ```txt
    Visit https://www.rust-lang.org for documentation.
    The package registry is at https://crates.io.
    Relative URLs like /docs/guide need a base URL.
    ```

### Local File Processing

Process local HTML files:

```bash
# Absolute path
twars-url2md /home/user/document.html -o output/

# Relative path  
twars-url2md ./local/file.html -o converted/

# File URL format
twars-url2md file:///absolute/path/to/document.html -o output/

# Multiple local files via find
find ./html_docs -name "*.html" | twars-url2md --stdin -o converted_docs/
```

## Output Modes

### Directory Structure (Default)

Creates hierarchical directory structure mirroring URL paths:

```bash
twars-url2md https://doc.rust-lang.org/book/ch01-01-installation.html -o output/
```

**Result**: `output/doc.rust-lang.org/book/ch01-01-installation.md`

### Single File Output

When output path ends with `.md`, content is saved to that specific file:

```bash
# Single URL to single file
twars-url2md https://www.rust-lang.org -o rust-homepage.md

# Multiple URLs concatenated  
twars-url2md https://rust-lang.org https://crates.io -o combined.md
```

### Packed Output (`--pack`)

Combines multiple URLs into one file with clear delimiters:

```bash
twars-url2md -i urls.txt --pack documentation.md
```

**Packed output format**:
```markdown
# https://www.rust-lang.org

[Content from rust-lang.org homepage]

# https://crates.io  

[Content from crates.io homepage]

# https://doc.rust-lang.org/book/

[Content from Rust book]
```

### Combined Output Modes

Use both directory structure and packed output:

```bash
twars-url2md -i urls.txt -o individual_files/ --pack combined_archive.md
```

This creates:
- Individual files in `individual_files/` directory  
- Combined content in `combined_archive.md`

## Verbose Logging

Enable detailed logging with `-v` or `--verbose`:

```bash
twars-url2md -i urls.txt -o output/ -v
```

**Verbose output includes**:
- URL processing progress
- HTTP request details
- HTML cleaning steps  
- File writing operations
- Error details and retry attempts

### Advanced Logging Control

Use `RUST_LOG` environment variable for fine-grained control:

```bash
# Debug level for twars-url2md only
RUST_LOG=twars_url2md=debug twars-url2md -i urls.txt -o output/

# Trace level (very detailed)
RUST_LOG=twars_url2md=trace twars-url2md -i urls.txt -o output/

# Info level for all components
RUST_LOG=info twars-url2md -i urls.txt -o output/
```

## Error Handling

### Retry Behavior

`twars-url2md` automatically retries failed requests:

- **Default**: 2 additional retries (3 total attempts)
- **Backoff**: Exponential backoff between retries
- **Errors**: Network timeouts, DNS failures, HTTP 5xx errors

### Graceful Failure

- Individual URL failures don't stop batch processing
- Failed URLs are logged but processing continues
- Final exit code reflects overall success/failure

### Common Error Scenarios

=== "Network Issues"
    ```bash
    # Verbose mode shows retry attempts
    twars-url2md https://unreliable-site.com -v
    ```
    
    **Output**:
    ```
    WARN Failed to fetch https://unreliable-site.com: Connection timeout
    INFO Retrying in 1s... (attempt 2/3)
    INFO Successfully fetched after retry
    ```

=== "SSL/TLS Problems"
    ```bash
    twars-url2md https://site-with-ssl-issues.com -v
    ```
    
    **Troubleshooting**:
    - Update system CA certificates
    - Check if site uses self-signed certificates
    - Verify system date/time is correct

=== "HTML Parsing Issues"
    ```bash
    # Monolith panic recovery
    twars-url2md https://site-with-malformed-html.com -v
    ```
    
    If HTML cleaning fails, the tool attempts fallback processing.

## Advanced Usage Patterns

### Pipeline Integration

```bash
# Extract links from webpage and process them
curl -s https://awesome-rust.com | \
  grep -o 'https://[^"]*' | \
  head -20 | \
  twars-url2md --stdin -o awesome_projects/
```

### Batch Processing with Filtering

```bash
# Process only certain domains
grep 'rust-lang.org' all_urls.txt | \
  twars-url2md --stdin -o rust_docs/

# Skip already processed URLs
comm -23 <(sort all_urls.txt) <(sort processed_urls.txt) | \
  twars-url2md --stdin -o remaining/
```

### Monitoring Progress

```bash
# Large job with progress monitoring
wc -l big_url_list.txt  # Check total count
twars-url2md -i big_url_list.txt -o output/ -v | tee conversion.log
```

### Parallel Processing

For very large URL lists, split into chunks:

```bash
# Split large file
split -l 50 huge_urls.txt chunk_

# Process chunks in parallel
for chunk in chunk_*; do
  twars-url2md -i "$chunk" -o "output_${chunk}/" -v &
done
wait

# Combine results if needed
find output_chunk_* -name "*.md" -exec cp {} final_output/ \;
```

## Integration Examples

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Convert documentation URLs
  run: |
    echo "${{ vars.DOC_URLS }}" | \
    twars-url2md --stdin -o docs/ --pack complete-docs.md -v
```

### Cron Job

```bash
#!/bin/bash
# daily-news-archive.sh
DATE=$(date +%Y-%m-%d)
curl -s "https://news-api.com/today" | \
  jq -r '.articles[].url' | \
  twars-url2md --stdin --pack "news-${DATE}.md" -v
```

### Build System Integration

```makefile
# Makefile example
docs: urls.txt
	twars-url2md -i urls.txt -o docs/ --pack docs-complete.md -v
	
.PHONY: docs
```

## Performance Considerations

### Concurrency

- **Automatic**: CPU core detection for optimal concurrency
- **Typical**: 2x CPU cores for I/O bound operations
- **Maximum**: Capped at reasonable limits to avoid overwhelming servers

### Memory Usage

- **Streaming**: Minimal memory footprint per URL
- **Batch Size**: Automatically managed based on available resources
- **Large Files**: Handled efficiently without loading entire content into memory

### Network Behavior

- **HTTP/2**: Preferred when available for better CDN compatibility
- **Connection Reuse**: Efficient connection pooling
- **Rate Limiting**: Respectful of server resources

## Troubleshooting

### Common Issues

=== "Permission Denied"
    ```bash
    # Create output directory first
    mkdir -p output
    twars-url2md https://example.com -o output/
    
    # Or use user directory
    twars-url2md https://example.com -o ~/Documents/markdown/
    ```

=== "Empty Output Files"
    ```bash
    # Enable verbose mode to diagnose
    twars-url2md https://problematic-url.com -v
    
    # Check if content is JavaScript-rendered
    # (not supported - requires pre-rendered HTML)
    ```

=== "Encoding Issues"
    ```bash
    # Modern sites should work automatically
    # Check verbose output for encoding detection
    twars-url2md https://non-english-site.com -v
    ```

### Debug Mode

For development or troubleshooting:

```bash
# Maximum verbosity
RUST_LOG=trace twars-url2md https://example.com -v

# Focus on specific components  
RUST_LOG=twars_url2md::html=debug twars-url2md https://example.com -v
```

---

!!! tip "Best Practices"
    - Always use `-v` for large batch jobs to monitor progress
    - Test with a small set of URLs before processing large lists
    - Use `--pack` for creating single-file archives
    - Monitor system resources during large conversion jobs
    - Keep URL lists organized and documented for reproducibility