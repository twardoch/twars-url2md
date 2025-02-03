# twars-url2md

A command-line tool that converts web pages to Markdown. It fetches HTML content, processes it to remove unnecessary styling, and converts it to clean Markdown format.

## Features

- Fetch HTML content from URLs with proper user agent identification
- Remove styles and unnecessary formatting
- Convert HTML to clean Markdown
- Process multiple URLs in parallel
- Smart output path handling based on URL structure
- Support for input from:
  - Command line arguments
  - Input file (one URL per line)
  - Standard input (space or newline separated)

## Installation

### From crates.io

```bash
cargo install twars-url2md
```

### From source

```bash
git clone https://github.com/twardoch/twars-url2md.git
cd twars-url2md
cargo install --path .
```

## Usage

```bash
# Process a single URL and print to stdout
twars-url2md https://example.com

# Process a single URL and save to file
twars-url2md https://example.com -o example.md

# Process multiple URLs and save to current directory (creates domain-based folders)
twars-url2md https://example.com https://example.org

# Process multiple URLs and save to specific directory
twars-url2md https://example.com https://example.org -o output/

# Process URLs from a file
twars-url2md -f urls.txt -o output/

# Read URLs from stdin
echo "https://example.com https://example.org" | twars-url2md --stdin

# Show verbose output
twars-url2md -v https://example.com
```

### Output Path Structure

For URLs like `scheme://username:password@host:port/path?query#fragment`:
- Username, password, query parameters, port, and fragments are ignored
- Files are organized in folders based on the host and path
- For URLs ending in `/` or with no path, `index.md` is used as the filename
- For other URLs, the last path component is used as the filename (with `.md` extension)

Example:
```
$ twars-url2md https://example.com/ https://example.org/foo/bar
Created: example.com/index.md
Created: example.org/foo/bar.md
```

## License

MIT License

## Author

Adam Twardoch (@twardoch) 