use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use tokio;

use crate::url::extract_urls_from_text;

/// Command-line interface for URL processing
#[derive(Parser)]
#[command(
    name = "twars-url2md",
    author = "Adam Twardoch",
    version = env!("CARGO_PKG_VERSION"),
    about = "Convert web pages to clean Markdown format while preserving content structure",
    long_about = "\
A powerful CLI tool that fetches web pages and converts them to clean Markdown format \
using Monolith for content extraction and htmd for conversion"
)]
pub struct Cli {
    /// Input file to process
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output directory for markdown files
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Read from stdin
    #[arg(long)]
    stdin: bool,

    /// Base URL for resolving relative URLs
    #[arg(long)]
    base_url: Option<String>,

    /// Output file to pack all markdown files together
    #[arg(short = 'p', long)]
    pack: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    /// Parse command-line arguments with custom error handling
    pub fn parse_args() -> Result<Self> {
        let args: Vec<_> = std::env::args().collect();
        let cli = if args.iter().any(|arg| arg == "-v" || arg == "--verbose") {
            Self::parse()
        } else {
            match Self::try_parse() {
                Ok(cli) => {
                    // Add validation for input arguments
                    if !cli.stdin && cli.input.is_none() {
                        eprintln!("Error: Either --stdin or --input must be specified");
                        eprintln!("Run with --help for usage information");
                        std::process::exit(1);
                    }
                    cli
                }
                Err(err) => {
                    if err.kind() == clap::error::ErrorKind::DisplayHelp
                        || err.kind() == clap::error::ErrorKind::DisplayVersion
                    {
                        // For help and version, Clap prints the message itself.
                        // We just need to exit cleanly.
                        std::process::exit(0);
                    }
                    // For other errors, print a concise message.
                    // Clap's default error messages can be verbose.
                    eprintln!(
                        "Error: {}",
                        err.render()
                            .to_string()
                            .lines()
                            .next()
                            .unwrap_or("Invalid command line arguments.")
                    );
                    eprintln!("Run with --help for usage information.");
                    std::process::exit(1);
                }
            }
        };

        Ok(cli)
    }

    /// Collect URLs from all input sources
    pub fn collect_urls(&self) -> io::Result<Vec<String>> {
        tracing::debug!("Collecting URLs from input sources...");
        // Get content from stdin or file
        let content = if self.stdin {
            tracing::info!("Reading URLs from stdin.");
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else if let Some(input_path) = &self.input {
            tracing::info!("Reading URLs from file: {}", input_path.display());
            fs::read_to_string(input_path)?
        } else {
            // This case should be caught by parse_args validation
            tracing::error!("Neither stdin nor input file specified during URL collection.");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Internal error: Neither stdin nor input file specified, but validation passed.",
            ));
        };

        // Extract URLs from content
        let urls = extract_urls_from_text(&content, self.base_url.as_deref());
        tracing::debug!("Extracted {} URLs from content.", urls.len());
        Ok(urls)
    }

    /// Create configuration from CLI arguments
    pub fn create_config(&self) -> crate::Config {
        crate::Config {
            verbose: self.verbose,
            max_retries: 2,
            output_base: self.output.clone().unwrap_or_else(|| PathBuf::from(".")),
            single_file: self.input.is_none(),
            has_output: self.output.is_some(),
            pack_file: self.pack.clone(),
        }
    }
}

pub async fn run() -> io::Result<()> {
    // Use unwrap() instead of ? because parse_args returns anyhow::Result
    // which is not compatible with io::Result
    let cli = match Cli::parse_args() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    };

    // Validate input options
    if cli.stdin && cli.input.is_some() {
        eprintln!("Error: Cannot use both --stdin and --input");
        std::process::exit(1);
    }

    // Extract URLs from content
    let urls = cli.collect_urls()?;

    // Process output
    if let Some(output_dir) = cli.output.clone() {
        fs::create_dir_all(&output_dir)?;
        for url in urls {
            // Create markdown file for each URL
            let mut file_path = output_dir.clone();
            file_path.push(format!("{}.md", url_to_filename(&url)));
            tokio::fs::write(file_path, format!("# {}\n\n{}\n", url, url)).await?;
        }
    } else {
        // Print URLs to stdout if no output directory specified
        for url in urls {
            println!("{}", url);
        }
    }

    Ok(())
}

fn url_to_filename(url: &str) -> String {
    // Convert URL to a valid filename
    let mut filename = url
        .replace(
            [
                ':', '/', '?', '#', '[', ']', '@', '!', '$', '&', '\'', '(', ')', '*', '+', ',',
                ';', '=',
            ],
            "_",
        )
        .replace([' ', '\t', '\n', '\r'], "");

    // Ensure the filename is not too long
    if filename.len() > 200 {
        filename.truncate(200);
    }

    filename
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_collect_urls_from_text_file() -> Result<()> {
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("sample_urls.txt");
        let test_content = "\
            https://example.com/\n\
            http://test.org/\n\
            https://rust-lang.org/\n\
            https://github.com/example/repo\n\
            http://blog.example.com/post/123\n\
            https://docs.example.com/guide#section\n\
            ftp://invalid.com\n\
            not-a-url.com\n\
            www.example.com";

        // Create test file with sample URLs
        fs::write(&test_file, test_content)?;

        // Test file input
        let cli = Cli {
            input: Some(test_file),
            output: None,
            stdin: false,
            base_url: None,
            pack: None,
            verbose: false,
        };

        let urls = cli.collect_urls()?;
        println!("Found URLs: {:?}", urls);
        verify_urls(&urls);

        Ok(())
    }

    fn verify_urls(urls: &[String]) {
        println!("Found URLs: {:?}", urls);

        // Test for basic URLs (with trailing slashes)
        assert!(urls.iter().any(|u| u == "https://example.com/"));
        assert!(urls.iter().any(|u| u == "http://test.org/"));
        assert!(urls.iter().any(|u| u == "https://rust-lang.org/"));

        // Test for URLs with paths and fragments
        assert!(urls.iter().any(|u| u == "https://github.com/example/repo"));
        assert!(urls.iter().any(|u| u == "http://blog.example.com/post/123"));
        assert!(urls
            .iter()
            .any(|u| u == "https://docs.example.com/guide#section"));

        // Make sure invalid URLs are not included
        assert!(!urls.iter().any(|u| u.starts_with("ftp://")));
        assert!(!urls.iter().any(|u| u == "not-a-url.com"));
        assert!(!urls.iter().any(|u| u == "www.example.com"));

        assert_eq!(urls.len(), 6, "Expected exactly 6 valid URLs");
    }
}
