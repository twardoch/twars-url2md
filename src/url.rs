use anyhow::{Context, Result};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use linkify::{LinkFinder, LinkKind};
use markup5ever_rcdom as rcdom;
// anyhow::Context is already imported via `use anyhow::{Context, Result};`
// use rayon::prelude::*; // No longer used after removing extract_urls_from_html_efficient
use regex;
use std::path::{Path, PathBuf};
pub use url::Url;

/// Create an output path for a URL based on its structure
pub fn create_output_path(url: &Url, base_dir: &Path) -> Result<PathBuf> {
    let host = url.host_str().unwrap_or("unknown");

    let path_segments: Vec<_> = url.path().split('/').filter(|s| !s.is_empty()).collect();

    // Build the directory path, including trailing-segment directories if the URL ends with '/'
    let mut dir_path_full = base_dir.join(host);

    // Decide which segments belong to directories vs. filename
    let segments_for_dirs: &[&str] = if path_segments.is_empty() {
        &[]
    } else if url.path().ends_with('/') {
        // All segments are directories when URL ends with '/'
        &path_segments[..]
    } else {
        // All except the last segment represent directories
        &path_segments[..path_segments.len() - 1]
    };

    for segment in segments_for_dirs {
        dir_path_full = dir_path_full.join(segment);
    }

    // Ensure directories exist on disk
    std::fs::create_dir_all(&dir_path_full).with_context(|| {
        format!("Failed to create directory: {}", dir_path_full.display())
    })?;

    // Determine filename
    let filename = if url.path().ends_with('/') || path_segments.is_empty() {
        "index.md".to_string()
    } else {
        let last_segment = path_segments.last().unwrap();
        Path::new(last_segment)
            .file_stem()
            .map(|s| format!("{}.md", s.to_string_lossy()))
            .unwrap_or_else(|| format!("{}.md", last_segment))
    };

    Ok(dir_path_full.join(filename))
}

/// Extract URLs from any text input
pub fn extract_urls_from_text(text: &str, base_url: Option<&str>) -> Vec<String> {
    // Pre-allocate with a reasonable capacity based on text length
    let estimated_capacity = text.len() / 100; // More conservative estimate
    let mut urls = Vec::with_capacity(estimated_capacity.min(1000));

    // Add logic to identify local file paths
    // This regex is static and assumed to be valid. Panicking here is acceptable if it's malformed.
    let file_regex = regex::Regex::new(r"^(file://)?(/[^/\s]+(?:/[^/\s]+)*\.html?)$")
        .expect("Invalid static regex for file paths");

    // Process text lines to extract URLs and local file paths
    for line in text.lines() {
        let line = line.trim();

        // Check if line is a local file path
        if file_regex.is_match(line) {
            // Convert to file:// URL format if not already
            let file_url = if line.starts_with("file://") {
                line.to_string()
            } else {
                format!("file://{}", line)
            };
            urls.push(file_url);
        } else if !line.is_empty() {
            // Process as regular URL
            process_text_chunk(line, base_url, &mut urls);
        }
    }

    // Use unstable sort for better performance since order doesn't matter for deduplication
    urls.sort_unstable();
    urls.dedup();
    urls
}

/// Process a chunk of text to extract URLs
fn process_text_chunk(text: &str, base_url: Option<&str>, urls: &mut Vec<String>) {
    let trimmed_text = text.trim();
    if trimmed_text.starts_with('<') {
        // Attempt to parse as HTML if it looks like an HTML tag/document fragment
        match extract_urls_from_html(trimmed_text, base_url) {
            Ok(extracted) => {
                urls.extend(extracted);
            }
            Err(e) => {
                // Log the error and fall back to simple LinkFinder for this chunk
                tracing::debug!(
                    "Failed to parse text chunk as HTML ({}): '{}...'. Falling back to LinkFinder.",
                    e,
                    trimmed_text.chars().take(50).collect::<String>()
                );
                let finder = LinkFinder::new();
                urls.extend(finder.links(trimmed_text).filter_map(|link| {
                    if link.kind() == &LinkKind::Url {
                        try_parse_url(link.as_str(), base_url)
                    } else {
                        None
                    }
                }));
            }
        }
    } else {
        // Standard LinkFinder for non-HTML-like text
        let finder = LinkFinder::new();
        urls.extend(finder.links(trimmed_text).filter_map(|link| {
            if link.kind() == &LinkKind::Url {
                try_parse_url(link.as_str(), base_url)
            } else {
                None
            }
        }));
    }
}

// Removed extract_urls_from_html_efficient.
// Its logic will be integrated into process_text_chunk's fallback,
// and extract_urls_from_html is the primary method for HTML content.

/// Extract URLs from HTML content, including attributes and text content
pub fn extract_urls_from_html(html: &str, base_url: Option<&str>) -> Result<Vec<String>> {
    let mut urls = Vec::new();

    // Parse HTML document
    let dom = parse_document(rcdom::RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to parse HTML for URL extraction: {}", e))?;

    // Extract URLs from HTML structure using iterative approach
    let mut stack = vec![dom.document.clone()];
    while let Some(node) = stack.pop() {
        // Process element nodes
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

        // Add child nodes to stack
        for child in node.children.borrow().iter() {
            stack.push(child.clone());
        }
    }

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
    Ok(urls)
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

    // Handle file:// URLs
    if url_str.starts_with("file://") {
        return Some(url_str.to_string());
    }

    // Check if it could be a local file path
    if url_str.starts_with('/') && Path::new(url_str).exists() {
        return Some(format!("file://{}", url_str));
    }

    // Try parsing as absolute URL first
    if let Ok(url) = Url::parse(url_str) {
        if url.scheme() == "http" || url.scheme() == "https" {
            // Normalize: drop trailing slash for bare domain URLs ("https://example.com/")
            let mut normalized = url.to_string();
            if url.path() == "/" && url.query().is_none() && url.fragment().is_none() {
                normalized.pop(); // remove the trailing '/'
            }
            return Some(normalized);
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

// process_url_with_retry and process_url_with_content (with retry logic)
// have been moved to src/html.rs and made pub(crate) there.

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
