use anyhow::{Context, Result};
use monolith::opts::Options;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::markdown;

/// Log an error message if verbose mode is enabled
fn log_error(msg: &str, verbose: bool) {
    if verbose {
        eprintln!("Warning: {}", msg);
    }
}

/// Process a URL by downloading its content and converting to Markdown
pub async fn process_url_async(
    url: &str,
    output_path: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    // Skip non-HTML URLs
    if url.ends_with(".jpg")
        || url.ends_with(".jpeg")
        || url.ends_with(".png")
        || url.ends_with(".gif")
        || url.ends_with(".svg")
        || url.ends_with(".webp")
        || url.ends_with(".pdf")
        || url.ends_with(".mp4")
        || url.ends_with(".webm")
    {
        log_error(&format!("Skipping non-HTML URL: {}", url), verbose);
        return Ok(());
    }

    let client = create_http_client()?;

    // Pre-allocate a reasonably sized cache with specific types
    static CACHE_CAPACITY: usize = 1024;
    let _cache: HashMap<String, Vec<u8>> = HashMap::with_capacity(CACHE_CAPACITY);

    let html = match fetch_html(&client, url).await {
        Ok(html) => html,
        Err(e) => {
            log_error(
                &format!(
                    "Error fetching HTML from {}: {}. Using fallback processing.",
                    url, e
                ),
                verbose,
            );
            // Try to get raw HTML as fallback
            client.get(url).send().await?.text().await?
        }
    };

    let markdown = match markdown::convert_html_to_markdown(&html) {
        Ok(md) => md,
        Err(e) => {
            log_error(
                &format!(
                    "Error converting to Markdown: {}. Using simplified conversion.",
                    e
                ),
                verbose,
            );
            // Fallback to simpler conversion if htmd fails
            html.replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<br />", "\n")
                .replace("<p>", "\n\n")
                .replace("</p>", "")
        }
    };

    match output_path {
        Some(path) => {
            // Use async file operations for better I/O performance
            if let Some(parent) = path.parent() {
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                    log_error(
                        &format!("Failed to create directory {}: {}", parent.display(), e),
                        verbose,
                    );
                }
            }
            tokio::fs::write(&path, markdown)
                .await
                .with_context(|| format!("Failed to write to file: {}", path.display()))?;
            if verbose {
                eprintln!("Created: {}", path.display());
            }
        }
        None => println!("{}", markdown),
    }

    Ok(())
}

/// Create an HTTP client with appropriate headers and optimized settings
fn create_http_client() -> Result<Client> {
    let mut headers = HeaderMap::with_capacity(4); // Pre-allocate for known headers
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(crate::USER_AGENT_STRING),
    );

    Client::builder()
        .default_headers(headers)
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")
}

