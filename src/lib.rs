//! # twars-url2md
//!
//! A high-performance Rust library and CLI tool for converting web pages to clean Markdown files.
//!
//! ## Overview
//!
//! `twars-url2md` fetches web pages, cleans their HTML content using Monolith, and converts them
//! to readable Markdown format. It supports concurrent processing, intelligent error recovery,
//! and flexible output options.
//!
//! ## Features
//!
//! - **URL Extraction**: Finds URLs in plain text, HTML, and Markdown
//! - **Concurrent Processing**: Adaptive parallelism based on CPU cores
//! - **Error Recovery**: Automatic retries with exponential backoff
//! - **Flexible Output**: Directory structure or single file output
//! - **Local File Support**: Process local HTML files alongside remote URLs
//!
//! ## Library Usage
//!
//! ```rust,no_run
//! use twars_url2md::{process_urls, Config};
//! use std::path::PathBuf;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let urls = vec![
//!     "https://example.com".to_string(),
//!     "https://rust-lang.org".to_string(),
//! ];
//!
//! let config = Config {
//!     verbose: true,
//!     max_retries: 3,
//!     output_base: PathBuf::from("./output"),
//!     single_file: false,
//!     has_output: true,
//!     pack_file: None,
//! };
//!
//! let errors = process_urls(urls, config).await?;
//! for (url, error) in errors {
//!     eprintln!("Failed to process {}: {}", url, error);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## CLI Usage
//!
//! ```bash
//! # Process URLs from a file
//! twars-url2md -i urls.txt -o ./output
//!
//! # Process from stdin
//! echo "https://example.com" | twars-url2md --stdin -o ./output
//!
//! # Create packed output
//! twars-url2md -i urls.txt --pack combined.md
//! ```

use crate::url::Url;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub mod cli;
mod html;
pub mod markdown;
pub mod url;

pub use cli::Cli;
// pub use error::Error; // Removed

include!(concat!(env!("OUT_DIR"), "/built.rs"));

/// Version information with build details
pub fn version() -> String {
    format!(
        "{}\nBuild Time: {}\nTarget: {}\nProfile: {}",
        env!("CARGO_PKG_VERSION"),
        BUILT_TIME_UTC,
        TARGET,
        PROFILE
    )
}

/// Default user agent string for HTTP requests
pub(crate) const USER_AGENT_STRING: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Configuration for URL processing.
///
/// This struct contains all the configuration options needed to process
/// a batch of URLs, including output paths, retry settings, and verbosity.
#[derive(Debug, Clone)]
pub struct Config {
    /// Enable verbose logging output
    pub verbose: bool,
    /// Maximum number of retries for failed URLs
    pub max_retries: u32,
    /// Base directory or file path for output
    pub output_base: PathBuf,
    /// Whether to output to a single file instead of directory structure
    pub single_file: bool,
    /// Whether an output path was specified
    pub has_output: bool,
    /// Optional file path to pack all content into
    pub pack_file: Option<PathBuf>,
}

