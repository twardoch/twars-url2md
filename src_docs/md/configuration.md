# Configuration

Customize `twars-url2md` behavior through command-line options, environment variables, and advanced configuration techniques.

## Runtime Configuration

### Logging Configuration

Control logging output with environment variables and command-line flags:

#### Basic Logging

```bash
# Enable verbose mode (INFO + DEBUG for twars-url2md)
twars-url2md -i urls.txt -o output/ -v

# Equivalent environment variable
RUST_LOG=info,twars_url2md=debug twars-url2md -i urls.txt -o output/
```

#### Advanced Logging Control

```bash
# Maximum verbosity for debugging
RUST_LOG=trace twars-url2md -i urls.txt -o output/

# Module-specific logging
RUST_LOG=twars_url2md::html=debug,twars_url2md::markdown=info twars-url2md -i urls.txt -o output/

# Focus on specific components
RUST_LOG=twars_url2md=debug,reqwest=info,curl=warn twars-url2md -i urls.txt -o output/
```

#### Log Format Customization

```bash
# Structured JSON logging (for log analysis tools)
RUST_LOG=info RUST_LOG_FORMAT=json twars-url2md -i urls.txt -o output/ -v

# Minimal logging output
RUST_LOG=error twars-url2md -i urls.txt -o output/

# Log to file
twars-url2md -i urls.txt -o output/ -v 2> conversion.log
```

### Retry Configuration

While retry counts aren't directly configurable via CLI, you can implement custom retry logic:

```bash
#!/bin/bash
# Custom retry wrapper

retry_command() {
    local max_attempts=5
    local delay=2
    local attempt=1
    
    while ((attempt <= max_attempts)); do
        if twars-url2md "$@"; then
            return 0
        fi
        
        echo "Attempt $attempt failed, retrying in ${delay}s..."
        sleep $delay
        delay=$((delay * 2))  # Exponential backoff
        ((attempt++))
    done
    
    echo "All $max_attempts attempts failed"
    return 1
}

# Usage
retry_command -i urls.txt -o output/ -v
```

## Environment Variables

### System Environment

Key environment variables that affect behavior:

| Variable | Purpose | Example |
|----------|---------|---------|
| `RUST_LOG` | Logging configuration | `RUST_LOG=debug` |
| `RUST_BACKTRACE` | Error backtraces | `RUST_BACKTRACE=1` |
| `HTTP_PROXY` | HTTP proxy settings | `HTTP_PROXY=http://proxy:8080` |
| `HTTPS_PROXY` | HTTPS proxy settings | `HTTPS_PROXY=http://proxy:8080` |
| `NO_PROXY` | Proxy bypass list | `NO_PROXY=localhost,127.0.0.1` |

### Proxy Configuration

For corporate or restricted environments:

```bash
# HTTP proxy
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
export NO_PROXY=localhost,127.0.0.1,*.local

twars-url2md -i urls.txt -o output/ -v

# Authenticated proxy
export HTTP_PROXY=http://username:password@proxy.company.com:8080
twars-url2md -i urls.txt -o output/ -v

# SOCKS proxy (if curl supports it)
export ALL_PROXY=socks5://proxy.company.com:1080
twars-url2md -i urls.txt -o output/ -v
```

### Debug Configuration

Enhanced debugging for troubleshooting:

```bash
# Full debug mode with backtraces
RUST_LOG=trace RUST_BACKTRACE=full twars-url2md -i problematic_urls.txt -o debug_output/ -v

# Memory debugging (if compiled with debug info)
RUST_LOG=debug RUST_BACKTRACE=1 valgrind --tool=memcheck twars-url2md -i urls.txt -o output/
```

## Configuration Files

### URL Configuration Files

Organize URLs with comments and metadata:

```txt
# urls.txt - Main documentation URLs
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/std/
https://doc.rust-lang.org/cargo/

# API documentation  
https://docs.rs/serde/
https://docs.rs/tokio/
https://docs.rs/clap/

# Community resources
https://forge.rust-lang.org/
https://rustc-dev-guide.rust-lang.org/
```

### Batch Configuration Scripts

Create reusable configuration scripts:

