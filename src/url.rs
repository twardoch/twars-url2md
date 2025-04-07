use anyhow::{Context, Result};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use linkify::{LinkFinder, LinkKind};
use markup5ever_rcdom as rcdom;
use rayon::prelude::*;
use regex;
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
    // Pre-allocate with a reasonable capacity based on text length
    let estimated_capacity = text.len() / 100; // More conservative estimate
    let mut urls = Vec::with_capacity(estimated_capacity.min(1000));

    // Add logic to identify local file paths
    let file_regex = regex::Regex::new(r"^(file://)?(/[^/\s]+(?:/[^/\s]+)*\.html?)$").unwrap();

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
    if text.trim().starts_with('<') {
        extract_urls_from_html_efficient(text, base_url, urls);
    } else {
        let finder = LinkFinder::new();
        urls.extend(finder.links(text).filter_map(|link| {
            if link.kind() == &LinkKind::Url {
                try_parse_url(link.as_str(), base_url)
            } else {
                None
            }
        }));
    }
}

/// More efficient HTML URL extraction
fn extract_urls_from_html_efficient(html: &str, base_url: Option<&str>, urls: &mut Vec<String>) {
    // Use a pre-configured link finder for better performance
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);

    // Process in parallel if HTML is large enough
    if html.len() > 50_000 {
        // Split HTML into chunks at word boundaries
        let chunks: Vec<&str> = html.split_whitespace().collect();
        let processed_urls: Vec<String> = chunks
            .par_iter()
            .flat_map(|&chunk| {
                finder
                    .links(chunk)
                    .filter_map(|link| try_parse_url(link.as_str(), base_url))
                    .collect::<Vec<_>>()
            })
            .collect();
        urls.extend(processed_urls);
    } else {
        urls.extend(
            finder
                .links(html)
                .filter_map(|link| try_parse_url(link.as_str(), base_url)),
        );
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
    urls
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
    // Special handling for file:// URLs - no retry needed
    if url.starts_with("file://") {
        if verbose {
            eprintln!("Processing local file: {}", url);
        }
        match crate::html::process_url_async(url, output_path, verbose).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err((url.to_string(), e)),
        }
    }

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

/// Process a URL and return the Markdown content
pub async fn process_url_with_content(
    url: &str,
    output_path: Option<PathBuf>,
    verbose: bool,
    max_retries: u32,
) -> Result<Option<String>, (String, anyhow::Error)> {
    let mut last_error = None;
    let mut content = None;

    for attempt in 0..=max_retries {
        if attempt > 0 && verbose {
            eprintln!(
                "Retrying {} (attempt {}/{})",
                url,
                attempt + 1,
                max_retries + 1
            );
        }

        match crate::html::process_url_with_content(url, output_path.clone(), verbose).await {
            Ok(md_content) => {
                content = Some(md_content);
                break;
            }
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                }
            }
        }
    }

    if let Some(content) = content {
        Ok(Some(content))
    } else {
        Err((url.to_string(), last_error.unwrap()))
    }
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
