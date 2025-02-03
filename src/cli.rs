use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

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
}

impl Cli {
    /// Parse command-line arguments with custom error handling
    pub fn parse_args() -> Result<Self> {
        let args: Vec<_> = std::env::args().collect();
        let cli = if args.iter().any(|arg| arg == "-v" || arg == "--verbose") {
            Self::parse()
        } else {
            match Self::try_parse() {
                Ok(cli) => cli,
                Err(err) => {
                    if err.kind() == clap::error::ErrorKind::DisplayHelp
                        || err.kind() == clap::error::ErrorKind::DisplayVersion
                    {
                        println!("{}", err);
                        std::process::exit(0);
                    }
                    eprintln!(
                        "Error: {}",
                        err.render()
                            .to_string()
                            .lines()
                            .next()
                            .unwrap_or("Invalid usage")
                    );
                    std::process::exit(1);
                }
            }
        };

        Ok(cli)
    }

    /// Collect URLs from all input sources
    pub fn collect_urls(&self) -> io::Result<Vec<String>> {
        // Get content from stdin or file
        let content = if self.stdin {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else if let Some(input_path) = &self.input {
            fs::read_to_string(input_path)?
        } else {
            unreachable!()
        };

        // Extract URLs from content
        Ok(extract_urls_from_text(&content, self.base_url.as_deref()))
    }

    /// Create configuration from CLI arguments
    pub fn create_config(&self) -> crate::Config {
        crate::Config {
            verbose: true,
            max_retries: 2,
            output_base: self
                .output
                .clone()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(".")),
            single_file: self.input.is_none(),
            has_output: self.output.is_some(),
        }
    }
}

pub fn run() -> io::Result<()> {
    let cli = Cli::parse();

    // Validate input options
    if !cli.stdin && cli.input.is_none() {
        eprintln!("Error: Either --stdin or --input must be specified");
        std::process::exit(1);
    }

    if cli.stdin && cli.input.is_some() {
        eprintln!("Error: Cannot use both --stdin and --input");
        std::process::exit(1);
    }

    // Extract URLs from content
    let urls = cli.collect_urls()?;

    // Process output
    if let Some(output_dir) = cli.output {
        fs::create_dir_all(&output_dir)?;
        for url in urls {
            // Create markdown file for each URL
            let mut file_path = output_dir.clone();
            file_path.push(format!("{}.md", url_to_filename(&url)));
            fs::write(file_path, format!("# {}\n\n{}\n", url, url))?;
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
