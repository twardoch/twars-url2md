use crate::url::Url;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub mod cli;
mod error;
mod html;
mod markdown;
pub mod url;

pub use cli::Cli;
pub use error::Error;

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
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let pb = Arc::new(pb);
    // Adaptive concurrency based on CPU cores
    let concurrency_limit = thread::available_parallelism()
        .map(|n| n.get() * 2) // 2 tasks per CPU core
        .unwrap_or(10);

    // If pack_file is specified, collect the markdown content
    let should_pack = config.pack_file.is_some();
    let pack_path = config.pack_file.clone();
    let packed_content = if should_pack {
        Arc::new(tokio::sync::Mutex::new(Vec::with_capacity(urls.len())))
    } else {
        Arc::new(tokio::sync::Mutex::new(Vec::new()))
    };

    // Clone the URLs vector before moving it into the stream
    let urls_for_ordering = urls.clone();

    let results = stream::iter(urls.into_iter().map(|url| {
        let pb = Arc::clone(&pb);
        let config = config.clone();
        let packed_content = Arc::clone(&packed_content);
        async move {
            if config.verbose {
                eprintln!("Processing: {}", url);
            }
            match Url::parse(&url) {
                Ok(url_parsed) => {
                    let out_path = if config.single_file
                        && config.has_output
                        && !config.output_base.is_dir()
                    {
                        Some(config.output_base)
                    } else {
                        url::create_output_path(&url_parsed, &config.output_base).ok()
                    };

                    let result = if should_pack {
                        // Process URL and collect content for packing
                        match url::process_url_with_content(
                            &url,
                            out_path,
                            config.verbose,
                            config.max_retries,
                        )
                        .await
                        {
                            Ok(content) => {
                                if let Some(md_content) = content {
                                    let mut content_vec = packed_content.lock().await;
                                    content_vec.push((url.clone(), md_content));
                                }
                                Ok(())
                            }
                            Err(e) => Err(e),
                        }
                    } else {
                        // Process URL normally
                        url::process_url_with_retry(
                            &url,
                            out_path,
                            config.verbose,
                            config.max_retries,
                        )
                        .await
                    };

                    if let Some(pb) = &*pb {
                        pb.inc(1);
                    }
                    result
                }
                Err(e) => {
                    if let Some(pb) = &*pb {
                        pb.inc(1);
                    }
                    Err((url, e.into()))
                }
            }
        }
    }))
    .buffer_unordered(concurrency_limit)
    .collect::<Vec<_>>()
    .await;

    if let Some(pb) = &*pb {
        pb.finish_with_message("Done!");
    }

    // Write the packed content to the specified file
    if let Some(pack_path) = pack_path {
        if config.verbose {
            eprintln!("Writing packed content to {}", pack_path.display());
        }

        if let Some(parent) = pack_path.parent() {
            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                eprintln!(
                    "Warning: Failed to create directory {}: {}",
                    parent.display(),
                    e
                );
            }
        }

        let mut packed_file = match tokio::fs::File::create(&pack_path).await {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating packed file: {}", e);
                return Ok(results.into_iter().filter_map(|r| r.err()).collect());
            }
        };

        // Get the locked packed_content
        let mut content_to_write = packed_content.lock().await;

        // Reorder packed_content to match the original URL order
        let mut url_to_index = std::collections::HashMap::new();
        for (i, url) in urls_for_ordering.iter().enumerate() {
            url_to_index.insert(url.clone(), i);
        }

        content_to_write.sort_by(|a, b| {
            let a_idx = url_to_index.get(&a.0).unwrap_or(&usize::MAX);
            let b_idx = url_to_index.get(&b.0).unwrap_or(&usize::MAX);
            a_idx.cmp(b_idx)
        });

        for (url, content) in content_to_write.iter() {
            if let Err(e) = packed_file
                .write_all(format!("# {}\n\n{}\n\n---\n\n", url, content).as_bytes())
                .await
            {
                eprintln!("Error writing to packed file: {}", e);
            }
        }
    }

    // Process results as before
    let mut errors = Vec::new();
    for r in results {
        match r {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Warning: Failed to process {}: {}", e.0, e.1);
                errors.push(e);
            }
        }
    }

    Ok(errors)
}
