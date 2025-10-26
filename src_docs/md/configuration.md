# Configuration

Customize `twars-url2md` behavior through command-line options, environment variables, and configuration files.

## Runtime Configuration

### Logging

Control logging with environment variables and command-line flags:

#### Basic Logging

```bash
# Enable verbose mode (INFO + DEBUG for twars-url2md)
twars-url2md -i urls.txt -o output/ -v

# Equivalent environment variable
RUST_LOG=info,twars_url2md=debug twars-url2md -i urls.txt -o output/
```

#### Advanced Logging Control

```bash
# Maximum verbosity
RUST_LOG=trace twars-url2md -i urls.txt -o output/

# Module-specific logging
RUST_LOG=twars_url2md::html=debug,twars_url2md::markdown=info twars-url2md -i urls.txt -o output/

# Component-specific logging
RUST_LOG=twars_url2md=debug,reqwest=info,curl=warn twars-url2md -i urls.txt -o output/
```

#### Log Format

```bash
# JSON logging for analysis tools
RUST_LOG=info RUST_LOG_FORMAT=json twars-url2md -i urls.txt -o output/ -v

# Minimal output
RUST_LOG=error twars-url2md -i urls.txt -o output/

# Log to file
twars-url2md -i urls.txt -o output/ -v 2> conversion.log
```

### Retry Logic

Retry counts aren't configurable via CLI. Use a wrapper script:

```bash
#!/bin/bash
# retry.sh

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
        delay=$((delay * 2))
        ((attempt++))
    done
    
    echo "All $max_attempts attempts failed"
    return 1
}

retry_command -i urls.txt -o output/ -v
```

## Environment Variables

### System Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `RUST_LOG` | Logging level | `RUST_LOG=debug` |
| `RUST_BACKTRACE` | Error traces | `RUST_BACKTRACE=1` |
| `HTTP_PROXY` | HTTP proxy | `HTTP_PROXY=http://proxy:8080` |
| `HTTPS_PROXY` | HTTPS proxy | `HTTPS_PROXY=http://proxy:8080` |
| `NO_PROXY` | Proxy exceptions | `NO_PROXY=localhost,127.0.0.1` |

### Proxy Settings

For restricted environments:

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

### Debug Settings

For troubleshooting:

```bash
# Full debug with backtraces
RUST_LOG=trace RUST_BACKTRACE=full twars-url2md -i problematic_urls.txt -o debug_output/ -v

# Memory debugging (with debug build)
RUST_LOG=debug RUST_BACKTRACE=1 valgrind --tool=memcheck twars-url2md -i urls.txt -o output/
```

## Configuration Files

### URL Files

Organize URLs with comments:

```txt
# urls.txt - Documentation sources
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/std/
https://doc.rust-lang.org/cargo/

# API docs  
https://docs.rs/serde/
https://docs.rs/tokio/
https://docs.rs/clap/

# Community resources
https://forge.rust-lang.org/
https://rustc-dev-guide.rust-lang.org/
```

### Batch Processing Script

Reusable configuration:

```bash
#!/bin/bash
# rust-docs.sh

set -euo pipefail

BASE_OUTPUT="rust_documentation"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_DIR="${BASE_OUTPUT}_${TIMESTAMP}"
PACK_FILE="${BASE_OUTPUT}_complete_${TIMESTAMP}.md"

export RUST_LOG=info,twars_url2md=debug

CORE_URLS="config/rust_core_urls.txt"
CRATE_URLS="config/rust_crate_urls.txt"
COMMUNITY_URLS="config/rust_community_urls.txt"

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

main() {
    echo "Starting Rust documentation collection..."
    echo "Output directory: $OUTPUT_DIR"
    echo "Pack file: $PACK_FILE"
    
    process_category "Core" "$CORE_URLS" "core"
    process_category "Crates" "$CRATE_URLS" "crates"  
    process_category "Community" "$COMMUNITY_URLS" "community"
    
    echo "Creating combined archive..."
    find "$OUTPUT_DIR" -name "*.md" | \
        twars-url2md --stdin --pack "$PACK_FILE" -v
    
    echo "Collection completed successfully!"
    echo "Individual files: $OUTPUT_DIR/"
    echo "Combined archive: $PACK_FILE"
}

main "$@"
```

### Environment-Specific Scripts

=== "Development"
    ```bash
    #!/bin/bash
    # config/dev.sh
    
    export RUST_LOG=debug
    export RUST_BACKTRACE=1
    
    twars-url2md -i test_urls.txt -o dev_output/ -v
    ```

=== "Production"
    ```bash
    #!/bin/bash
    # config/prod.sh
    
    export RUST_LOG=info
    
    if ! twars-url2md -i prod_urls.txt -o prod_output/ --pack prod_archive.md -v; then
        echo "Processing failed" >&2
        exit 1
    fi
    ```

