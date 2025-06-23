use anyhow::Result;
use std::panic;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn run() -> Result<()> {
    // Disable backtrace for cleaner error messages by default, can be overridden by RUST_BACKTRACE=1
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "0");
    }

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

    tracing::debug!("CLI args parsed and config created: {:?}", config);

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
    // Set custom panic hook that prevents abort
    // TODO: Re-evaluate this after error handling refinement. Goal is to avoid panics.
    panic::set_hook(Box::new(|panic_info| {
        // Use tracing for panic information if available, otherwise eprintln
        // Check if a tracer is installed, otherwise tracing::event! will panic
        if tracing::dispatcher::has_been_set() {
            if let Some(location) = panic_info.location() {
                tracing::error!(
                    "Panic occurred in {} at line {}: {}",
                    location.file(),
                    location.line(),
                    panic_info
                );
            } else {
                tracing::error!("Panic occurred: {}", panic_info);
            }
        } else {
            if let Some(location) = panic_info.location() {
                eprintln!(
                    "PANIC: Processing error in {} at line {}: {}",
                    location.file(),
                    location.line(),
                    panic_info
                );
            } else {
                eprintln!("PANIC: Processing error occurred: {}", panic_info);
            }
        }
    }));

    // Run the program in a catch_unwind to prevent unwinding across FFI boundaries
    // TODO: Re-evaluate this after error handling refinement. // This is the re-evaluation.
    // The run() function is expected to handle its errors and return a Result.
    // Panics from dependencies like monolith are caught internally.
    // Thus, catch_unwind around run() is no longer strictly necessary.
    if let Err(e) = run() {
        // Use tracing for error reporting if available
        if tracing::dispatcher::has_been_set() {
            tracing::error!("Application error: {:?}", e);
        } else {
            eprintln!("Error: {:?}", e);
        }
        std::process::exit(1);
    }
    // If run() completes without error, exit(0) is implicit.
    // If run() itself panics (which it shouldn't for recoverable errors),
    // the panic_hook will log it, and the process will abort.
}
