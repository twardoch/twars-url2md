use anyhow::{Context, Result};
use monolith::opts::Options;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::markdown;

/// Process a URL by downloading its content and converting to Markdown
pub async fn process_url_async(url: &str, output_path: Option<PathBuf>) -> Result<()> {
    let client = create_http_client()?;
    let html = fetch_html(&client, url).await?;
    let markdown = markdown::convert_html_to_markdown(&html)?;

    match output_path {
        Some(path) => {
            fs::write(&path, markdown)
                .with_context(|| format!("Failed to write to file: {}", path.display()))?;
            eprintln!("Created: {}", path.display());
        }
        None => println!("{}", markdown),
    }

    Ok(())
}

/// Create an HTTP client with appropriate headers
fn create_http_client() -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(crate::USER_AGENT_STRING),
    );

    Client::builder()
        .default_headers(headers)
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

    // Create monolith options with specified flags
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
        ..Default::default()
    };

    // Convert URL to reqwest::Url
    let document_url =
        reqwest::Url::parse(url).with_context(|| format!("Failed to parse URL: {}", url))?;

    // Create DOM from HTML
    let dom = monolith::html::html_to_dom(&html_bytes.to_vec(), charset.clone());

    // Process assets and embed them
    let mut cache = HashMap::new();
    let blocking_client = reqwest::blocking::Client::new();
    monolith::html::walk_and_embed_assets(
        &mut cache,
        &blocking_client,
        &document_url,
        &dom.document,
        &options,
    );

    // Serialize back to HTML
    let processed_html = monolith::html::serialize_document(dom, charset, &options);
    String::from_utf8(processed_html)
        .with_context(|| "Failed to convert processed HTML to UTF-8".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
