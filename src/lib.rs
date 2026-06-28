//! # twars-url2md
//!
//! A fast Rust library and CLI tool that turns web pages into clean Markdown.
//!
//! ## Overview
//!
//! `twars-url2md` grabs web pages using libcurl and turns them into readable Markdown
//! using htmd. It handles multiple URLs at once, retries automatically if a request fails, and can output to a directory or a single packed file.
//!
//! ## Features
//!
//! - **URL Extraction**: Finds URLs in plain text, HTML, and Markdown files
//! - **Concurrent Processing**: Runs tasks in parallel based on your CPU cores
//! - **Error Recovery**: Automatically retries failed downloads with exponential backoff
//! - **Flexible Output**: Saves files in a folder structure or packs them into one file
//! - **Local File Support**: Converts local HTML files just like web URLs
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
//! # Process URLs from a file and save to output directory
//! twars-url2md -i urls.txt -o ./output
//!
//! # Process URLs from stdin
//! echo "https://example.com" | twars-url2md --stdin -o ./output
//!
//! # Combine everything into one big Markdown file
//! twars-url2md -i urls.txt --pack combined.md
//! ```

use crate::url::Url;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
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

// ---------------------------------------------------------------------------
// Crash-safe JSONL progress tracking
// ---------------------------------------------------------------------------

/// Name of the append-only progress file written inside each output directory.
///
/// One JSON object per line: `{"url":"…","status":"done"}`.
/// The file is stored **inside the output directory**, so changing `--output`
/// automatically starts a fresh run — the old file is simply not found.
pub(crate) const PROGRESS_FILE: &str = ".url2md_progress.jsonl";

/// Load the set of already-completed URLs from the progress file in `output_dir`.
///
/// Returns an empty [`HashSet`] when the file is absent or unreadable so that
/// the first run always starts from scratch without requiring pre-existing state.
/// Lines that cannot be parsed (e.g. truncated by a crash) are silently skipped.
fn load_progress(output_dir: &Path) -> HashSet<String> {
    let path = output_dir.join(PROGRESS_FILE);
    let mut done = HashSet::new();
    if let Ok(content) = std::fs::read_to_string(&path) {
        for line in content.lines() {
            if let Some(url) = parse_progress_url(line) {
                done.insert(url.to_string());
            }
        }
    }
    done
}

/// Append a single `{"url":"…","status":"done"}` record to the progress file.
///
/// Opens with `append` mode so concurrent task writes never truncate each
/// other's data.  Failures are logged as warnings and swallowed — progress
/// tracking is best-effort and must never abort a successful conversion.
async fn append_progress(output_dir: &Path, url: &str) {
    use tokio::io::AsyncWriteExt as _;
    let path = output_dir.join(PROGRESS_FILE);
    // Minimal JSON escaping: backslash and double-quote are rare in URLs but valid.
    let escaped = url.replace('\\', "\\\\").replace('"', "\\\"");
    let line = format!("{{\"url\":\"{escaped}\",\"status\":\"done\"}}\n");
    match tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await
    {
        Ok(mut f) => {
            if let Err(e) = f.write_all(line.as_bytes()).await {
                tracing::warn!("Progress write failed for {}: {}", url, e);
            }
        }
        Err(e) => tracing::warn!("Cannot open progress file {:?}: {}", path, e),
    }
}

/// Extract the URL field from one JSONL progress line.
///
/// Handles the minimal format `{"url":"…","status":"done"}` produced by
/// [`append_progress`].  Returns `None` if the line is malformed so that a
/// truncated write during a crash never blocks future runs.
fn parse_progress_url(line: &str) -> Option<&str> {
    const MARKER: &str = "\"url\":\"";
    let start = line.find(MARKER)? + MARKER.len();
    let rest = &line[start..];
    // Walk characters to find the closing quote, honouring backslash escapes.
    let mut end = None;
    let mut chars = rest.char_indices();
    while let Some((i, ch)) = chars.next() {
        if ch == '\\' {
            chars.next(); // skip the escaped character
            continue;
        }
        if ch == '"' {
            end = Some(i);
            break;
        }
    }
    let end = end?;
    let url = &rest[..end];
    if url.is_empty() {
        None
    } else {
        Some(url)
    }
}

/// Settings for turning URLs into Markdown.
///
/// Holds everything needed to run a batch of URLs:
/// where to save files, how many times to retry failures, and whether to print verbose logs.
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

