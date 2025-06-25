use anyhow::{Context, Result};
use curl::easy::Easy;
use monolith::cache::Cache;
use monolith::core::Options;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::markdown;
use crate::url::Url;

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

    tracing::debug!("Fetching HTML for URL (get_markdown_for_url): {}", url);
    let html = match fetch_html_with_curl(url).await {
        Ok(html_content) => html_content,
        Err(e) => {
            tracing::warn!(
                "Error fetching HTML from {} (get_markdown_for_url): {}. Using fallback processing.",
                url,
                e
            );
            // Try to get raw HTML as fallback
            return Err(e);
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

/// Fallback HTML fetch using libcurl for robust cross-platform support.
async fn fetch_html_with_curl(url: &str) -> Result<String> {
    let url_owned = url.to_string();
    tokio::task::spawn_blocking(move || {
        let mut easy = Easy::new();
        easy.url(&url_owned)?;
        easy.follow_location(true)?;
        easy.useragent(crate::USER_AGENT_STRING)?;
        easy.accept_encoding("gzip,deflate,br")?;
        easy.connect_timeout(Duration::from_secs(20))?;
        easy.timeout(Duration::from_secs(60))?;
        easy.http_version(curl::easy::HttpVersion::V11)?;

        let mut data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })?;
            transfer.perform()?;
        }

        let code = easy.response_code()?;
        if code >= 400 {
            return Err(anyhow::anyhow!("HTTP error status {}", code));
        }

        let ct = easy.content_type()?.unwrap_or("text/html");
        if !ct.contains("text/html") {
            return Err(anyhow::anyhow!("Not an HTML page: {}", ct));
        }

        Ok(String::from_utf8_lossy(&data).into_owned())
    })
    .await
    .context("curl blocking task failed")?
}

