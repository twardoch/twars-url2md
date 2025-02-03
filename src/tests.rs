#![cfg(test)]

use super::*;
use std::io::Write;
use tempfile::NamedTempFile;

// Helper function to create a temporary file with content
fn create_temp_file(content: &str) -> Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(file)
}

#[test]
fn test_extract_urls_from_plain_text() {
    let text = r#"
        Here are some URLs:
        https://example.com
        http://test.org/path
        Not a URL: example.com
        https://sub.domain.com/path/to/page?param=value#fragment
        Mixed content: Visit https://rust-lang.org for more info!
    "#;

    let urls = extract_urls_from_text(text);
    assert_eq!(urls.len(), 4);
    assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
    assert!(urls.iter().any(|u| u.starts_with("http://test.org/path")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://sub.domain.com/path/to/page?param=value")));
    assert!(urls.iter().any(|u| u.starts_with("https://rust-lang.org")));
}

#[test]
fn test_extract_urls_from_html() {
    let html = r#"
        <html>
            <body>
                <a href="https://example.com">Link</a>
                <p>Visit <a href="http://test.org/path">this site</a></p>
                <img src="https://images.example.com/pic.jpg">
                <script>
                    var url = 'https://script.example.com';
                </script>
                <!-- https://comment.example.com -->
            </body>
        </html>
    "#;

    let urls = extract_urls_from_text(html);
    assert_eq!(urls.len(), 5);
    assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
    assert!(urls.iter().any(|u| u.starts_with("http://test.org/path")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://images.example.com/pic.jpg")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://script.example.com")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://comment.example.com")));
}

#[test]
fn test_extract_urls_from_markdown() {
    let markdown = r#"
        # Test Document

        [Link 1](https://example.com)
        ![Image](https://images.example.com/pic.jpg)

        Regular URL: https://plain.example.com

        Reference style:
        [ref link][1]

        [1]: https://reference.example.com
    "#;

    let urls = extract_urls_from_text(markdown);
    assert_eq!(urls.len(), 4);
    assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://images.example.com/pic.jpg")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://plain.example.com")));
    assert!(urls
        .iter()
        .any(|u| u.starts_with("https://reference.example.com")));
}

#[test]
fn test_extract_urls_handles_invalid_urls() {
    let text = r#"
        Valid: https://example.com
        Invalid: https://
        Valid: http://test.org
        Invalid: http://.
        Invalid: https://invalid.\
        Valid: https://sub.domain.com/path
        Invalid: ftp://example.com
        Invalid: not-a-url
        Invalid: file:///path/to/file
    "#;

    let urls = extract_urls_from_text(text);
    let urls_str: Vec<_> = urls.iter().map(|u| u.as_str()).collect();
    assert_eq!(
        urls.len(),
        3,
        "Expected exactly 3 valid URLs, found: {:?}",
        urls_str
    );
    assert!(urls_str.contains(&"https://example.com"));
    assert!(urls_str.contains(&"http://test.org"));
    assert!(urls_str.contains(&"https://sub.domain.com/path"));
}

#[test]
fn test_extract_urls_handles_duplicates() {
    let text = r#"
        https://example.com
        http://test.org
        https://example.com
        http://test.org
        https://example.com
    "#;

    let urls = extract_urls_from_text(text);
    assert_eq!(urls.len(), 2);
    assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
    assert!(urls.iter().any(|u| u.starts_with("http://test.org")));
}

#[test]
fn test_extract_urls_with_special_characters() {
    let text = r#"
        https://example.com/path with spaces
        https://example.com/path%20with%20encoding
        https://example.com/path(with)parentheses
        https://example.com/path?param=value&other=123
        https://example.com/path#fragment
        https://user:pass@example.com/path
        https://example.com/path/with/unicode/⭐/✨
    "#;

    let urls = extract_urls_from_text(text);
    assert_eq!(urls.len(), 7);
}

#[tokio::test]
async fn test_create_output_path() -> Result<()> {
    let base_dir = tempfile::tempdir()?;

    // Test simple URL
    let url = Url::parse("https://example.com/page")?;
    let path = create_output_path(&url, base_dir.path())?;
    assert_eq!(
        path.strip_prefix(base_dir.path())?.to_str().unwrap(),
        "example.com/page.md"
    );

    // Test URL with trailing slash
    let url = Url::parse("https://example.com/path/")?;
    let path = create_output_path(&url, base_dir.path())?;
    assert_eq!(
        path.strip_prefix(base_dir.path())?.to_str().unwrap(),
        "example.com/path/index.md"
    );

    // Test URL with no path
    let url = Url::parse("https://example.com")?;
    let path = create_output_path(&url, base_dir.path())?;
    assert_eq!(
        path.strip_prefix(base_dir.path())?.to_str().unwrap(),
        "example.com/index.md"
    );

    Ok(())
}

// Integration test for CLI functionality
#[tokio::test]
async fn test_cli_text_input() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let input_content = r#"
        Check out these sites:
        https://example.com
        http://test.org
        Visit https://rust-lang.org for more!
    "#;

    let input_file = create_temp_file(input_content)?;

    let cli = Cli {
        urls: vec![],
        input_file: None,
        stdin: false,
        text_input: Some(input_file.path().to_path_buf()),
        text_stdin: false,
        output: Some(temp_dir.path().to_path_buf()),
        verbose: false,
    };

    // Run the equivalent of main() logic with our CLI args
    let mut urls = Vec::new();

    if let Some(text_input) = &cli.text_input {
        let content = fs::read_to_string(text_input)?;
        urls.extend(extract_urls_from_text(&content));
    }

    urls.sort_unstable();
    urls.dedup();

    let urls_str: Vec<_> = urls.iter().map(|u| u.as_str()).collect();
    assert_eq!(
        urls.len(),
        3,
        "Expected exactly 3 valid URLs, found: {:?}",
        urls_str
    );
    assert!(
        urls_str.contains(&"https://example.com"),
        "Missing https://example.com in {:?}",
        urls_str
    );
    assert!(
        urls_str.contains(&"http://test.org"),
        "Missing http://test.org in {:?}",
        urls_str
    );
    assert!(
        urls_str.contains(&"https://rust-lang.org"),
        "Missing https://rust-lang.org in {:?}",
        urls_str
    );

    Ok(())
}

// Add new test for URL processing with retries
#[tokio::test]
async fn test_process_url_with_retry() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let url = "https://example.com".to_string();
    let output_path = Some(temp_dir.path().join("output.md"));

    // Test successful processing
    let result = process_url_with_retry(url.clone(), output_path.clone(), true).await;
    assert!(result.is_err(), "Expected error for non-existent URL");

    Ok(())
}
