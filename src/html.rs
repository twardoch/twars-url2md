use anyhow::{Context, Result};
use monolith::cache::Cache;
use monolith::core::Options;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::markdown;

/// Internal helper to fetch HTML and convert to Markdown for a given URL.
/// Returns Ok(None) if URL is skipped (e.g., non-HTML).
/// Returns Ok(Some(String)) with Markdown content if successful.
/// Returns Err if fetching or conversion fails.
async fn get_markdown_for_url(url: &str) -> Result<Option<String>> {
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
        tracing::debug!("Skipping non-HTML URL (get_markdown_for_url): {}", url);
        return Ok(None);
    }

    tracing::debug!(
        "Creating HTTP client for URL (get_markdown_for_url): {}",
        url
    );
    let client = create_http_client()?;

    tracing::debug!("Fetching HTML for URL (get_markdown_for_url): {}", url);
    let html = match fetch_html(&client, url).await {
        Ok(html_content) => html_content,
        Err(e) => {
            tracing::warn!(
                "Error fetching HTML from {} (get_markdown_for_url): {}. Using fallback processing.",
                url,
                e
            );
            // Try to get raw HTML as fallback
            client.get(url).send().await?.text().await?
        }
    };

    tracing::debug!(
        "Converting HTML to Markdown for URL (get_markdown_for_url): {}",
        url
    );
    match markdown::convert_html_to_markdown(&html) {
        Ok(md) => Ok(Some(md)),
        Err(e) => {
            tracing::warn!(
                "Error converting to Markdown for URL {} (get_markdown_for_url): {}. Using simplified conversion.",
                url,
                e
            );
            // Fallback to simpler conversion if htmd fails
            let simplified_md = html
                .replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<br />", "\n")
                .replace("<p>", "\n\n")
                .replace("</p>", "");
            Ok(Some(simplified_md))
        }
    }
}

/// Process a URL by downloading its content and converting to Markdown
pub async fn process_url_async(
    url: &str,
    output_path: Option<PathBuf>,
    // verbose: bool, // verbose is now handled by tracing
) -> Result<()> {
    match get_markdown_for_url(url).await? {
        Some(markdown_content) => {
            if let Some(path) = output_path {
                tracing::debug!(
                    "Writing Markdown to file (process_url_async): {}",
                    path.display()
                );
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        tracing::debug!(
                            "Creating parent directory (process_url_async): {}",
                            parent.display()
                        );
                        if let Err(e) = tokio::fs::create_dir_all(parent).await {
                            tracing::warn!(
                                "Failed to create directory {} (process_url_async): {}",
                                parent.display(),
                                e
                            );
                        }
                    }
                }
                tokio::fs::write(&path, &markdown_content) // Pass by reference
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to write to file (process_url_async): {}",
                            path.display()
                        )
                    })?;
                tracing::info!("Created (process_url_async): {}", path.display());
            } else {
                tracing::debug!(
                    "Printing Markdown to stdout for URL (process_url_async): {}",
                    url
                );
                println!("{}", markdown_content);
            }
        }
        None => {
            // URL was skipped (e.g. non-HTML), already logged by get_markdown_for_url
            tracing::debug!("URL skipped, no action needed (process_url_async): {}", url);
        }
    }
    Ok(())
}

/// Process a URL by downloading its content and converting to Markdown
/// Returns the Markdown content
pub async fn process_url_with_content(
    url: &str,
    output_path: Option<PathBuf>,
    // verbose: bool, // verbose is now handled by tracing
) -> Result<String> {
    match get_markdown_for_url(url).await? {
        Some(markdown_content) => {
            if let Some(path) = output_path {
                tracing::debug!(
                    "Writing Markdown to file (process_url_with_content): {}",
                    path.display()
                );
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        tracing::debug!(
                            "Creating parent directory (process_url_with_content): {}",
                            parent.display()
                        );
                        if let Err(e) = tokio::fs::create_dir_all(parent).await {
                            tracing::warn!(
                                "Failed to create directory {} (process_url_with_content): {}",
                                parent.display(),
                                e
                            );
                        }
                    }
                }
                tokio::fs::write(&path, &markdown_content) // Pass by reference
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to write to file (process_url_with_content): {}",
                            path.display()
                        )
                    })?;
                tracing::info!("Created (process_url_with_content): {}", path.display());
            }
            Ok(markdown_content)
        }
        None => {
            // URL was skipped
            tracing::debug!(
                "URL skipped, returning empty string (process_url_with_content): {}",
                url
            );
            Ok(String::new())
        }
    }
}