```bash
#!/bin/bash
# config/rust-docs.sh - Rust documentation collection

set -euo pipefail

# Configuration
BASE_OUTPUT="rust_documentation"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_DIR="${BASE_OUTPUT}_${TIMESTAMP}"
PACK_FILE="${BASE_OUTPUT}_complete_${TIMESTAMP}.md"

# Logging configuration
export RUST_LOG=info,twars_url2md=debug

# URL categories
CORE_URLS="config/rust_core_urls.txt"
CRATE_URLS="config/rust_crate_urls.txt"
COMMUNITY_URLS="config/rust_community_urls.txt"

# Processing function
process_category() {
    local name="$1"
    local url_file="$2"
    local output_subdir="$3"
    
    echo "Processing $name URLs..."
    mkdir -p "$OUTPUT_DIR/$output_subdir"
    
    if twars-url2md -i "$url_file" -o "$OUTPUT_DIR/$output_subdir/" -v; then
        echo "✓ $name processing completed"
        return 0
    else
        echo "✗ $name processing failed"
        return 1
    fi
}

# Main processing
main() {
    echo "Starting Rust documentation collection..."
    echo "Output directory: $OUTPUT_DIR"
    echo "Pack file: $PACK_FILE"
    
    # Process categories
    process_category "Core" "$CORE_URLS" "core"
    process_category "Crates" "$CRATE_URLS" "crates"  
    process_category "Community" "$COMMUNITY_URLS" "community"
    
    # Create combined archive
    echo "Creating combined archive..."
    find "$OUTPUT_DIR" -name "*.md" | \
        twars-url2md --stdin --pack "$PACK_FILE" -v
    
    echo "Collection completed successfully!"
    echo "Individual files: $OUTPUT_DIR/"
    echo "Combined archive: $PACK_FILE"
}

main "$@"
```

### Environment-Specific Configurations

=== "Development"
    ```bash
    #!/bin/bash
    # config/dev.sh
    
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    
    # Use local test URLs
    twars-url2md -i test_urls.txt -o dev_output/ -v
    ```

=== "Production"
    ```bash
    #!/bin/bash
    # config/prod.sh
    
    export RUST_LOG=info
    
    # Production processing with error handling
    if ! twars-url2md -i prod_urls.txt -o prod_output/ --pack prod_archive.md -v; then
        echo "Production processing failed" >&2
        exit 1
    fi
    ```

=== "CI/CD"
    ```bash
    #!/bin/bash
    # config/ci.sh
    
    set -euo pipefail
    
    export RUST_LOG=info,twars_url2md=debug
    
    # CI-specific output paths
    OUTPUT_DIR="${CI_PROJECT_DIR}/generated_docs"
    ARTIFACT_FILE="${CI_PROJECT_DIR}/docs_archive_${CI_COMMIT_SHA}.md"
    
    mkdir -p "$OUTPUT_DIR"
    
    twars-url2md -i ci_urls.txt -o "$OUTPUT_DIR/" --pack "$ARTIFACT_FILE" -v
    ```

## Performance Tuning

### Concurrency Configuration

While not directly configurable, you can influence concurrency behavior:

```bash
# Limit system resources to control concurrency
ulimit -n 1024  # Limit file descriptors
ulimit -u 512   # Limit processes

# Use nice to lower CPU priority for background processing
nice -n 10 twars-url2md -i large_url_list.txt -o output/ -v

# Process during off-peak hours
echo "0 2 * * * /path/to/twars-url2md -i /path/to/urls.txt -o /path/to/output/ -v" | crontab -
```

### Memory Management

```bash
# Monitor memory usage during processing
{
    twars-url2md -i urls.txt -o output/ -v &
    PID=$!
    
    while kill -0 $PID 2>/dev/null; do
        ps -p $PID -o pid,vsz,rss,pcpu
        sleep 10
    done
    
    wait $PID
} | tee memory_usage.log
```

### Network Configuration

```bash
# Timeout configuration (system-level)
# These affect the underlying curl library

export CURL_CA_BUNDLE=/path/to/custom/ca-bundle.crt  # Custom CA bundle
export SSL_CERT_FILE=/path/to/custom/ca-bundle.crt   # Alternative CA bundle

# Corporate firewall handling
export http_proxy=http://proxy.company.com:8080
export https_proxy=http://proxy.company.com:8080
export no_proxy=localhost,127.0.0.1,*.company.com

twars-url2md -i corporate_urls.txt -o output/ -v
```

## Output Customization

### Directory Structure Templates

Customize output organization:

```bash
#!/bin/bash
# Custom output organization

process_by_domain() {
    local url_file="$1"
    
    # Group URLs by domain
    awk -F'/' '{print $3}' "$url_file" | sort -u | while read -r domain; do
        grep "$domain" "$url_file" > "urls_${domain}.txt"
        mkdir -p "output_by_domain/$domain"
        twars-url2md -i "urls_${domain}.txt" -o "output_by_domain/$domain/" -v
        rm "urls_${domain}.txt"
    done
}

process_by_domain "all_urls.txt"
```

### Content Processing Templates

