# Quick Start

Get up and running with `twars-url2md` in minutes. This guide covers the most common use cases with practical examples.

## Basic Conversion

### Single URL

Convert one URL to Markdown:

```bash
# Convert to stdout (preview)
twars-url2md https://www.rust-lang.org

# Save to a directory structure
twars-url2md https://www.rust-lang.org -o output/

# Save to a specific file
twars-url2md https://www.rust-lang.org -o rust-lang.md
```

### Multiple URLs

```bash
# Multiple URLs as arguments
twars-url2md https://www.rust-lang.org https://crates.io -o output/

# From a file (one URL per line)
echo -e "https://www.rust-lang.org\nhttps://crates.io" > urls.txt
twars-url2md -i urls.txt -o output/
```

## Input Methods

### From Files

Create a file with URLs:

```bash
# Create URL list
cat > my_urls.txt << EOF
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/std/
https://doc.rust-lang.org/cargo/
EOF

# Process the file
twars-url2md -i my_urls.txt -o rust_docs/
```

### From Standard Input

```bash
# Pipe URLs from another command
curl -s https://example.com/links.html | \
  twars-url2md --stdin --base-url https://example.com -o extracted/

# Manual input (Ctrl+D to finish)
twars-url2md --stdin -o manual_input/
```

### From HTML/Markdown Content

`twars-url2md` can extract URLs from HTML or Markdown content:

```bash
# Extract links from a webpage
curl -s https://awesome-rust.com | \
  twars-url2md --stdin --base-url https://awesome-rust.com -o awesome_rust/

# Process a markdown file with links
twars-url2md -i README.md --base-url https://github.com/user/repo -o docs/
```

## Output Options

### Directory Structure (Default)

Creates organized directory structure:

```bash
twars-url2md https://doc.rust-lang.org/book/ch01-01-installation.html -o output/
```

Creates: `output/doc.rust-lang.org/book/ch01-01-installation.md`

### Single File Output

```bash
# Specify .md extension
twars-url2md https://doc.rust-lang.org/book/ -o rust-book.md
```

### Packed Output

Combine multiple URLs into one file with headers:

```bash
# Pack multiple URLs
twars-url2md -i urls.txt --pack combined-docs.md

# Pack with individual files too
twars-url2md -i urls.txt -o individual/ --pack combined.md
```

Example packed output:
```markdown
# https://www.rust-lang.org

Content from rust-lang.org...

# https://crates.io

Content from crates.io...
```

## Common Workflows

### Documentation Archiving

```bash
# Archive a project's documentation
cat > project_docs.txt << EOF
https://project.example.com/docs/
https://project.example.com/api/
https://project.example.com/tutorials/
EOF

twars-url2md -i project_docs.txt -o project_archive/ --pack project_complete.md -v
```

### Research Collection

```bash
# Collect research papers
twars-url2md \
  https://arxiv.org/abs/2301.12345 \
  https://papers.nips.cc/paper/2023/hash/abcd1234 \
  --pack research_collection.md \
  -v
```

### Blog Post Conversion

```bash
# Convert blog posts to markdown
find blog_urls.txt -type f | \
  twars-url2md --stdin -o blog_archive/ \
  --verbose
```

## Local File Processing

Process local HTML files:

```bash
# Single local file
twars-url2md /path/to/document.html -o converted/

# Multiple local files
find . -name "*.html" | twars-url2md --stdin -o html_converted/

# With file:// URLs
twars-url2md file:///absolute/path/to/document.html -o output/
```

## Useful Options

### Verbose Mode

See detailed processing information:

```bash
twars-url2md -i urls.txt -o output/ -v
```

### Custom Base URL

Resolve relative links in extracted content:

```bash
curl -s https://news.ycombinator.com | \
  twars-url2md --stdin --base-url https://news.ycombinator.com -o hn_links/
```

## Quick Examples by Use Case

=== "Website Backup"
    ```bash
    # Backup important pages
    cat > backup_urls.txt << EOF
    https://company.com/important-doc
    https://company.com/api-reference  
    https://company.com/user-guide
    EOF
    
    twars-url2md -i backup_urls.txt -o backup/ --pack complete-backup.md -v
    ```

=== "Research Collection"
    ```bash
    # Academic papers and articles
    twars-url2md \
      "https://arxiv.org/abs/2301.07041" \
      "https://openreview.net/forum?id=abc123" \
      --pack research-$(date +%Y%m%d).md
    ```

=== "Tutorial Archive"
    ```bash
    # Save programming tutorials
    echo "https://doc.rust-lang.org/book/" | \
      twars-url2md --stdin -o tutorials/ -v
    ```

=== "News Articles"
    ```bash
    # Daily news collection
    cat today_articles.txt | \
      twars-url2md --stdin --pack "news-$(date +%Y-%m-%d).md"
    ```

## Performance Tips

### Batch Processing

For large numbers of URLs:

```bash
# Enable verbose logging to monitor progress
twars-url2md -i large_url_list.txt -o output/ -v

# Split large files for parallel processing
split -l 100 huge_urls.txt chunk_
for chunk in chunk_*; do
  twars-url2md -i "$chunk" -o "output_${chunk}/" &
done
wait
```

### Resource Management

```bash
# Monitor system resources during large jobs
htop &  # or your preferred system monitor
twars-url2md -i many_urls.txt -o output/ -v
```

## Troubleshooting Quick Fixes

### Common Issues

```bash
# SSL certificate issues
twars-url2md https://site-with-ssl-issues.com -v  # Check verbose output

# Timeout issues
twars-url2md https://slow-site.com -v  # Monitor with verbose logging

# Permission issues
mkdir -p ~/my_output
twars-url2md https://example.com -o ~/my_output/
```

### Verification

```bash
# Test installation
twars-url2md --version

# Test with a reliable URL
twars-url2md https://httpbin.org/html -o test/

# Verify output
ls -la test/
cat test/httpbin.org/html.md
```

## Next Steps

Now that you're familiar with basic usage:

- Learn about [Advanced Features](advanced.md) for complex workflows  
- Review [Configuration](configuration.md) for customization options
- Check [Usage](usage.md) for comprehensive command reference
- Explore [API Reference](api.md) for library integration

---

!!! tip "Pro Tips"
    - Use `-v` (verbose) to monitor progress on large jobs
    - The `--pack` option is great for creating single-file archives
    - Directory structure output mirrors the original URL hierarchy
    - Local HTML files can be processed just like remote URLs