/// Process a list of URLs with the given configuration.
///
/// This is the main entry point for batch URL processing. It handles concurrent
/// downloads, error recovery, and output generation according to the provided
/// configuration.
///
/// # Arguments
///
/// * `urls` - A vector of URLs to process
/// * `config` - Configuration options for processing
///
/// # Returns
///
/// A `Result` containing a vector of errors (URL and error pairs) for URLs that
/// failed to process. An empty vector indicates all URLs were processed successfully.
///
/// # Examples
///
/// ```rust,no_run
/// use twars_url2md::{process_urls, Config};
/// use std::path::PathBuf;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let urls = vec![
///     "https://example.com".to_string(),
///     "https://rust-lang.org".to_string(),
/// ];
///
/// let config = Config {
///     verbose: true,
///     max_retries: 3,
///     output_base: PathBuf::from("./output"),
///     single_file: false,
///     has_output: true,
///     pack_file: None,
/// };
///
/// let errors = process_urls(urls, config).await?;
/// if errors.is_empty() {
///     println!("All URLs processed successfully!");
/// } else {
///     for (url, error) in errors {
///         eprintln!("Failed to process {}: {}", url, error);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub async fn process_urls(
    urls: Vec<String>,
    config: Config,
) -> Result<Vec<(String, anyhow::Error)>> {
    use indicatif::{ProgressBar, ProgressStyle};
    use tokio::io::AsyncWriteExt;

    let pb = if urls.len() > 1 {
        let pb = ProgressBar::new(urls.len() as u64);
        let style = ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .map_err(|e| {
                tracing::warn!(
                    "Failed to create progress bar template: {}. Using default style.",
                    e
                )
            })
            .unwrap_or_else(|_| ProgressStyle::default_bar()) // Fallback to default style
            .progress_chars("#>-");
        pb.set_style(style);
        Some(pb)
    } else {
        None
    };

    let pb = Arc::new(pb);
    // Adaptive concurrency based on CPU cores
    let concurrency_limit = thread::available_parallelism()
        .map(|n| n.get() * 2) // 2 tasks per CPU core
        .unwrap_or(10);
    tracing::debug!("Concurrency limit set to: {}", concurrency_limit);

    // If pack_file is specified, collect the markdown content
    let should_pack = config.pack_file.is_some();
    let pack_path = config.pack_file.clone();
    let packed_content = if should_pack {
        tracing::debug!("Packing mode enabled. Output will be: {:?}", pack_path);
        Arc::new(tokio::sync::Mutex::new(Vec::with_capacity(urls.len())))
    } else {
        Arc::new(tokio::sync::Mutex::new(Vec::new())) // Empty vec if not packing
    };

    // Clone the URLs vector before moving it into the stream for ordering packed content later
    let urls_for_ordering = if should_pack {
        urls.clone()
    } else {
        Vec::new()
    };

    let results = stream::iter(urls.into_iter().map(|url_str| {
        let pb_clone = Arc::clone(&pb);
        let config_clone = config.clone();
        let packed_content_clone = Arc::clone(&packed_content);
        async move {
            tracing::info!("Processing URL: {}", url_str);
            match Url::parse(&url_str) {
                Ok(url_parsed) => {
                    let out_path = if config_clone.single_file
                        && config_clone.has_output
                        && !config_clone.output_base.is_dir()
                    {
                        Some(config_clone.output_base)
                    } else {
                        match url::create_output_path(&url_parsed, &config_clone.output_base) {
                            Ok(p) => Some(p),
                            Err(e) => {
                                tracing::error!("Failed to create output path for {}: {}", url_str, e);
                                None
                            }
                        }
                    };
                    tracing::debug!("Output path for {}: {:?}", url_str, out_path);

                    let result = if should_pack {
                        // Process URL and collect content for packing
                        // Calls html::process_url_content_with_retry now
                        match html::process_url_content_with_retry(
                            &url_str,
                            out_path,
                            config_clone.max_retries,
                        )
                        .await
                        {
                            Ok(content_opt) => {
                                if let Some(md_content) = content_opt {
                                    // md_content will be empty if it's a real empty page,
                                    // content_opt will be None if it was a non-HTML skip.
                                    // The packing logic should only add non-empty actual content.
                                    // The current check `if !md_content.is_empty()` is correct.
                                    if !md_content.is_empty() {
                                        let mut content_vec = packed_content_clone.lock().await;
                                        content_vec.push((url_str.clone(), md_content));
                                        tracing::debug!("Collected content for packing for URL: {}", url_str);
                                    } else {
                                        tracing::debug!("Content for URL {} is empty, not packing. (Could be actual empty page or handled skip)", url_str);
                                    }
                                } else {
                                     tracing::debug!("URL {} was skipped (e.g. non-HTML), no content to pack.", url_str);
                                }
                                Ok(())
                            }
                            Err(e) => {
                                tracing::warn!("Failed to process and get content for {}: {}", url_str, e.1);
                                Err(e)
                            }
                        }
                    } else {
                        // Process URL normally (writes to file or stdout via process_url_async)
                        // Calls html::process_url_with_retry now
                        html::process_url_with_retry(
                            &url_str,
                            out_path,
                            config_clone.max_retries,
                        )
                        .await
                    };

                    if let Some(pb_instance) = &*pb_clone {
                        pb_instance.inc(1);
                    }
                    result
                }
                Err(e) => {
                    tracing::error!("Failed to parse URL {}: {}", url_str, e);
                    if let Some(pb_instance) = &*pb_clone {
                        pb_instance.inc(1);
                    }
                    Err((url_str, e.into()))
                }
            }
        }
    }))
    .buffer_unordered(concurrency_limit)
    .collect::<Vec<_>>()
    .await;

    if let Some(pb_instance) = &*pb {
        pb_instance.finish_with_message("Processing complete!");
        // pb_instance.finish_and_clear(); // Optionally clear the progress bar
    }

    // Write the packed content to the specified file
    if let Some(path) = pack_path {
        tracing::info!("Writing packed content to {}", path.display());

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tracing::debug!(
                    "Creating parent directory for packed file: {}",
                    parent.display()
                );
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                    tracing::error!(
                        "Failed to create directory {} for packed file: {}",
                        parent.display(),
                        e
                    );
                    // Continue to attempt writing, but log the error.
                }
            }
        }

        let mut packed_file = match tokio::fs::File::create(&path).await {
            Ok(file) => file,
            Err(e) => {
                tracing::error!(
                    "Fatal: Error creating packed file {}: {}",
                    path.display(),
                    e
                );
                // Collect all errors from processing and return them
                return Ok(results.into_iter().filter_map(|r| r.err()).collect());
            }
        };

        // Get the locked packed_content
        let mut content_to_write = packed_content.lock().await;

        // Reorder packed_content to match the original URL order
        if !urls_for_ordering.is_empty() {
            let mut url_to_index = std::collections::HashMap::new();
            for (i, u) in urls_for_ordering.iter().enumerate() {
                url_to_index.insert(u.clone(), i);
            }

            content_to_write.sort_by(|a, b| {
                let a_idx = url_to_index.get(&a.0).unwrap_or(&usize::MAX);
                let b_idx = url_to_index.get(&b.0).unwrap_or(&usize::MAX);
                a_idx.cmp(b_idx)
            });
            tracing::debug!("Packed content reordered according to input URL order.");
        }

        for (url_str, content) in content_to_write.iter() {
            let formatted_entry = format!("# {}\n\n{}\n\n---\n\n", url_str, content);
            if let Err(e) = packed_file.write_all(formatted_entry.as_bytes()).await {
                tracing::error!(
                    "Error writing entry for {} to packed file {}: {}",
                    url_str,
                    path.display(),
                    e
                );
            }
        }
        tracing::info!(
            "Successfully wrote {} entries to packed file {}",
            content_to_write.len(),
            path.display()
        );
    }

    // Collect and return errors
    let mut errors = Vec::new();
    for r in results {
        if let Err(e) = r {
            // Error already logged at source, just collect for summary
            errors.push(e);
        }
    }

    Ok(errors)
}