/// Create an HTTP client with appropriate headers and optimized settings
fn create_http_client() -> Result<Client> {
    let mut headers = HeaderMap::with_capacity(6); // Pre-allocate for known headers
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(crate::USER_AGENT_STRING),
    );
    headers.insert(
        "Accept",
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        ),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );

    Client::builder()
        .default_headers(headers)
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(30))
        .timeout(Duration::from_secs(60)) // Overall request deadline (headers + body)
        .connect_timeout(Duration::from_secs(20)) // TCP/TLS handshake deadline
        // Allow HTTP/2 (default). Some CDNs only serve large pages efficiently over h2
        // and may stall or throttle h1 connections, which manifested as 30 s time-outs
        // on `helpx.adobe.com`.
        .build()
        .context("Failed to create HTTP client")
}

/// Fetch HTML content from a URL using monolith with specified options
async fn fetch_html(client: &Client, url: &str) -> Result<String> {
    // Handle file:// URLs
    if url.starts_with("file://") {
        let path = url.strip_prefix("file://").unwrap_or(url);
        return match tokio::fs::read_to_string(path).await {
            Ok(content) => Ok(content),
            Err(e) => Err(anyhow::anyhow!("Failed to read local file {}: {}", path, e)),
        };
    }

    tracing::debug!("Sending HTTP request to: {}", url);

    // Wrap the request in a timeout to catch hanging connections
    let response = match tokio::time::timeout(Duration::from_secs(30), client.get(url).send()).await
    {
        Ok(Ok(resp)) => {
            tracing::debug!("Received HTTP response from: {}", url);
            resp
        }
        Ok(Err(e)) => {
            tracing::error!("HTTP request failed for {}: {}", url, e);
            return Err(anyhow::anyhow!("Failed to fetch URL {}: {}", url, e));
        }
        Err(_) => {
            tracing::error!("HTTP request timed out for {} after 30 seconds", url);
            return Err(anyhow::anyhow!("Request timed out for URL: {}", url));
        }
    };

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

    let (_, charset, _) = monolith::core::parse_content_type(content_type);

    tracing::debug!("Reading response body from: {}", url);
    let html_bytes = response
        .bytes()
        .await
        .with_context(|| format!("Failed to read response body from URL: {}", url))?;
    tracing::debug!(
        "Response body read successfully, size: {} bytes",
        html_bytes.len()
    );

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
    let html_bytes_arc = Arc::new(html_bytes.to_vec());

    // Re-enable Monolith with better timeout handling
    // Try to process with Monolith in a blocking task, fall back to simple HTML if it panics or times out.
    tracing::debug!("Starting Monolith processing for: {}", url);
    let monolith_future = tokio::task::spawn_blocking({
        let html_bytes_task = Arc::clone(&html_bytes_arc);
        let simple_html_task = simple_html.clone();
        // Move charset, document_url, options into the closure for spawn_blocking
        move || {
            // This inner closure is for catch_unwind
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // DOM creation can panic (e.g. charset not found by monolith)
                let dom = monolith::html::html_to_dom(&html_bytes_task, charset.clone());

                // Attempt to create a blocking client for asset embedding
                let client_result = reqwest::blocking::Client::builder()
                    .user_agent(crate::USER_AGENT_STRING)
                    .timeout(std::time::Duration::from_secs(5)) // Add timeout to monolith's client
                    .connect_timeout(std::time::Duration::from_secs(3))
                    .build();

                if let Ok(client) = client_result {
                    let cache_map: Cache = Cache::new(0, None); // Removed mut
                    let mut cache: Option<Cache> = Some(cache_map);
                    // walk_and_embed_assets can panic
                    monolith::html::walk_and_embed_assets(
                        &mut cache,
                        &client,
                        &document_url, // document_url was moved into spawn_blocking closure
                        &dom.document,
                        &options, // options was moved into spawn_blocking closure
                    );
                } else {
                    tracing::warn!(
                        "Monolith: Failed to create blocking client for asset embedding ({}). Skipping asset embedding for {}.",
                        client_result.err().map(|e| e.to_string()).unwrap_or_else(|| "unknown error".into()),
                        document_url
                    );
                }

                // serialize_document can panic
                monolith::html::serialize_document(dom, charset, &options)
            })) {
                Ok(processed_bytes) => {
                    // Monolith operations completed without panic
                    tracing::debug!("Monolith processing successful for {}", document_url);
                    processed_bytes
                }
                Err(panic_payload) => {
                    // Monolith panicked at some point during DOM, asset, or serialization
                    let panic_msg = if let Some(s) = panic_payload.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = panic_payload.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "Unknown panic".to_string()
                    };
                    tracing::warn!(
                        "Monolith panicked while processing {}: {}. Falling back to simple HTML.",
                        document_url,
                        panic_msg // Use moved document_url
                    );
                    simple_html_task.into_bytes()
                }
            }
        }
    });

    // Apply timeout to the monolith processing
    let processed_html_bytes = match tokio::time::timeout(Duration::from_secs(10), monolith_future)
        .await
    {
        Ok(Ok(bytes)) => bytes, // Success
        Ok(Err(e)) => {
            // spawn_blocking error
            tracing::error!("Task for monolith processing panicked or was cancelled for {}: {}. Falling back to simple HTML.", url, e);
            simple_html.into_bytes()
        }
        Err(_) => {
            // Timeout
            tracing::warn!("Monolith processing timed out after 10 seconds for {}. Falling back to simple HTML.", url);
            simple_html.into_bytes()
        }
    };

    String::from_utf8(processed_html_bytes).map_err(|e| {
        anyhow::anyhow!(
            "Failed to convert processed HTML (UTF-8) for {}: {}",
            url,
            e
        )
    })
}