=== "CI/CD"
    ```bash
    #!/bin/bash
    # config/ci.sh
    
    set -euo pipefail
    
    export RUST_LOG=info,twars_url2md=debug
    
    OUTPUT_DIR="${CI_PROJECT_DIR}/generated_docs"
    ARTIFACT_FILE="${CI_PROJECT_DIR}/docs_archive_${CI_COMMIT_SHA}.md"
    
    mkdir -p "$OUTPUT_DIR"
    twars-url2md -i ci_urls.txt -o "$OUTPUT_DIR/" --pack "$ARTIFACT_FILE" -v
    ```

## Performance Tuning

### Concurrency

Influence concurrency through system limits:

```bash
# Limit file descriptors and processes
ulimit -n 1024
ulimit -u 512

# Lower CPU priority for background work
nice -n 10 twars-url2md -i large_url_list.txt -o output/ -v

# Schedule off-peak processing
echo "0 2 * * * /path/to/twars-url2md -i /path/to/urls.txt -o /path/to/output/ -v" | crontab -
```

### Memory Monitoring

```bash
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

### Network Settings

```bash
# Custom CA bundle
export CURL_CA_BUNDLE=/path/to/custom/ca-bundle.crt
export SSL_CERT_FILE=/path/to/custom/ca-bundle.crt

# Corporate firewall settings
export http_proxy=http://proxy.company.com:8080
export https_proxy=http://proxy.company.com:8080
export no_proxy=localhost,127.0.0.1,*.company.com

twars-url2md -i corporate_urls.txt -o output/ -v
```

## Output Customization

### Directory Organization

Group URLs by domain:

```bash
#!/bin/bash
# group_by_domain.sh

process_by_domain() {
    local url_file="$1"
    
    awk -F'/' '{print $3}' "$url_file" | sort -u | while read -r domain; do
        grep "$domain" "$url_file" > "urls_${domain}.txt"
        mkdir -p "output_by_domain/$domain"
        twars-url2md -i "urls_${domain}.txt" -o "output_by_domain/$domain/" -v
        rm "urls_${domain}.txt"
    done
}

process_by_domain "all_urls.txt"
```

### Content Post-processing

```bash
#!/bin/bash
# post_process.sh

# Process URLs
twars-url2md -i urls.txt -o raw_output/ -v

# Add headers to markdown files
find raw_output -name "*.md" | while read -r file; do
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
    echo "# Documentation Index"
    echo ""
    echo "Generated on: $(date)"
    echo ""
    find raw_output -name "*.md" | sort | while read -r file; do
        title=$(head -n 10 "$file" | grep -E '^#' | head -n 1 | sed 's/^# *//')
        echo "- [$title]($file)"
    done
} > raw_output/INDEX.md
```

## Integration

### Docker

```dockerfile
# Dockerfile
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash

ENV RUST_LOG=info
ENV PATH=/root/.local/bin:$PATH

COPY config/ /app/config/
WORKDIR /app

CMD ["twars-url2md", "-i", "config/urls.txt", "-o", "output/", "-v"]
```

### Kubernetes

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

### Systemd Service

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

```ini
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

## Troubleshooting

### Debug Script

```bash
#!/bin/bash
# debug.sh

set -x

echo "=== Environment ==="
env | grep -E "(RUST|HTTP|PROXY)" | sort

echo "=== System Info ==="
uname -a
which twars-url2md
twars-url2md --version

echo "=== Network Test ==="
curl -I https://httpbin.org/get

echo "=== Processing Test ==="
RUST_LOG=trace RUST_BACKTRACE=full \
    twars-url2md https://httpbin.org/html -o debug_test/ -v

echo "=== Results ==="
find debug_test -type f -exec ls -la {} \;
find debug_test -name "*.md" -exec head -5 {} \;
```

### Configuration Validator

```bash
#!/bin/bash
# validate.sh

validate_urls() {
    local url_file="$1"
    
    echo "Validating URLs in $url_file..."
    
    while IFS= read -r url; do
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
    
    if command -v twars-url2md >/dev/null 2>&1; then
        echo "✓ Binary found"
    else
        echo "✗ Binary missing"
        return 1
    fi
    
    twars-url2md --version
    
    if curl -s https://httpbin.org/get >/dev/null; then
        echo "✓ Network OK"
    else
        echo "✗ Network failed"
        return 1
    fi
}

validate_environment
validate_urls "$1"
```

---

!!! tip "Configuration Best Practices"
    - Store sensitive data in environment variables
    - Use version control for configuration files
    - Test configurations before deployment
    - Monitor resource usage
    - Document custom workflows