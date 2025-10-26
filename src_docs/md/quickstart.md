# Quick Start

Get `twars-url2md` running fast. This guide shows common tasks with real examples.

## Basic Conversion

### Single URL

Turn one URL into Markdown:

```bash
# Print to stdout
twars-url2md https://www.rust-lang.org

# Save to a directory
twars-url2md https://www.rust-lang.org -o output/

# Save to a file
twars-url2md https://www.rust-lang.org -o rust-lang.md
```

### Multiple URLs

```bash
# List URLs as arguments
twars-url2md https://www.rust-lang.org https://crates.io -o output/

# From a file (one URL per line)
echo -e "https://www.rust-lang.org\nhttps://crates.io" > urls.txt
twars-url2md -i urls.txt -o output/
```

## Input Methods

### From Files

Make a list of URLs:

```bash
# Create the list
cat > my_urls.txt << EOF
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/std/
https://doc.rust-lang.org/cargo/
EOF

# Run conversion
twars-url2md -i my_urls.txt -o rust_docs/
```

### From Standard Input

```bash
# Pipe from another command
curl -s https://example.com/links.html | \
  twars-url2md --stdin --base-url https://example.com -o extracted/

# Manual input (press Ctrl+D when done)
twars-url2md --stdin -o manual_input/
```

### From HTML/Markdown Content

Pull URLs from content:

```bash
# Get links from a webpage
curl -s https://awesome-rust.com | \
  twars-url2md --stdin --base-url https://awesome-rust.com -o awesome_rust/

# Pull links from a markdown file
twars-url2md -i README.md --base-url https://github.com/user/repo -o docs/
```

## Output Options

### Directory Structure (Default)

Organizes output by domain and path:

```bash
twars-url2md https://doc.rust-lang.org/book/ch01-01-installation.html -o output/
```

Result: `output/doc.rust-lang.org/book/ch01-01-installation.md`

### Single File Output

```bash
# Force single file with .md extension
twars-url2md https://doc.rust-lang.org/book/ -o rust-book.md
```

### Packed Output

Merge multiple URLs into one file:

```bash
# Pack all into one file
twars-url2md -i urls.txt --pack combined-docs.md

# Save individual files AND pack
twars-url2md -i urls.txt -o individual/ --pack combined.md
```

Packed format:
```markdown
# https://www.rust-lang.org

Content from rust-lang.org...

# https://crates.io

Content from crates.io...
```

## Common Workflows

### Documentation Archiving

```bash
# Archive project docs
cat > project_docs.txt << EOF
https://project.example.com/docs/
https://project.example.com/api/
https://project.example.com/tutorials/
EOF

twars-url2md -i project_docs.txt -o project_archive/ --pack project_complete.md -v
```

### Research Collection

```bash
# Collect papers
twars-url2md \
  https://arxiv.org/abs/2301.12345 \
  https://papers.nips.cc/paper/2023/hash/abcd1234 \
  --pack research_collection.md \
  -v
```

### Blog Post Conversion

```bash
# Convert blog posts
find blog_urls.txt -type f | \
  twars-url2md --stdin -o blog_archive/ \
  --verbose
```

## Local File Processing

Handle local HTML files:

```bash
# One file
twars-url2md /path/to/document.html -o converted/

# Many files
find . -name "*.html" | twars-url2md --stdin -o html_converted/

# With file:// protocol
twars-url2md file:///absolute/path/to/document.html -o output/
```

## Useful Options

### Verbose Mode

Show what's happening:

```bash
twars-url2md -i urls.txt -o output/ -v
```

### Custom Base URL

Fix relative links in content:

```bash
curl -s https://news.ycombinator.com | \
  twars-url2md --stdin --base-url https://news.ycombinator.com -o hn_links/
```

## Quick Examples by Use Case

=== "Website Backup"
    ```bash
    # Backup pages
    cat > backup_urls.txt << EOF
    https://company.com/important-doc
    https://company.com/api-reference  
    https://company.com/user-guide
    EOF
    
    twars-url2md -i backup_urls.txt -o backup/ --pack complete-backup.md -v
    ```

=== "Research Collection"
    ```bash
    # Save papers
    twars-url2md \
      "https://arxiv.org/abs/2301.07041" \
      "https://openreview.net/forum?id=abc123" \
      --pack research-$(date +%Y%m%d).md
    ```

=== "Tutorial Archive"
    ```bash
    # Save tutorials
    echo "https://doc.rust-lang.org/book/" | \
      twars-url2md --stdin -o tutorials/ -v
    ```

=== "News Articles"
    ```bash
    # Save daily articles
    cat today_articles.txt | \
      twars-url2md --stdin --pack "news-$(date +%Y-%m-%d).md"
    ```

## Performance Tips

### Batch Processing

Speed up large jobs:

```bash
# Watch progress
twars-url2md -i large_url_list.txt -o output/ -v

# Split and run in parallel
split -l 100 huge_urls.txt chunk_
for chunk in chunk_*; do
  twars-url2md -i "$chunk" -o "output_${chunk}/" &
done
wait
```

### Resource Management

Keep an eye on system load:

```bash
# Monitor while running
htop &  
twars-url2md -i many_urls.txt -o output/ -v
```

## Troubleshooting

### Common Fixes

```bash
# SSL problems
twars-url2md https://site-with-ssl-issues.com -v

# Slow sites
twars-url2md https://slow-site.com -v

# Permission errors
mkdir -p ~/my_output
twars-url2md https://example.com -o ~/my_output/
```

### Verification

```bash
# Check version
twars-url2md --version

# Test with known good URL
twars-url2md https://httpbin.org/html -o test/

# Confirm result
ls -la test/
cat test/httpbin.org/html.md
```

## Next Steps

- [Advanced Features](advanced.md)  
- [Configuration](configuration.md)  
- [Usage](usage.md)  
- [API Reference](api.md)  

---

!!! tip ""
    - Use `-v` for progress on big jobs
    - `--pack` merges content into one file
    - Default output mirrors URL structure
    - Local files work like remote ones