// --- Functions moved from url.rs ---

// Ensure tokio::time::Duration is available if not already imported at the top
// use tokio::time::Duration; // Already imported via html.rs top-level imports if used by create_http_client etc.
// PathBuf is already used and Result from anyhow.

/// Processes a URL, retrying on failure. Writes to output_path or stdout.
pub(crate) async fn process_url_with_retry(
    url: &str,
    output_path: Option<PathBuf>,
    max_retries: u32,
) -> Result<(), (String, anyhow::Error)> {
    if url.starts_with("file://") {
        tracing::info!("Processing local file (no retry needed): {}", url);
        // Call to self::process_url_async (which is already in html.rs)
        match self::process_url_async(url, output_path).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                tracing::error!("Error processing local file {}: {}", url, e);
                return Err((url.to_string(), e));
            }
        }
    }

    let mut last_error = None;
    for attempt in 0..=max_retries {
        if attempt > 0 {
            tracing::info!(
                "Retrying {} (attempt {}/{})",
                url,
                attempt + 1,
                max_retries + 1
            );
        }
        // Call to self::process_url_async
        match self::process_url_async(url, output_path.clone()).await {
            Ok(_) => {
                if attempt > 0 {
                    tracing::info!("Successfully processed {} on attempt {}", url, attempt + 1);
                }
                return Ok(());
            }
            Err(e) => {
                tracing::debug!("Attempt {} failed for {}: {}", attempt + 1, url, e);
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                }
            }
        }
    }
    Err((url.to_string(), last_error.unwrap()))
}

/// Fetches and processes URL content, retrying on failure. Optionally writes to file and returns content.
pub(crate) async fn process_url_content_with_retry(
    // Renamed to avoid collision
    url: &str,
    output_path: Option<PathBuf>,
    max_retries: u32,
) -> Result<Option<String>, (String, anyhow::Error)> {
    let mut last_error = None;
    let mut content: Option<String> = None;

    for attempt in 0..=max_retries {
        if attempt > 0 {
            tracing::info!(
                "Retrying {} for content (attempt {}/{})",
                url,
                attempt + 1,
                max_retries + 1
            );
        }
        // Call to self::process_url_with_content (the one already in html.rs that returns String)
        match self::process_url_with_content(url, output_path.clone()).await {
            Ok(md_content) => {
                if !md_content.is_empty() {
                    if attempt > 0 {
                        tracing::info!(
                            "Successfully fetched non-empty content for {} on attempt {}",
                            url,
                            attempt + 1
                        );
                    }
                    content = Some(md_content);
                } else {
                    // md_content is empty. Check if it was a deliberate skip by get_markdown_for_url.
                    // get_markdown_for_url returns None for skips, and process_url_with_content translates that to String::new().
                    let was_skipped = self::get_markdown_for_url(url)
                        .await
                        .unwrap_or(None)
                        .is_none();
                    if was_skipped {
                        tracing::debug!("URL {} was skipped (e.g. non-HTML), retry logic will yield None for content.", url);
                        content = None; // Explicitly set to None for a skip.
                    } else {
                        tracing::info!(
                            "Successfully fetched empty content for {} on attempt {}",
                            url,
                            attempt + 1
                        );
                        content = Some(md_content); // Actual empty page
                    }
                }
                break; // Processing successful (or determined skip), exit retry loop.
            }
            Err(e) => {
                tracing::debug!(
                    "Attempt {} to fetch content failed for {}: {}",
                    attempt + 1,
                    url,
                    e
                );
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                }
            }
        }
    }

    // If content is Some, it means success or empty result from processing
    // If content is None, it means either all retries failed, or it was a non-HTML skip
    if content.is_some() {
        Ok(content) // This will be Some(String) or Some("")
    } else if self::get_markdown_for_url(url)
        .await
        .unwrap_or(None)
        .is_none()
        && last_error.is_none()
    {
        // Explicitly skipped by get_markdown_for_url (e.g. non-HTML) and no actual processing error occurred.
        Ok(None)
    } else {
        Err((
            url.to_string(),
            last_error
                .unwrap_or_else(|| anyhow::anyhow!("Unknown error after retries for {}", url)),
        ))
    }
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
        let cache_map: Cache = Cache::new(0, None);
        let mut cache: Option<Cache> = Some(cache_map);
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
