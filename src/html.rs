use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
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

/// Fetch HTML content from a URL
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

    let (_, _charset, _) = monolith::utils::parse_content_type(content_type);

    let html_bytes = response
        .bytes()
        .await
        .with_context(|| format!("Failed to read response body from URL: {}", url))?;
    let html_vec = html_bytes.to_vec();

    let processed_html = String::from_utf8(html_vec).context("Failed to convert HTML to UTF-8")?;

    Ok(processed_html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use html5ever::parse_document;
    use html5ever::serialize::{serialize as html_serialize, SerializeOpts};
    use html5ever::tendril::TendrilSink;
    use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

    fn remove_styles(node: &Handle) {
        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = node.data
        {
            // Remove style tags
            if name.local.as_ref() == "style" {
                node.children.borrow_mut().clear();
            }

            // Remove style attributes
            let mut attrs = attrs.borrow_mut();
            attrs.retain(|attr| attr.name.local.as_ref() != "style");
        }

        // Process children
        for child in node.children.borrow().iter() {
            remove_styles(child);
        }
    }

    #[test]
    fn test_html_processing() -> Result<()> {
        // Read test HTML file
        let html =
            fs::read_to_string("testdata/sample.html").context("Failed to read test HTML file")?;

        // Parse HTML
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap();

        // Remove styles
        remove_styles(&dom.document);

        // Serialize back to HTML
        let mut html_buf = Vec::new();
        html_serialize(
            &mut html_buf,
            &SerializableHandle::from(dom.document),
            SerializeOpts::default(),
        )?;
        let processed_html = String::from_utf8(html_buf)?;

        // Convert processed HTML to markdown
        let markdown = markdown::convert_html_to_markdown(&processed_html)?;

        // Verify content structure
        assert!(markdown.contains("# Main Heading"));
        assert!(markdown.contains("## Sub Heading"));
        assert!(markdown.contains("*   List item 1"));
        assert!(markdown.contains("*   List item 2"));
        assert!(markdown.contains("[A link](https://example.com)"));

        Ok(())
    }

    #[test]
    fn test_process_url_output() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let output_path = temp_dir.path().join("test.md");

        // Read test HTML and convert directly
        let html = fs::read_to_string("testdata/sample.html")?;
        let markdown = markdown::convert_html_to_markdown(&html)?;

        // Write to output file
        fs::write(&output_path, &markdown)?;

        // Verify file exists and contains expected content
        assert!(output_path.exists());
        let output_content = fs::read_to_string(&output_path)?;
        assert!(output_content.contains("# Main Heading"));

        Ok(())
    }
}
