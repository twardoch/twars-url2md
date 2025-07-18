//! twars-url2md binary - CLI for converting web pages to Markdown.
//!
//! Usage:
//! ```bash
//! twars-url2md -i urls.txt -o ./output
//! echo "https://example.com" | twars-url2md --stdin
//! twars-url2md -i urls.txt --pack combined.md
//! ```

use anyhow::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn run() -> Result<()> {
    // Parse command-line arguments
    let cli = twars_url2md::cli::Cli::parse_args()?;

    // Create configuration
    let config = cli.create_config();

    // Initialize tracing subscriber
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| {
        if config.verbose {
            EnvFilter::try_new("info,twars_url2md=debug")
        } else {
            EnvFilter::try_new("info")
        }
    })?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter_layer)
        .init();

    // Collect URLs from all input sources
    let urls = cli.collect_urls()?;
    tracing::info!("Collected {} URLs to process.", urls.len());

    // Process URLs
    let rt = tokio::runtime::Runtime::new()?;
    let errors = rt.block_on(twars_url2md::process_urls(urls, config))?;

    // Report summary
    if !errors.is_empty() {
        tracing::warn!("\nSummary of failures:");
        for (url, error) in &errors {
            tracing::warn!("  {} - {}", url, error);
        }
        tracing::warn!("\n{} URLs failed to process", errors.len());
    } else {
        tracing::info!("All URLs processed successfully.");
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
