use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set custom panic hook for the main thread
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            eprintln!(
                "Warning: Processing error in {} at line {}: {}",
                location.file(),
                location.line(),
                panic_info
            );
        } else {
            eprintln!("Warning: Processing error occurred: {}", panic_info);
        }
    }));

    // Disable backtrace for cleaner error messages
    std::env::set_var("RUST_BACKTRACE", "0");

    // Parse command-line arguments
    let cli = twars_url2md::cli::Cli::parse_args()?;

    // Collect URLs from all input sources
    let urls = cli.collect_urls()?;

    // Create configuration
    let config = cli.create_config();
    let verbose = config.verbose;

    // Process URLs
    let errors = twars_url2md::process_urls(urls, config).await?;

    // Report summary
    if !errors.is_empty() && verbose {
        eprintln!("\nSummary of failures:");
        for (url, error) in &errors {
            eprintln!("  {} - {}", url, error);
        }
        eprintln!("\n{} URLs failed to process", errors.len());
    }

    Ok(())
}
