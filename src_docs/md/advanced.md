# Advanced Features

Explore powerful features and sophisticated workflows that make `twars-url2md` suitable for complex content processing scenarios.

## URL Extraction and Processing

### Smart URL Detection

`twars-url2md` uses intelligent URL extraction to find links in various content formats:

```bash
# Extract from HTML page with mixed content
curl -s https://news.ycombinator.com | \
  twars-url2md --stdin --base-url https://news.ycombinator.com -o hn_links/
```

**Supports**:
- HTML `<a href="...">` tags
- Markdown `[text](url)` links  
- Plain text URLs (http/https)
- Relative URLs with base URL resolution
- Multiple URL formats in single input

### Base URL Resolution

Resolve relative URLs found in content:

```bash
# HTML content with relative links
cat << 'EOF' > content.html
<a href="/docs/guide.html">Guide</a>
<a href="../api/reference.html">API</a>
<a href="https://external.com/page">External</a>  
EOF

twars-url2md -i content.html --base-url https://mysite.com/current/page -o resolved/
```

**Result**: Relative URLs become:
- `/docs/guide.html` → `https://mysite.com/docs/guide.html`
- `../api/reference.html` → `https://mysite.com/api/reference.html`
- External URLs remain unchanged

### URL Filtering and Validation

Built-in URL validation and filtering:

```bash
# Only HTTP/HTTPS URLs are processed
cat << 'EOF' > mixed_urls.txt
https://valid-site.com
http://another-site.com  
ftp://ignored-ftp-site.com
mailto:ignored@email.com
https://valid-secure-site.com
EOF

twars-url2md -i mixed_urls.txt -o filtered/ -v
```

**Automatic filtering**:
- ✅ `http://` and `https://` URLs
- ❌ `ftp://`, `mailto:`, `file://` (except local files)
- ❌ Invalid or malformed URLs
- ❌ Duplicate URLs (automatic deduplication)

## Advanced Output Options

### Packed Output with Custom Headers

The `--pack` option creates structured single-file output:

```bash
twars-url2md -i research_papers.txt --pack "research-$(date +%Y-%m-%d).md" -v
```

**Packed format features**:
- URL as primary heading (`# https://example.com`)
- Preserves original URL order
- Clear content separation
- Maintains markdown structure within each section

### Hybrid Output Mode

Combine individual files with packed output:

```bash
# Creates both directory structure AND packed file
twars-url2md -i urls.txt -o individual_files/ --pack archive.md -v
```

**Use cases**:
- Individual files for browsing/editing
- Packed file for searching/archiving
- Different output formats for different workflows

### Output Path Generation

Understanding output path creation:

```bash
# URL: https://doc.rust-lang.org/book/ch01-01-installation.html
twars-url2md "https://doc.rust-lang.org/book/ch01-01-installation.html" -o output/
```

**Generated path**: `output/doc.rust-lang.org/book/ch01-01-installation.md`

**Path generation rules**:
1. Domain becomes top-level directory
2. Path segments become subdirectories
3. Final segment becomes filename with `.md` extension
4. Query parameters and fragments are normalized/removed

## Local File Processing

### HTML File Conversion

Process local HTML files with full feature support:

```bash
# Single local file
twars-url2md /path/to/document.html -o converted/

# Multiple files with find
find ./html_docs -name "*.html" -print0 | \
  xargs -0 -I {} twars-url2md {} -o converted/

# Using file:// URLs
twars-url2md file:///absolute/path/to/doc.html -o output/
```

### Batch Local Processing

```bash
# Process entire directory structure  
find ./website_backup -type f -name "*.html" | \
  twars-url2md --stdin -o markdown_site/ -v

# Preserve directory structure in output
for html_file in $(find ./src -name "*.html"); do
  rel_path="${html_file#./src/}"
  output_path="./md/${rel_path%.html}.md"
  mkdir -p "$(dirname "$output_path")"
  twars-url2md "$html_file" -o "$output_path"
done
```

## Content Processing Pipeline

### Multi-stage Processing

Complex workflows combining extraction and conversion:

