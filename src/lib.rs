use crate::url::Url;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub mod cli;
// mod error; // Removed
mod html;
mod markdown;
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
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:123.0) Gecko/20100101 Firefox/123.0";

/// Configuration for URL processing
#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub max_retries: u32,
    pub output_base: PathBuf,
    pub single_file: bool,
    pub has_output: bool,
    pub pack_file: Option<PathBuf>,
}

/// Process a list of URLs with the given configuration
pub async fn process_urls(
    urls: Vec<String>,
    config: Config,
) -> Result<Vec<(String, anyhow::Error)>> {
    use indicatif::{ProgressBar, ProgressStyle};
    use tokio::io::AsyncWriteExt;

    let pb = if urls.len() > 1 {
        let pb = ProgressBar::new(urls.len() as u64);
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .map_err(|e| tracing::warn!("Failed to create progress bar template: {}. Using default style.", e))
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
    let urls_for_ordering = if should_pack { urls.clone() } else { Vec::new() };

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
                tracing::debug!("Creating parent directory for packed file: {}", parent.display());
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
                tracing::error!("Fatal: Error creating packed file {}: {}", path.display(), e);
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
                tracing::error!("Error writing entry for {} to packed file {}: {}", url_str, path.display(), e);
            }
        }
        tracing::info!("Successfully wrote {} entries to packed file {}", content_to_write.len(), path.display());
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
