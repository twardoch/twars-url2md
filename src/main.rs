use anyhow::{Context, Result};
use clap::Parser;
use futures::future::join_all;
use html5ever::serialize::{serialize, SerializeOpts};
use indicatif::{ProgressBar, ProgressStyle};
use markup5ever_rcdom::{Handle, NodeData, SerializableHandle};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

const USER_AGENT_STRING: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:123.0) Gecko/20100101 Firefox/123.0";

/// Convert web pages to Markdown using Monolith and htmd
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// URLs to process
    #[arg(required_unless_present_any = ["input_file", "stdin"])]
    urls: Vec<String>,

    /// File containing URLs (one per line)
    #[arg(short = 'f', long)]
    input_file: Option<PathBuf>,

    /// Read URLs from stdin (space or newline separated)
    #[arg(long)]
    stdin: bool,

    /// Output file (for single URL) or directory (for multiple URLs)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn create_output_path(url: &Url, base_dir: &Path) -> Result<PathBuf> {
    // Get host
    let host = url.host_str().unwrap_or("unknown");
    let mut path_components = vec![host];

    // Split path into components, ignoring empty parts
    let path_segments: Vec<_> = url.path().split('/').filter(|s| !s.is_empty()).collect();

    if !path_segments.is_empty() {
        // Add all path components except the last one
        path_components.extend(&path_segments[..path_segments.len() - 1]);

        // Create the directory structure
        let dir_path = base_dir.join(path_components.join("/"));
        fs::create_dir_all(&dir_path)
            .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;

        // Get the last component for the filename
        if let Some(last) = path_segments.last() {
            // If the URL ends with a slash or the last segment is empty, use index.md
            let filename = if url.path().ends_with('/') || last.is_empty() {
                "index.md".to_string()
            } else {
                // Remove any file extension and add .md
                if let Some(stem) = Path::new(last).file_stem() {
                    format!("{}.md", stem.to_string_lossy())
                } else {
                    format!("{}.md", last)
                }
            };
            Ok(dir_path.join(filename))
        } else {
            Ok(dir_path.join("index.md"))
        }
    } else {
        // No path segments, just use the host directory
        let dir_path = base_dir.join(host);
        fs::create_dir_all(&dir_path)
            .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;
        Ok(dir_path.join("index.md"))
    }
}

fn remove_styles(node: &Handle) {
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            // Remove style tags
            if name.local.as_ref() == "style" {
                node.children.borrow_mut().clear();
            }

            // Remove style attributes
            let mut attrs = attrs.borrow_mut();
            attrs.retain(|attr| attr.name.local.as_ref() != "style");
        }
        _ => {}
    }

    // Process children
    for child in node.children.borrow().iter() {
        remove_styles(child);
    }
}

async fn process_url_async(url: String, output_path: Option<PathBuf>) -> Result<()> {
    // Parse the URL
    let url_parsed = Url::parse(&url).with_context(|| format!("Invalid URL: {}", url))?;

    // Create a client for fetching assets
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STRING));
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .context("Failed to create HTTP client")?;

    // Fetch the initial HTML
    let response = client
        .get(url_parsed.as_str())
        .send()
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/html; charset=utf-8");

    let (_, charset, _) = monolith::utils::parse_content_type(content_type);

    let html_bytes = response
        .bytes()
        .await
        .with_context(|| format!("Failed to read response body from URL: {}", url))?;

    // Create DOM from HTML
    let dom = monolith::html::html_to_dom(&html_bytes.to_vec(), charset);
    let doc = dom.document.clone();

    // Remove all styles from the document
    remove_styles(&doc);

    // Process all assets in the document
    let mut html_buf = Vec::new();
    serialize(
        &mut html_buf,
        &SerializableHandle::from(doc),
        SerializeOpts::default(),
    )
    .with_context(|| "Failed to serialize HTML")?;

    // Convert HTML to string
    let html = String::from_utf8(html_buf).with_context(|| "Failed to convert HTML to UTF-8")?;

    // Convert to Markdown using htmd
    let markdown = htmd::convert(&html)
        .with_context(|| format!("Failed to convert HTML to Markdown for URL: {}", url))?;

    // Write output
    if let Some(path) = output_path {
        fs::write(&path, markdown)
            .with_context(|| format!("Failed to write to file: {}", path.display()))?;
        eprintln!("Created: {}", path.display());
    } else {
        println!("{}", markdown);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI args but don't show the usual cargo output unless verbose
    let args = std::env::args().collect::<Vec<_>>();
    let cli = if args.iter().any(|arg| arg == "-v" || arg == "--verbose") {
        Cli::parse()
    } else {
        Cli::try_parse_from(args).map_err(|e| {
            // Only show usage errors, not the fancy clap output
            anyhow::anyhow!(
                "{}",
                e.render()
                    .to_string()
                    .lines()
                    .next()
                    .unwrap_or("Invalid usage")
            )
        })?
    };

    // Collect all URLs
    let mut urls = cli.urls;

    // Add URLs from file if specified
    if let Some(input_file) = cli.input_file {
        let content = fs::read_to_string(&input_file)
            .with_context(|| format!("Failed to read input file: {}", input_file.display()))?;
        urls.extend(
            content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(String::from),
        );
    }

    // Add URLs from stdin if requested
    if cli.stdin {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        while let Some(line) = lines.next() {
            let line = line?;
            urls.extend(
                line.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(String::from),
            );
        }
    }

    if urls.is_empty() {
        anyhow::bail!("No URLs provided. Use positional arguments, -f/--input-file, or --stdin");
    }

    // Remove duplicates while preserving order
    urls.sort_unstable();
    urls.dedup();

    let single_file = urls.len() == 1;
    let output_base = cli
        .output
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let has_output = cli.output.is_some();

    // Create progress bar
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

    // Process URLs in parallel
    let pb = Arc::new(pb);
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let pb = Arc::clone(&pb);
            let output_base = output_base.clone();
            let verbose = cli.verbose;

            tokio::spawn(async move {
                if verbose {
                    eprintln!("Processing: {}", url);
                }
                let url_parsed = Url::parse(&url)?;
                let out_path = if single_file && has_output && !output_base.is_dir() {
                    Some(output_base)
                } else {
                    Some(create_output_path(&url_parsed, &output_base)?)
                };
                let result = process_url_async(url, out_path).await;

                if let Some(pb) = &*pb {
                    pb.inc(1);
                }
                result
            })
        })
        .collect();

    // Wait for all tasks to complete
    let results = join_all(tasks).await;
    if let Some(pb) = &*pb {
        pb.finish_with_message("Done!");
    }

    // Check for errors
    for result in results {
        result??;
    }

    Ok(())
}