```bash
#!/bin/bash
# Advanced content pipeline

# Stage 1: Extract URLs from multiple sources
{
  curl -s https://awesome-rust.com | grep -o 'https://[^"]*github.com[^"]*'
  curl -s https://crates.io/categories | grep -o 'https://[^"]*crates.io[^"]*'
  cat manually_curated_urls.txt
} | sort -u > all_urls.txt

# Stage 2: Filter and categorize
grep 'github.com' all_urls.txt > github_repos.txt
grep 'crates.io' all_urls.txt > crate_pages.txt  
grep -v -E '(github\.com|crates\.io)' all_urls.txt > other_sites.txt

# Stage 3: Process each category
twars-url2md -i github_repos.txt -o repos/ --pack github-projects.md -v &
twars-url2md -i crate_pages.txt -o crates/ --pack rust-crates.md -v &
twars-url2md -i other_sites.txt -o misc/ --pack other-resources.md -v &

wait

# Stage 4: Create master index
cat > complete-rust-resources.md << 'EOF'
# Complete Rust Resources Archive

## GitHub Projects
EOF
cat github-projects.md >> complete-rust-resources.md

echo -e "\n## Rust Crates" >> complete-rust-resources.md
cat rust-crates.md >> complete-rust-resources.md

echo -e "\n## Other Resources" >> complete-rust-resources.md
cat other-resources.md >> complete-rust-resources.md
```

### Content Filtering and Processing

```bash
# Extract specific domains only
curl -s https://link-aggregator.com | \
  grep -o 'https://[^"]*' | \
  grep -E '\.(edu|gov|org)/' | \
  twars-url2md --stdin -o academic_content/ -v

# Process recent articles only (if timestamps available)
awk -F',' '$2 > "'$(date -d '30 days ago' +%s)'" {print $1}' timestamped_urls.csv | \
  twars-url2md --stdin -o recent_articles/ --pack recent.md
```

## Integration Patterns

### CI/CD Integration

#### GitHub Actions Workflow

```yaml
name: Documentation Sync
on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM
  workflow_dispatch:

jobs:
  sync-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install twars-url2md
        run: |
          curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash
          echo "$HOME/.local/bin" >> $GITHUB_PATH
      
      - name: Process documentation URLs
        run: |
          twars-url2md -i .github/doc_urls.txt -o docs/ --pack docs-complete.md -v
      
      - name: Commit updates
        run: |
          git config --local user.email "action@github.com"  
          git config --local user.name "GitHub Action"
          git add docs/
          git diff --staged --quiet || git commit -m "Update documentation $(date +%Y-%m-%d)"
          git push
```

#### Jenkins Pipeline

```groovy
pipeline {
    agent any
    triggers {
        cron('H 2 * * *')
    }
    stages {
        stage('Install Tools') {
            steps {
                sh '''
                    curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash
                    export PATH="$HOME/.local/bin:$PATH"
                '''
            }
        }
        stage('Convert Documentation') {
            steps {
                sh '''
                    export PATH="$HOME/.local/bin:$PATH"
                    twars-url2md -i urls.txt -o output/ --pack archive.md -v
                '''
            }
        }
        stage('Archive Results') {
            steps {
                archiveArtifacts artifacts: 'output/**/*.md, archive.md'
            }
        }
    }
}
```

### API Integration

#### REST API Processing

```bash
#!/bin/bash
# Process URLs from REST API

# Fetch URLs from API
curl -H "Authorization: Bearer $API_TOKEN" \
     https://api.example.com/urls | \
     jq -r '.urls[]' > api_urls.txt

# Process with error handling
if twars-url2md -i api_urls.txt -o api_content/ --pack "api-content-$(date +%Y%m%d).md" -v; then
    echo "Success: $(wc -l < api_urls.txt) URLs processed"
    # Upload results
    aws s3 cp api-content-*.md s3://my-bucket/archives/
else
    echo "Error: Processing failed"
    exit 1
fi
```

#### Webhook Integration

```python
#!/usr/bin/env python3
# Webhook handler for URL processing

from flask import Flask, request, jsonify
import subprocess
import tempfile
import os

app = Flask(__name__)

@app.route('/process-urls', methods=['POST'])
def process_urls():
    try:
        urls = request.json.get('urls', [])
        
        # Create temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            for url in urls:
                f.write(f"{url}\n")
            temp_file = f.name
        
        # Process URLs
        result = subprocess.run([
            'twars-url2md', '-i', temp_file, 
            '-o', 'webhook_output/', 
            '--pack', f'webhook-{int(time.time())}.md',
            '-v'
        ], capture_output=True, text=True)
        
        # Cleanup
        os.unlink(temp_file)
        
        if result.returncode == 0:
            return jsonify({'status': 'success', 'message': 'URLs processed'})
        else:
            return jsonify({'status': 'error', 'message': result.stderr}), 500
            
    except Exception as e:
        return jsonify({'status': 'error', 'message': str(e)}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
```

## Performance Optimization

### Parallel Processing Strategies

For very large URL sets:

