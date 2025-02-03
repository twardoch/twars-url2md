use crate::url::Url;
use anyhow::Result;
use std::path::PathBuf;

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
    use futures::future::join_all;
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
    let mut errors = Vec::new();
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let pb = Arc::clone(&pb);
            let config = config.clone();

            tokio::spawn(async move {
                if config.verbose {
                    eprintln!("Processing: {}", url);
                }

                let result = match Url::parse(&url) {
                    Ok(url_parsed) => {
                        let out_path = if config.single_file
                            && config.has_output
                            && !config.output_base.is_dir()
                        {
                            Some(config.output_base)
                        } else {
                            match url::create_output_path(&url_parsed, &config.output_base) {
                                Ok(path) => Some(path),
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Failed to create output path for {}: {}",
                                        url, e
                                    );
                                    None
                                }
                            }
                        };
                        url::process_url_with_retry(
                            &url,
                            out_path,
                            config.verbose,
                            config.max_retries,
                        )
                        .await
                    }
                    Err(e) => Err((url, e.into())),
                };

                if let Some(pb) = &*pb {
                    pb.inc(1);
                }

                result
            })
        })
        .collect();

    let results = join_all(tasks).await;
    if let Some(pb) = &*pb {
        pb.finish_with_message("Done!");
    }

    for result in results {
        match result {
            Ok(Ok(())) => {}
            Ok(Err((url, error))) => {
                eprintln!("Warning: Failed to process {}: {}", url, error);
                errors.push((url, error));
            }
            Err(e) => {
                eprintln!("Warning: Task failed: {}", e);
                errors.push(("Unknown URL".to_string(), e.into()));
            }
        }
    }

    Ok(errors)
}