```bash
#!/bin/bash
# Content post-processing

# Standard processing
twars-url2md -i urls.txt -o raw_output/ -v

# Post-process markdown files
find raw_output -name "*.md" | while read -r file; do
    # Add custom header
    {
        echo "---"
        echo "generated: $(date -Iseconds)"
        echo "source: twars-url2md"
        echo "---"
        echo ""
        cat "$file"
    } > "${file}.tmp" && mv "${file}.tmp" "$file"
done

# Create index
{
    echo "# Generated Documentation Index"
    echo ""
    echo "Generated on: $(date)"
    echo ""
    find raw_output -name "*.md" | sort | while read -r file; do
        title=$(head -n 10 "$file" | grep -E '^#' | head -n 1 | sed 's/^# *//')
        echo "- [$title]($file)"
    done
} > raw_output/INDEX.md
```

## Integration Configurations

### Docker Configuration

```dockerfile
# Dockerfile for configured environment
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install twars-url2md
RUN curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash

# Set environment
ENV RUST_LOG=info
ENV PATH=/root/.local/bin:$PATH

# Copy configuration
COPY config/ /app/config/
WORKDIR /app

# Default command
CMD ["twars-url2md", "-i", "config/urls.txt", "-o", "output/", "-v"]
```

### Kubernetes Configuration

```yaml
# k8s-job.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: url-to-md-processor
spec:
  template:
    spec:
      containers:
      - name: processor
        image: twars-url2md:latest
        env:
        - name: RUST_LOG
          value: "info,twars_url2md=debug"
        - name: HTTP_PROXY
          valueFrom:
            configMapKeyRef:
              name: proxy-config
              key: http_proxy
        volumeMounts:
        - name: config-volume
          mountPath: /app/config
        - name: output-volume
          mountPath: /app/output
        command: ["twars-url2md"]
        args: ["-i", "/app/config/urls.txt", "-o", "/app/output/", "-v"]
      volumes:
      - name: config-volume
        configMap:
          name: url-config
      - name: output-volume
        persistentVolumeClaim:
          claimName: output-pvc
      restartPolicy: Never
```

### Systemd Service Configuration

```ini
# /etc/systemd/system/url-processor.service
[Unit]
Description=URL to Markdown Processor
After=network.target

[Service]
Type=oneshot
User=processor
Group=processor
Environment=RUST_LOG=info,twars_url2md=debug
ExecStart=/usr/local/bin/twars-url2md -i /etc/url-processor/urls.txt -o /var/lib/url-processor/output/ -v
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

```bash
# Timer for periodic processing
# /etc/systemd/system/url-processor.timer
[Unit]
Description=Run URL processor daily
Requires=url-processor.service

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
```

## Troubleshooting Configuration

### Debug Configuration

```bash
#!/bin/bash
# debug_config.sh - Comprehensive debugging

set -x  # Enable command tracing

# Environment debugging
echo "=== Environment ==="
env | grep -E "(RUST|HTTP|PROXY)" | sort

echo "=== System Information ==="
uname -a
which twars-url2md
twars-url2md --version

echo "=== Network Connectivity ==="
curl -I https://httpbin.org/get

echo "=== Processing Test ==="
RUST_LOG=trace RUST_BACKTRACE=full \
    twars-url2md https://httpbin.org/html -o debug_test/ -v

echo "=== Debug Results ==="
find debug_test -type f -exec ls -la {} \;
find debug_test -name "*.md" -exec head -5 {} \;
```

### Configuration Validation

```bash
#!/bin/bash
# validate_config.sh

validate_urls() {
    local url_file="$1"
    
    echo "Validating URLs in $url_file..."
    
    while IFS= read -r url; do
        # Skip comments and empty lines
        [[ "$url" =~ ^[[:space:]]*# ]] && continue
        [[ -z "${url// }" ]] && continue
        
        if curl -s --head "$url" | head -n 1 | grep -q "200 OK"; then
            echo "✓ $url"
        else
            echo "✗ $url"
        fi
    done < "$url_file"
}

validate_environment() {
    echo "Validating environment..."
    
    # Check binary
    if command -v twars-url2md >/dev/null 2>&1; then
        echo "✓ twars-url2md binary found"
    else
        echo "✗ twars-url2md binary not found in PATH"
        return 1
    fi
    
    # Check version
    twars-url2md --version
    
    # Check network
    if curl -s https://httpbin.org/get >/dev/null; then
        echo "✓ Network connectivity OK"
    else
        echo "✗ Network connectivity failed"
        return 1
    fi
}

# Run validations
validate_environment
validate_urls "$1"
```

---

!!! tip "Configuration Best Practices"
    - Use environment variables for sensitive data (proxies, credentials)
    - Create reusable configuration scripts for common workflows
    - Test configurations in development before production deployment
    - Document custom configurations for team members
    - Use version control for configuration files and scripts
    - Monitor resource usage and adjust configurations accordingly