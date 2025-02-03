use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
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