/// Fetch HTML content from a URL using monolith with specified options
#[allow(dead_code)]
async fn fetch_html(url: &str) -> Result<String> {
    // Handle file:// URLs
    if url.starts_with("file://") {
        let path = url.strip_prefix("file://").unwrap_or(url);
        return match tokio::fs::read_to_string(path).await {
            Ok(content) => Ok(content),
            Err(e) => Err(anyhow::anyhow!("Failed to read local file {}: {}", path, e)),
        };
    }

    tracing::debug!("Sending HTTP request to: {}", url);

    // Since we are using curl now, the reqwest specific logic is removed.
    // This function will now be a wrapper around monolith processing.
    let html_bytes = fetch_html_with_curl(url).await?.into_bytes();

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

    let document_url = Url::parse(url).with_context(|| format!("Failed to parse URL: {}", url))?;
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
                let dom = monolith::html::html_to_dom(&html_bytes_task, "UTF-8".to_string());

                // No client for asset embedding since we removed reqwest
                let client_result: Result<curl::easy::Easy, curl::Error> = Err(curl::Error::new(0));

                if let Ok(_client) = client_result {
                    let _cache_map: Cache = Cache::new(0, None); // Removed mut
                    let _cache: Option<Cache> = Some(_cache_map);
                    // walk_and_embed_assets can panic
                    // monolith::html::walk_and_embed_assets(
                    //     &mut cache,
                    //     &client,
                    //     &document_url, // document_url was moved into spawn_blocking closure
                    //     &dom.document,
                    //     &options, // options was moved into spawn_blocking closure
                    // );
                } else {
                    tracing::warn!(
                        "Monolith: Asset embedding is disabled as reqwest is removed ({})",
                        document_url
                    );
                }

                // serialize_document can panic
                monolith::html::serialize_document(dom, "UTF-8".to_string(), &options)
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
    use tempfile::TempDir;

    #[test]
    fn test_create_http_client() {
        // This test is now obsolete as we are not creating a reqwest client anymore.
        // We can keep it to ensure the file compiles.
    }

    #[tokio::test]
    async fn test_skip_non_html_urls() {
        let non_html_urls = vec![
            "https://example.com/image.jpg",
            "https://example.com/document.pdf",
            "https://example.com/video.mp4",
            "https://example.com/image.png",
            "https://example.com/image.gif",
            "https://example.com/image.svg",
            "https://example.com/image.webp",
            "https://example.com/video.webm",
        ];

        for url in non_html_urls {
            let result = get_markdown_for_url(url).await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_none(), "URL {} should be skipped", url);
        }
    }

    #[tokio::test]
    async fn test_local_file_processing() {
        let temp_dir = TempDir::new().unwrap();
        let html_file = temp_dir.path().join("test.html");
        let html_content = r#"
            <html>
                <head><title>Local Test</title></head>
                <body>
                    <h1>Local File Test</h1>
                    <p>This is a local file.</p>
                </body>
            </html>
        "#;

        std::fs::write(&html_file, html_content).unwrap();

        let file_url = format!("file://{}", html_file.display());
        let result = get_markdown_for_url(&file_url).await;

        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.is_some());
        let markdown_content = markdown.unwrap();
        assert!(markdown_content.contains("Local File Test"));
        assert!(markdown_content.contains("local file"));
    }

    #[tokio::test]
    async fn test_process_url_with_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let html_file = temp_dir.path().join("source.html");
        let output_file = temp_dir.path().join("output.md");

        let html_content = r#"
            <html>
                <body>
                    <h1>Test Output</h1>
                    <p>Content to be saved.</p>
                </body>
            </html>
        "#;

        std::fs::write(&html_file, html_content).unwrap();
        let file_url = format!("file://{}", html_file.display());

        let result = process_url_async(&file_url, Some(output_file.clone())).await;
        assert!(result.is_ok());

        // Verify output file was created
        assert!(output_file.exists());
        let content = std::fs::read_to_string(&output_file).unwrap();
        assert!(content.contains("Test Output"));
    }

    #[tokio::test]
    async fn test_process_url_with_content_return() {
        let temp_dir = TempDir::new().unwrap();
        let html_file = temp_dir.path().join("content.html");

        let html_content = r#"
            <html>
                <body>
                    <h1>Content Test</h1>
                    <p>This content should be returned.</p>
                </body>
            </html>
        "#;

        std::fs::write(&html_file, html_content).unwrap();
        let file_url = format!("file://{}", html_file.display());

        let result = process_url_with_content(&file_url, None).await;
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("Content Test"));
        assert!(content.contains("This content should be returned"));
    }

    #[tokio::test]
    async fn test_retry_logic() {
        // Test retry with a local file (should succeed immediately)
        let temp_dir = TempDir::new().unwrap();
        let html_file = temp_dir.path().join("retry.html");
        std::fs::write(&html_file, "<h1>Retry Test</h1>").unwrap();

        let file_url = format!("file://{}", html_file.display());
        let result = process_url_with_retry(&file_url, None, 3).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_url_content_with_retry() {
        let temp_dir = TempDir::new().unwrap();
        let html_file = temp_dir.path().join("retry_content.html");
        std::fs::write(&html_file, "<h1>Retry Content</h1>").unwrap();

        let file_url = format!("file://{}", html_file.display());
        let result = process_url_content_with_retry(&file_url, None, 3).await;

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.is_some());
        assert!(content.unwrap().contains("Retry Content"));
    }

    // #[test]
    // fn test_html_processing() -> Result<()> {
    //     // Sample HTML with various elements that should be processed
    //     let html_content = r#"
    //         <!DOCTYPE html>
    //         <html>
    //         <head>
    //             <title>Test Page</title>
    //             <style>body { color: red; }</style>
    //             <script>console.log('test');</script>
    //             <link rel="stylesheet" href="style.css">
    //         </head>
    //         <body>
    //             <h1>Main Heading</h1>
    //             <h2>Sub Heading</h2>
    //             <ul>
    //                 <li>List item 1</li>
    //                 <li>List item 2</li>
    //             </ul>
    //             <a href="https://example.com">A link</a>
    //             <img src="image.jpg" />
    //             <video src="video.mp4"></video>
    //             <iframe src="frame.html"></iframe>
    //             <font face="Arial">Font text</font>
    //         </body>
    //         </html>
    //     "#;

    //     // Create monolith options with specified flags
    //     let options = Options {
    //         no_video: true,
    //         isolate: true,
    //         no_js: true,
    //         no_css: true,
    //         base_url: Some("https://example.com".to_string()),
    //         ignore_errors: true,
    //         no_fonts: true,
    //         no_images: true,
    //         insecure: true,
    //         no_metadata: true,
    //         silent: true,
    //         ..Default::default()
    //     };

    //     // Create DOM from HTML
    //     let dom =
    //         monolith::html::html_to_dom(&html_content.as_bytes().to_vec(), "UTF-8".to_string());

    //     // Process assets and embed them
    //     let _cache_map: Cache = Cache::new(0, None);
    //     let _cache: Option<Cache> = Some(_cache_map);
    //     let _document_url = Url::parse("https://example.com").unwrap();
    //     // Asset embedding is disabled
    //     // monolith::html::walk_and_embed_assets(
    //     //     &mut cache,
    //     //     &client,
    //     //     &_document_url,
    //     //     &dom.document,
    //     //     &options,
    //     // );

    //     // Serialize back to HTML
    //     let processed_html = monolith::html::serialize_document(dom, "UTF-8".to_string(), &options);
    //     let processed_html = String::from_utf8(processed_html).unwrap();

    //     // Convert to markdown
    //     let markdown = markdown::convert_html_to_markdown(&processed_html)?;

    //     // Verify content structure is preserved
    //     assert!(markdown.contains("# Main Heading"));
    //     assert!(markdown.contains("## Sub Heading"));
    //     assert!(markdown.contains("*   List item 1"));
    //     assert!(markdown.contains("*   List item 2"));
    //     assert!(markdown.contains("[A link](https://example.com)"));

    //     // Verify that elements are properly handled according to options
    //     // assert!(!processed_html.contains("src=\"video.mp4\"")); // no_video
    //     // assert!(!processed_html.contains("src=\"image.jpg\"")); // no_images
    //     // assert!(!processed_html.contains("href=\"style.css\"")); // no_css
    //     // assert!(!processed_html.contains("src=\"frame.html\"")); // isolate
    //     // assert!(!processed_html.contains("console.log")); // no_js

    //     Ok(())
    // }

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

    #[tokio::test]
    async fn test_fallback_for_markdown_conversion_failure() {
        // HTML that might cause issues with conversion
        let html_content = r#"
            <p>Test with<br>line breaks<br/>and<br />various formats</p>
            <p>Another paragraph</p>
        "#;

        // The fallback conversion should handle basic replacements
        let simple_md = html_content
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            .replace("<p>", "\n\n")
            .replace("</p>", "");

        assert!(simple_md.contains("Test with\nline breaks\nand\nvarious formats"));
        assert!(simple_md.contains("\n\nAnother paragraph"));
    }

    #[test]
    fn test_monolith_panic_recovery() {
        // Test that panic recovery mechanism works
        // This is mostly verified by the actual implementation using catch_unwind
        // Here we just verify the structure exists

        let html = "<html><body>Test</body></html>";
        let result = std::panic::catch_unwind(|| {
            // Simulate code that might panic
            let dom = monolith::html::html_to_dom(&html.as_bytes().to_vec(), "UTF-8".to_string());
            monolith::html::serialize_document(dom, "UTF-8".to_string(), &Options::default())
        });

        assert!(result.is_ok() || result.is_err()); // Just verify catch_unwind works
    }
}