/// Fetch HTML content from a URL using monolith with specified options
async fn fetch_html(client: &Client, url: &str) -> Result<String> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;

    // Check content type
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/html; charset=utf-8");

    // Skip non-HTML content
    if !content_type.contains("text/html") {
        return Err(anyhow::anyhow!("Not an HTML page: {}", content_type));
    }

    let (_, charset, _) = monolith::utils::parse_content_type(content_type);

    let html_bytes = response
        .bytes()
        .await
        .with_context(|| format!("Failed to read response body from URL: {}", url))?;

    // First try simple HTML cleanup without Monolith
    let simple_html = String::from_utf8_lossy(&html_bytes)
        .replace("<!--", "")
        .replace("-->", "")
        .replace("<script", "<!--<script")
        .replace("</script>", "</script>-->")
        .replace("<style", "<!--<style")
        .replace("</style>", "</style>-->");

    // Try Monolith processing in a blocking task
    let options = Options {
        no_video: true,
        isolate: true,
        no_js: true,
        no_css: true,
        base_url: Some(url.to_string()),
        ignore_errors: true,
        no_fonts: true,
        no_images: true,
        insecure: true,
        no_metadata: true,
        silent: true,
        no_frames: true,       // Disable iframe processing
        unwrap_noscript: true, // Handle noscript content
        ..Default::default()
    };

    let document_url =
        reqwest::Url::parse(url).with_context(|| format!("Failed to parse URL: {}", url))?;
    let html_bytes = Arc::new(html_bytes.to_vec());
    let charset = charset.clone();

    // Try to process with Monolith, but fall back to simple HTML if anything fails
    let processed_html = tokio::task::spawn_blocking({
        let html_bytes = Arc::clone(&html_bytes);
        let simple_html = simple_html.clone();
        move || {
            // Set a thread-local panic hook that returns control instead of aborting
            let prev_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Try to create DOM with error handling
                let dom = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    monolith::html::html_to_dom(&html_bytes, charset.clone())
                })) {
                    Ok(dom) => dom,
                    Err(_) => return simple_html.as_bytes().to_vec(),
                };

                // Try to process assets with error handling
                let mut cache = HashMap::new();
                let blocking_client = reqwest::blocking::Client::new();

                // Wrap all Monolith operations in catch_unwind
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    monolith::html::walk_and_embed_assets(
                        &mut cache,
                        &blocking_client,
                        &document_url,
                        &dom.document,
                        &options,
                    );

                    monolith::html::serialize_document(dom, charset, &options)
                }));

                match result {
                    Ok(html) => html,
                    Err(_) => simple_html.as_bytes().to_vec(),
                }
            }));

            // Restore the previous panic hook
            std::panic::set_hook(prev_hook);

            // Return result or fallback to simple HTML
            match result {
                Ok(html) => html,
                Err(_) => simple_html.as_bytes().to_vec(),
            }
        }
    })
    .await
    .unwrap_or_else(|_| simple_html.as_bytes().to_vec());

    String::from_utf8(processed_html)
        .map_err(|e| anyhow::anyhow!("Failed to convert processed HTML to UTF-8: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_html_processing() -> Result<()> {
        // Sample HTML with various elements that should be processed
        let html_content = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Test Page</title>
                <style>body { color: red; }</style>
                <script>console.log('test');</script>
                <link rel="stylesheet" href="style.css">
            </head>
            <body>
                <h1>Main Heading</h1>
                <h2>Sub Heading</h2>
                <ul>
                    <li>List item 1</li>
                    <li>List item 2</li>
                </ul>
                <a href="https://example.com">A link</a>
                <img src="image.jpg" />
                <video src="video.mp4"></video>
                <iframe src="frame.html"></iframe>
                <font face="Arial">Font text</font>
            </body>
            </html>
        "#;

        // Create monolith options with specified flags
        let options = Options {
            no_video: true,
            isolate: true,
            no_js: true,
            no_css: true,
            base_url: Some("https://example.com".to_string()),
            ignore_errors: true,
            no_fonts: true,
            no_images: true,
            insecure: true,
            no_metadata: true,
            silent: true,
            ..Default::default()
        };

        // Create DOM from HTML
        let dom =
            monolith::html::html_to_dom(&html_content.as_bytes().to_vec(), "UTF-8".to_string());

        // Process assets and embed them
        let mut cache = HashMap::new();
        let client = reqwest::blocking::Client::new();
        let document_url = reqwest::Url::parse("https://example.com").unwrap();
        monolith::html::walk_and_embed_assets(
            &mut cache,
            &client,
            &document_url,
            &dom.document,
            &options,
        );

        // Serialize back to HTML
        let processed_html = monolith::html::serialize_document(dom, "UTF-8".to_string(), &options);
        let processed_html = String::from_utf8(processed_html).unwrap();

        // Convert to markdown
        let markdown = markdown::convert_html_to_markdown(&processed_html)?;

        // Verify content structure is preserved
        assert!(markdown.contains("# Main Heading"));
        assert!(markdown.contains("## Sub Heading"));
        assert!(markdown.contains("*   List item 1"));
        assert!(markdown.contains("*   List item 2"));
        assert!(markdown.contains("[A link](https://example.com)"));

        // Verify that elements are properly handled according to options
        assert!(!processed_html.contains("src=\"video.mp4\"")); // no_video
        assert!(!processed_html.contains("src=\"image.jpg\"")); // no_images
        assert!(!processed_html.contains("href=\"style.css\"")); // no_css
        assert!(!processed_html.contains("src=\"frame.html\"")); // isolate
        assert!(!processed_html.contains("console.log")); // no_js

        Ok(())
    }

    #[test]
    fn test_markdown_output() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let output_path = temp_dir.path().join("test.md");

        // Simple HTML content
        let html = "<h1>Test Content</h1>";

        // Process HTML directly
        let markdown = markdown::convert_html_to_markdown(html)?;

        // Write to output file
        fs::write(&output_path, &markdown)?;

        // Verify file exists and contains expected content
        assert!(output_path.exists());
        let output_content = fs::read_to_string(&output_path)?;
        assert!(output_content.contains("# Test Content"));

        Ok(())
    }
}