/// Process a list of URLs and convert them to Markdown.
///
/// The core engine. It grabs pages concurrently,
/// recovers from errors automatically, and writes the results based on your `Config`.
///
///
/// # Arguments
///
/// * `urls` - A vector of URLs to process
/// * `config` - Configuration options for processing
///
/// # Returns
///
/// Returns a list of URLs that failed along with their error messages.
/// If the list is empty, everything worked.
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

    // ------------------------------------------------------------------
    // Crash-safe resumability: track completed URLs in a per-output-dir
    // JSONL file.  Storing the file *inside* the output directory means
    // switching `--output` automatically starts a fresh run — the old
    // progress file is simply not found in the new directory.
    // ------------------------------------------------------------------
    let track_progress = config.has_output && !config.single_file && config.pack_file.is_none();
    let progress_dir: Option<Arc<PathBuf>> = if track_progress {
        let dir = config.output_base.clone();
        // Create the output dir early so the progress file has a place to live.
        if let Err(e) = std::fs::create_dir_all(&dir) {
            tracing::warn!("Cannot create output dir for progress tracking: {}", e);
        }
        Some(Arc::new(dir))
    } else {
        None
    };

    // Load completed URLs from a previous run and skip them.
    let already_done: HashSet<String> = progress_dir
        .as_deref()
        .map(|p| load_progress(p.as_path()))
        .unwrap_or_default();
    let skip_count = urls
        .iter()
        .filter(|u| already_done.contains(u.as_str()))
        .count();
    if skip_count > 0 {
        tracing::info!(
            "Resuming: skipping {} already-completed URL(s) (from {:?}).",
            skip_count,
            progress_dir.as_deref().map(|p| p.join(PROGRESS_FILE))
        );
    }
    let urls: Vec<String> = urls
        .into_iter()
        .filter(|u| !already_done.contains(u.as_str()))
        .collect();

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
        let progress_dir_clone = progress_dir.clone();
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

                    // Record successful completion in the JSONL progress file so
                    // the next run can skip this URL even if the process crashes later.
                    if result.is_ok() {
                        if let Some(ref pdir) = progress_dir_clone {
                            append_progress(pdir.as_path(), &url_str).await;
                        }
                    }

                    if let Some(pb_instance) = &*pb_clone {
                        pb_instance.inc(1);
                    }
                    result
                }
                Err(e) => {
                    let helpful_error = anyhow::anyhow!(
                        "Invalid URL '{}': {}\n\
                         Tip: URLs must include a protocol (http:// or https://)\n\
                         Example: https://example.com/page",
                        url_str,
                        e
                    );
                    tracing::error!("Failed to parse URL {}: {}", url_str, e);
                    if let Some(pb_instance) = &*pb_clone {
                        pb_instance.inc(1);
                    }
                    Err((url_str, helpful_error))
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
                        "Failed to create directory {} for packed file: {}\n\
                         Possible causes: insufficient permissions, read-only filesystem, or disk full\n\
                         Try: Check directory permissions or specify a different output location with --pack",
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
                    "Fatal: Cannot create packed file {}: {}\n\
                     Possible causes:\n\
                     - Permission denied: You don't have write access to this location\n\
                     - Disk full: No space left on device\n\
                     - Read-only filesystem: Target location is not writable\n\
                     - Path too long: File path exceeds system limits\n\
                     Try:\n\
                     - Use a different output location: --pack ~/output.md\n\
                     - Check available disk space: df -h\n\
                     - Verify directory permissions: ls -la $(dirname {})",
                    path.display(),
                    e,
                    path.display()
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // --- parse_progress_url ---

    #[test]
    fn test_parse_progress_url_valid() {
        let line = r#"{"url":"https://example.com","status":"done"}"#;
        assert_eq!(
            parse_progress_url(line),
            Some("https://example.com"),
            "should extract the URL field"
        );
    }

    #[test]
    fn test_parse_progress_url_with_path_and_query() {
        let line = r#"{"url":"https://example.com/path?q=1#frag","status":"done"}"#;
        assert_eq!(
            parse_progress_url(line),
            Some("https://example.com/path?q=1#frag"),
        );
    }

    #[test]
    fn test_parse_progress_url_invalid_lines() {
        assert_eq!(
            parse_progress_url("not json"),
            None,
            "plain text must yield None"
        );
        assert_eq!(parse_progress_url(""), None, "empty line must yield None");
        assert_eq!(
            parse_progress_url(r#"{"status":"done"}"#),
            None,
            "missing url field must yield None"
        );
        assert_eq!(
            parse_progress_url(r#"{"url":"","status":"done"}"#),
            None,
            "empty url must yield None"
        );
    }

    // --- load_progress ---

    #[test]
    fn test_load_progress_nonexistent_dir_returns_empty() {
        // A path that will never exist — must not panic and must return empty.
        let dir = std::path::Path::new("/tmp/__twars_url2md_nonexistent_testdir_xyzzy__");
        let done = load_progress(dir);
        assert!(
            done.is_empty(),
            "missing progress file should yield empty set"
        );
    }

    #[test]
    fn test_load_progress_reads_completed_urls() {
        let dir = TempDir::new().unwrap();
        let progress_file = dir.path().join(PROGRESS_FILE);
        std::fs::write(
            &progress_file,
            "{\"url\":\"https://example.com\",\"status\":\"done\"}\n\
             {\"url\":\"https://rust-lang.org\",\"status\":\"done\"}\n",
        )
        .unwrap();

        let done = load_progress(dir.path());
        assert!(
            done.contains("https://example.com"),
            "first URL should be loaded"
        );
        assert!(
            done.contains("https://rust-lang.org"),
            "second URL should be loaded"
        );
        assert_eq!(done.len(), 2, "exactly two URLs should be loaded");
    }

    #[test]
    fn test_load_progress_skips_malformed_lines() {
        let dir = TempDir::new().unwrap();
        let progress_file = dir.path().join(PROGRESS_FILE);
        // Truncated line (simulates a crash mid-write) followed by a good line.
        std::fs::write(
            &progress_file,
            "{\"url\":\"https://truncated.com\",\"sta\n\
             {\"url\":\"https://good.com\",\"status\":\"done\"}\n",
        )
        .unwrap();

        let done = load_progress(dir.path());
        // The truncated line lacks a closing quote for the URL — it must be skipped.
        assert!(
            done.contains("https://good.com"),
            "valid line should be loaded"
        );
    }

    // --- end-to-end progress file behaviour (requires tokio) ---

    #[tokio::test]
    async fn test_progress_file_created_after_processing() {
        let html_dir = TempDir::new().unwrap();
        let html_file = html_dir.path().join("prog.html");
        std::fs::write(
            &html_file,
            "<html><body><h1>Progress Test</h1></body></html>",
        )
        .unwrap();
        let file_url = format!("file://{}", html_file.display());

        let output_dir = TempDir::new().unwrap();
        let config = Config {
            verbose: false,
            max_retries: 0,
            output_base: output_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(vec![file_url.clone()], config).await.unwrap();
        assert!(errors.is_empty(), "local-file processing should succeed");

        let progress_path = output_dir.path().join(PROGRESS_FILE);
        assert!(
            progress_path.exists(),
            "progress file should be created in output dir"
        );

        let content = std::fs::read_to_string(&progress_path).unwrap();
        assert!(
            content.contains(&file_url),
            "progress file should record the processed URL"
        );
    }

    /// Verifies the crash-safe restart contract: when `--output` points to a
    /// **different** directory the old progress file is not found, so processing
    /// starts fresh and creates an independent progress file in the new location.
    #[tokio::test]
    async fn test_progress_file_restarts_on_output_dir_change() {
        let html_dir = TempDir::new().unwrap();
        let html_file = html_dir.path().join("resume.html");
        std::fs::write(&html_file, "<html><body><h1>Resume Test</h1></body></html>").unwrap();
        let file_url = format!("file://{}", html_file.display());

        // First run — output dir1.
        let dir1 = TempDir::new().unwrap();
        let config1 = Config {
            verbose: false,
            max_retries: 0,
            output_base: dir1.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };
        let errors1 = process_urls(vec![file_url.clone()], config1).await.unwrap();
        assert!(errors1.is_empty(), "first run should succeed");

        let progress1 = dir1.path().join(PROGRESS_FILE);
        assert!(progress1.exists(), "progress file should exist in dir1");
        let lines_in_dir1 = std::fs::read_to_string(&progress1).unwrap().lines().count();
        assert_eq!(lines_in_dir1, 1, "dir1 progress should have one entry");

        // Second run — different output dir2: must start fresh.
        let dir2 = TempDir::new().unwrap();
        let config2 = Config {
            verbose: false,
            max_retries: 0,
            output_base: dir2.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };
        let errors2 = process_urls(vec![file_url.clone()], config2).await.unwrap();
        assert!(errors2.is_empty(), "second run (new dir) should succeed");

        // dir2 must have its own fresh progress file.
        let progress2 = dir2.path().join(PROGRESS_FILE);
        assert!(
            progress2.exists(),
            "progress file should be created in dir2"
        );

        // dir1 progress must be unchanged — dir2 run must not touch it.
        let lines_in_dir1_after = std::fs::read_to_string(&progress1).unwrap().lines().count();
        assert_eq!(
            lines_in_dir1_after, lines_in_dir1,
            "dir1 progress file must be unchanged after dir2 run"
        );
    }
}