```bash
# Split large URL list into chunks
split -l 100 huge_urls.txt chunk_

# Process chunks in parallel with resource limits
MAX_JOBS=4
job_count=0

for chunk in chunk_*; do
    if ((job_count >= MAX_JOBS)); then
        wait -n  # Wait for any job to finish
        ((job_count--))
    fi
    
    twars-url2md -i "$chunk" -o "output_${chunk}/" -v &
    ((job_count++))
done

wait  # Wait for all remaining jobs

# Combine results
find output_chunk_* -name "*.md" -exec cp {} final_output/ \;
```

### Resource Management

```bash
# Monitor resource usage during processing
{
    echo "Starting URL processing at $(date)"
    echo "System resources before:"
    free -h
    df -h
    
    # Process URLs with monitoring
    twars-url2md -i large_url_list.txt -o output/ -v &
    PID=$!
    
    # Monitor during processing
    while kill -0 $PID 2>/dev/null; do
        echo "$(date): Memory usage:"
        ps -p $PID -o pid,vsz,rss,pcpu
        sleep 30
    done
    
    wait $PID
    EXIT_CODE=$?
    
    echo "Processing completed with exit code: $EXIT_CODE"
    echo "Final system resources:"
    free -h
    df -h
} | tee processing.log
```

### Optimization Tips

=== "Network Optimization"
    ```bash
    # For sites behind CDNs, verbose mode helps debug
    twars-url2md https://cdn-protected-site.com -v
    
    # Batch similar domains together for connection reuse
    grep 'github.com' urls.txt | twars-url2md --stdin -o github/ -v
    grep 'docs.rs' urls.txt | twars-url2md --stdin -o rust_docs/ -v
    ```

=== "Storage Optimization"  
    ```bash
    # Use packed output for space efficiency
    twars-url2md -i urls.txt --pack compressed-archive.md
    
    # Clean up intermediate files
    twars-url2md -i urls.txt -o temp/ --pack final.md && rm -rf temp/
    ```

=== "Processing Optimization"
    ```bash
    # Pre-filter URLs to reduce processing time
    grep -E '\.(html|htm|php)(\?|$)' urls.txt | \
      twars-url2md --stdin -o filtered_output/ -v
    ```

## Error Handling and Recovery

### Robust Error Handling

```bash
#!/bin/bash
# Robust processing with error recovery

set -euo pipefail

# Function to process URLs with retry
process_with_retry() {
    local input_file="$1"
    local output_dir="$2"
    local max_attempts=3
    local attempt=1
    
    while ((attempt <= max_attempts)); do
        echo "Attempt $attempt of $max_attempts"
        
        if twars-url2md -i "$input_file" -o "$output_dir" -v; then
            echo "Success on attempt $attempt"
            return 0
        else
            echo "Failed attempt $attempt"
            if ((attempt < max_attempts)); then
                echo "Retrying in $((attempt * 10)) seconds..."
                sleep $((attempt * 10))
            fi
            ((attempt++))
        fi
    done
    
    echo "All attempts failed"
    return 1
}

# Main processing
if ! process_with_retry "urls.txt" "output/"; then
    # Fallback: process individual URLs to identify problems
    echo "Batch processing failed, trying individual URLs..."
    
    while IFS= read -r url; do
        if ! twars-url2md "$url" -o "individual_output/" -v; then
            echo "Failed URL: $url" >> failed_urls.txt
        fi
    done < urls.txt
    
    echo "Check failed_urls.txt for problematic URLs"
fi
```

### Logging and Monitoring

```bash
# Comprehensive logging setup
export RUST_LOG=info,twars_url2md=debug

# Process with full logging
twars-url2md -i urls.txt -o output/ -v 2>&1 | \
  tee >(grep "ERROR" > errors.log) | \
  tee >(grep "WARN" > warnings.log) > full.log

# Generate processing report
{
    echo "Processing Report - $(date)"
    echo "===================="
    echo "Total URLs processed: $(wc -l < urls.txt)"
    echo "Errors encountered: $(wc -l < errors.log)"
    echo "Warnings: $(wc -l < warnings.log)"
    echo "Output files created: $(find output/ -name "*.md" | wc -l)"
    echo ""
    echo "Top errors:"
    sort errors.log | uniq -c | sort -nr | head -5
} > processing_report.txt
```

---

!!! tip "Advanced Workflows"
    - Combine multiple processing strategies for complex content pipelines
    - Use shell scripting to create custom workflows tailored to your needs
    - Monitor system resources during large batch operations
    - Implement proper error handling and recovery mechanisms for production use
    - Consider using container orchestration for very large-scale processing