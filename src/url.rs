use anyhow::{Context, Result};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use linkify::{LinkFinder, LinkKind};
use markup5ever_rcdom as rcdom;
use std::path::{Path, PathBuf};
use tokio::time::Duration;
pub use url::Url;

/// Create an output path for a URL based on its structure
pub fn create_output_path(url: &Url, base_dir: &Path) -> Result<PathBuf> {
    let host = url.host_str().unwrap_or("unknown");

    let path_segments: Vec<_> = url.path().split('/').filter(|s| !s.is_empty()).collect();

    let dir_path = base_dir.join(host);
    std::fs::create_dir_all(&dir_path)
        .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;

    let mut full_path = dir_path;
    if !path_segments.is_empty() {
        for segment in &path_segments[..path_segments.len() - 1] {
            full_path = full_path.join(segment);
            std::fs::create_dir_all(&full_path)?;
        }
    }

    let filename = if url.path().ends_with('/') || path_segments.is_empty() {
        "index.md".to_string()
    } else {
        let last_segment = path_segments.last().unwrap();
        if let Some(stem) = Path::new(last_segment).file_stem() {
            format!("{}.md", stem.to_string_lossy())
        } else {
            format!("{}.md", last_segment)
        }
    };

    Ok(full_path.join(filename))
}

/// Extract URLs from any text input
pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String> {
    if text.trim().starts_with('<') {
        extract_urls_from_html(text, base_url)
    } else {
        let finder = LinkFinder::new();
        let mut urls: Vec<String> = finder
            .links(text)
            .filter_map(|link| {
                if link.kind() == &LinkKind::Url {
                    try_parse_url(link.as_str(), base_url)
                } else {
                    None
                }
            })
            .collect();
        urls.sort();
        urls.dedup();
        urls
    }
}

/// Extract URLs from HTML content, including attributes and text content
pub fn extract_urls_from_html(html: &str, base_url: Option<&str>) -> Vec<String> {
    let mut urls = Vec::new();

    // Parse HTML document
    let dom = parse_document(rcdom::RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    // Extract URLs from HTML structure
    extract_urls_from_node(&dom.document, base_url, &mut urls);

    // Use LinkFinder as a fallback to catch any remaining URLs in text content
    let finder = LinkFinder::new();
    for link in finder.links(html) {
        if link.kind() == &LinkKind::Url {
            if let Some(url) = try_parse_url(link.as_str(), base_url) {
                urls.push(url);
            }
        }
    }

    // Deduplicate and sort URLs
    urls.sort();
    urls.dedup();
    urls
}

fn extract_urls_from_node(node: &rcdom::Handle, base_url: Option<&str>, urls: &mut Vec<String>) {
    // Only process element nodes
    if let rcdom::NodeData::Element { ref attrs, .. } = node.data {
        let attrs = attrs.borrow();

        // Define URL-containing attributes to check
        let url_attrs = ["href", "src", "data-src", "data-href", "data-url"];

        for attr in attrs.iter() {
            let attr_name = attr.name.local.to_string();
            let attr_value = attr.value.to_string();

            if url_attrs.contains(&attr_name.as_str()) {
                if let Some(url) = try_parse_url(&attr_value, base_url) {
                    urls.push(url);
                }
            } else if attr_name == "srcset" {
                // Handle srcset attribute which may contain multiple URLs
                for src in attr_value.split(',') {
                    let src = src.split_whitespace().next().unwrap_or("");
                    if let Some(url) = try_parse_url(src, base_url) {
                        urls.push(url);
                    }
                }
            }
        }
    }

    // Recursively process child nodes
    for child in node.children.borrow().iter() {
        extract_urls_from_node(child, base_url, urls);
    }
}

fn try_parse_url(url_str: &str, base_url: Option<&str>) -> Option<String> {
    // Skip obvious non-URLs
    if url_str.trim().is_empty()
        || url_str.starts_with("data:")
        || url_str.starts_with("javascript:")
        || url_str.starts_with('#')
        || url_str.contains('{')
        || url_str.contains('}')
        || url_str.contains('(')
        || url_str.contains(')')
        || url_str.contains('[')
        || url_str.contains(']')
        || url_str.contains('<')
        || url_str.contains('>')
        || url_str.contains('"')
        || url_str.contains('\'')
        || url_str.contains('`')
        || url_str.contains('\n')
        || url_str.contains('\r')
        || url_str.contains('\t')
        || url_str.contains(' ')
    {
        return None;
    }

    // Try parsing as absolute URL first
    if let Ok(url) = Url::parse(url_str) {
        if url.scheme() == "http" || url.scheme() == "https" {
            return Some(url.to_string());
        }
    }

    // If we have a base URL and the input looks like a relative URL, try joining
    if let Some(base) = base_url {
        if let Ok(base_url) = Url::parse(base) {
            if let Ok(url) = base_url.join(url_str) {
                if url.scheme() == "http" || url.scheme() == "https" {
                    return Some(url.to_string());
                }
            }
        }
    }

    None
}

/// Process a single URL with retries
pub async fn process_url_with_retry(
    url: &str,
    output_path: Option<PathBuf>,
    verbose: bool,
    max_retries: u32,
) -> Result<(), (String, anyhow::Error)> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        if attempt > 0 && verbose {
            eprintln!(
                "Retrying {} (attempt {}/{})",
                url,
                attempt + 1,
                max_retries + 1
            );
        }

        match crate::html::process_url_async(url, output_path.clone(), verbose).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                }
            }
        }
    }

    Err((url.to_string(), last_error.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_extract_urls_from_text() {
        let text = r#"
            https://example.com
            http://test.org
            Invalid: ftp://example.com
            https://example.com/path?query=value#fragment
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 3);
        assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
        assert!(urls.iter().any(|u| u.starts_with("http://test.org")));
    }

    #[tokio::test]
    async fn test_create_output_path() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let url = Url::parse("https://example.com/path/page")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.starts_with(temp_dir.path()));
        assert!(path.to_string_lossy().contains("example.com"));
        assert!(path.to_string_lossy().ends_with(".md"));

        Ok(())
    }
}
