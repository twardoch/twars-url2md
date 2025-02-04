use crate::url::Url;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
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
}

/// Process a list of URLs with the given configuration
pub async fn process_urls(
    urls: Vec<String>,
    config: Config,
) -> Result<Vec<(String, anyhow::Error)>> {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::sync::Arc;

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

    let results = stream::iter(urls.into_iter().map(|url| {
        let pb = Arc::clone(&pb);
        let config = config.clone();
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
                    let result = url::process_url_with_retry(
                        &url,
                        out_path,
                        config.verbose,
                        config.max_retries,
                    )
                    .await;
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
