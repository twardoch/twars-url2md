# Installation

`twars-url2md` supports several installation methods. Pick the one that fits your environment.

## Pre-compiled Binaries (Recommended)

Binaries are ready to use and require no dependencies.

### One-Line Installation

=== "Linux & macOS"
    ```bash
    curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash
    ```

=== "Custom Directory"
    ```bash
    curl -fsSL https://raw.githubusercontent.com/twardoch/twars-url2md/main/install.sh | bash -s -- --install-dir ~/.local/bin
    ```

=== "Windows PowerShell"
    ```powershell
    # Download and run the Windows installer manually
    # See manual installation section below
    ```

### Manual Binary Installation

Download from the [GitHub Releases page](https://github.com/twardoch/twars-url2md/releases/latest).

=== "macOS"
    ```bash
    # Intel x86_64
    curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-macos-x86_64.tar.gz | tar xz
    
    # Apple Silicon (M1/M2/M3)
    curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-macos-aarch64.tar.gz | tar xz
    
    # Move to PATH
    sudo mv twars-url2md /usr/local/bin/
    ```

=== "Linux"
    ```bash
    # x86_64
    curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-x86_64.tar.gz | tar xz
    
    # ARM64 (aarch64)
    curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-aarch64.tar.gz | tar xz
    
    # Static binary (musl) - works on any Linux
    curl -L https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-linux-x86_64-musl.tar.gz | tar xz
    
    # Move to PATH
    sudo mv twars-url2md /usr/local/bin/
    ```

=== "Windows"
    ```powershell
    # Download the zip file
    Invoke-WebRequest -Uri https://github.com/twardoch/twars-url2md/releases/latest/download/twars-url2md-windows-x86_64.zip -OutFile twars-url2md.zip
    
    # Extract
    Expand-Archive twars-url2md.zip -DestinationPath .
    
    # Move to a directory in your PATH (example: C:\Windows\System32)
    # Or add the current directory to your PATH environment variable
    ```

## Package Managers

### Cargo (Rust Package Manager)

If you have Rust installed (version 1.70.0 or later):

```bash
cargo install twars-url2md
```

This builds from source and may take a few minutes, but gives you an optimized binary for your system.

### Homebrew (macOS/Linux)

!!! warning "Coming Soon"
    Homebrew formula planned for a future release.

### Chocolatey (Windows)

!!! warning "Coming Soon"
    Chocolatey package planned for a future release.

## Building from Source

For developers or those who want the latest features:

### Prerequisites

- **Rust**: Version 1.70.0 or later ([Install Rust](https://rustup.rs/))
- **Git**: For cloning the repository
- **C compiler**: Usually part of system development tools

### Build Process

```bash
# Clone the repository
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md

# Build in release mode (optimized)
cargo build --release

# Binary located at target/release/twars-url2md
```

### Install Locally

```bash
# Install to ~/.cargo/bin (must be in your PATH)
cargo install --path .
```

### Development Build

For development or testing:

```bash
# Debug build (faster compilation, slower execution)
cargo build

# Run directly without installing
cargo run -- --help
```

## Verification

After installation, verify that `twars-url2md` works correctly:

```bash
# Check version and build info
twars-url2md --version

# Test with a simple URL
twars-url2md https://httpbin.org/html -o test_output/
```

Expected output shows version information and creates a markdown file.

## Container Usage

### Docker

No official Docker image yet, but you can create one using the static binary:

```dockerfile
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY twars-url2md /usr/local/bin/
ENTRYPOINT ["twars-url2md"]
```

Build and run:

```bash
# Build the Docker image
docker build -t twars-url2md .

# Run with volume mount for output
docker run -v $(pwd)/output:/output twars-url2md https://example.com -o /output
```

## Troubleshooting Installation

### Common Issues

=== "Permission Denied"
    ```bash
    # If you get permission denied when moving to /usr/local/bin
    sudo mv twars-url2md /usr/local/bin/
    
    # Or install to a user directory
    mkdir -p ~/.local/bin
    mv twars-url2md ~/.local/bin/
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

=== "Command Not Found"
    ```bash
    # Verify the binary is in your PATH
    which twars-url2md
    
    # Check your PATH
    echo $PATH
    
    # Add directory to PATH if needed
    export PATH="/usr/local/bin:$PATH"
    ```

=== "SSL/TLS Errors"
    ```bash
    # Update CA certificates (Linux)
    sudo apt-get update && sudo apt-get install ca-certificates
    
    # macOS - update certificates
    brew update && brew upgrade ca-certificates
    ```

=== "Rust Compilation Errors"
    ```bash
    # Update Rust toolchain
    rustup update
    
    # Install required components
    rustup component add rustfmt clippy
    
    # Clear cache and rebuild
    cargo clean
    cargo build --release
    ```

### Platform-Specific Notes

=== "Linux"
    - **Static binary** (`musl`) works on any Linux distribution
    - **Dynamic binary** requires glibc 2.17+ (available on most modern distributions)
    - For older distributions, use the static binary or build from source

=== "macOS"
    - **x86_64** version works on Intel Macs and Apple Silicon under Rosetta 2
    - **aarch64** version is optimized for Apple Silicon (M1/M2/M3)
    - macOS 10.15+ required for pre-compiled binaries

=== "Windows"
    - Requires Windows 10 or later
    - Binary is built with MSVC toolchain
    - No additional runtime dependencies required

## Next Steps

Once installed, see the [Quick Start Guide](quickstart.md) for basic usage or [Basic Usage](usage.md) for full command-line documentation.

---

!!! tip "Stay Updated"
    - Watch the [GitHub repository](https://github.com/twardoch/twars-url2md) for new releases
    - Check the [changelog](https://github.com/twardoch/twars-url2md/releases) for version updates
    - Enable GitHub notifications for release announcements