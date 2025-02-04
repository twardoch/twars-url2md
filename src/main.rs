use anyhow::Result;
use std::panic;

fn run() -> Result<()> {
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
    let rt = tokio::runtime::Runtime::new()?;
    let errors = rt.block_on(twars_url2md::process_urls(urls, config))?;

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

fn main() {
    // Set custom panic hook that prevents abort
    panic::set_hook(Box::new(|panic_info| {
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

    // Run the program in a catch_unwind to prevent unwinding across FFI boundaries
    let result = panic::catch_unwind(|| {
        if let Err(e) = run() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    });

    if result.is_err() {
        std::process::exit(1);
    }
}
