// this_file: tests/unit/url_tests.rs

use anyhow::Result;
use tempfile::TempDir;
use url::Url;

#[cfg(test)]
mod url_extraction_tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use twars_url2md::url::extract_urls_from_text;

    #[test]
    fn test_extract_urls_from_plain_text() {
        let text = r#"
            Check out https://example.com for more info.
            Also visit http://test.org/page
            And https://another.site/path/to/resource?param=value#section
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 3);
        // LinkFinder may add trailing slashes
        assert!(urls.iter().any(|u| u.starts_with("https://example.com")));
        assert!(urls.iter().any(|u| u.starts_with("http://test.org/page")));
        assert!(urls
            .iter()
            .any(|u| u.starts_with("https://another.site/path/to/resource")));
    }

    #[test]
    fn test_extract_urls_from_markdown() {
        let text = r#"
            [Example Link](https://example.com)
            [Another Link](http://test.org "Title")
            ![Image](https://images.com/pic.jpg)
            <https://raw-url.com>
        "#;

        let urls = extract_urls_from_text(text, None);
        assert!(!urls.is_empty());
        assert!(urls.iter().any(|u| u.contains("example.com")));
        assert!(urls.iter().any(|u| u.contains("test.org")));
    }

    #[test]
    fn test_extract_urls_with_base_url() {
        let text = r#"
            /relative/path
            ../another/path
            https://absolute.com/path
        "#;

        let urls = extract_urls_from_text(text, Some("https://base.com"));
        assert!(urls.iter().any(|u| u == "https://absolute.com/path"));
        // Note: LinkFinder doesn't recognize bare paths without base URL context
    }

    #[test]
    fn test_extract_urls_deduplication() {
        let text = r#"
            https://example.com
            https://example.com
            https://example.com/path
            https://example.com
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"https://example.com/path".to_string()));
    }

    #[test]
    fn test_extract_local_file_paths() {
        let text = r#"
            /path/to/file.html
            file:///absolute/path/file.html
            /another/document.htm
        "#;

        let urls = extract_urls_from_text(text, None);
        assert!(urls.iter().any(|u| u.starts_with("file://")));
    }

    #[test]
    fn test_invalid_urls_ignored() {
        let text = r#"
            ftp://not-supported.com
            javascript:alert('test')
            data:text/plain,hello
            mailto:test@example.com
            #anchor-only
            https://valid.com
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 1);
        assert!(urls.contains(&"https://valid.com".to_string()));
    }

    #[test]
    fn test_urls_with_special_characters() {
        let text = r#"
            https://example.com/path?q=hello+world&lang=en
            https://test.org/resource#section-2.1
            https://api.site/v1/users/123/profile
            https://cdn.example.com/~user/file.pdf
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 4);
    }

    #[test]
    fn test_empty_input() {
        let urls = extract_urls_from_text("", None);
        assert!(urls.is_empty());

        let urls = extract_urls_from_text("   \n\t  ", None);
        assert!(urls.is_empty());
    }

    #[test]
    fn test_mixed_content() {
        let text = r#"
            Regular text with https://url1.com embedded.
            <a href="https://url2.com">Link</a>
            [Markdown](https://url3.com)
            Raw: https://url4.com
        "#;

        let urls = extract_urls_from_text(text, None);
        assert!(!urls.is_empty());
    }
}

#[cfg(test)]
mod html_extraction_tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use twars_url2md::url::extract_urls_from_html;

    #[test]
    fn test_extract_urls_from_simple_html() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com">Example</a>
                    <img src="https://images.com/pic.jpg" />
                    <script src="https://cdn.com/script.js"></script>
                </body>
            </html>
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert_eq!(urls.len(), 3);
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"https://images.com/pic.jpg".to_string()));
        assert!(urls.contains(&"https://cdn.com/script.js".to_string()));
    }

    #[test]
    fn test_extract_urls_with_data_attributes() {
        let html = r#"
            <div data-src="https://example.com/data1">
                <img data-href="https://example.com/data2" />
                <span data-url="https://example.com/data3"></span>
            </div>
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert_eq!(urls.len(), 3);
        assert!(urls.iter().all(|u| u.contains("example.com/data")));
    }

    #[test]
    fn test_extract_urls_from_srcset() {
        let html = r#"
            <img srcset="
                https://example.com/small.jpg 480w,
                https://example.com/medium.jpg 800w,
                https://example.com/large.jpg 1200w
            " />
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert_eq!(urls.len(), 3);
        assert!(urls.contains(&"https://example.com/small.jpg".to_string()));
        assert!(urls.contains(&"https://example.com/medium.jpg".to_string()));
        assert!(urls.contains(&"https://example.com/large.jpg".to_string()));
    }

    #[test]
    fn test_extract_relative_urls_with_base() {
        let html = r#"
            <a href="/relative/path">Relative</a>
            <a href="another/path">Another</a>
            <a href="https://absolute.com">Absolute</a>
        "#;

        let urls = extract_urls_from_html(html, Some("https://base.com")).unwrap();
        assert!(urls.contains(&"https://absolute.com".to_string()));
        assert!(urls.contains(&"https://base.com/relative/path".to_string()));
        assert!(urls.contains(&"https://base.com/another/path".to_string()));
    }

    #[test]
    fn test_malformed_html_handling() {
        let html = r#"
            <a href="https://example.com">Unclosed tag
            <img src="https://images.com/pic.jpg"
            Still finds https://text-url.com in text
        "#;

        let result = extract_urls_from_html(html, None);
        // Should handle malformed HTML gracefully
        assert!(result.is_ok());
        let urls = result.unwrap();
        assert!(!urls.is_empty());
    }

    #[test]
    fn test_ignore_non_http_urls_in_html() {
        let html = r#"
            <a href="javascript:void(0)">JS Link</a>
            <a href="mailto:test@example.com">Email</a>
            <a href="ftp://files.com">FTP</a>
            <a href="https://valid.com">Valid</a>
            <img src="data:image/png;base64,..." />
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert_eq!(urls.len(), 1);
        assert!(urls.contains(&"https://valid.com".to_string()));
    }

    #[test]
    fn test_html_with_url_in_text() {
        let html = r#"
            <p>Visit https://inline-url.com for more info</p>
            <div>
                Another URL: http://text-url.org
            </div>
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert!(!urls.is_empty());
        assert!(urls.contains(&"https://inline-url.com".to_string()));
        assert!(urls.contains(&"http://text-url.org".to_string()));
    }

    #[test]
    fn test_empty_html() {
        let urls = extract_urls_from_html("", None).unwrap();
        assert!(urls.is_empty());

        let urls = extract_urls_from_html("<html></html>", None).unwrap();
        assert!(urls.is_empty());
    }

    #[test]
    fn test_duplicate_urls_in_html() {
        let html = r#"
            <a href="https://example.com">Link 1</a>
            <a href="https://example.com">Link 2</a>
            <img src="https://example.com" />
        "#;

        let urls = extract_urls_from_html(html, None).unwrap();
        assert_eq!(urls.len(), 1);
        assert!(urls.contains(&"https://example.com".to_string()));
    }
}

#[cfg(test)]
mod output_path_tests {
    use super::*;
    use twars_url2md::url::create_output_path;

    #[test]
    fn test_create_output_path_simple() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://example.com")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.starts_with(temp_dir.path()));
        assert!(path.to_string_lossy().contains("example.com"));
        assert!(path.to_string_lossy().ends_with("index.md"));

        Ok(())
    }

    #[test]
    fn test_create_output_path_with_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://example.com/blog/post")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.to_string_lossy().contains("example.com"));
        assert!(path.to_string_lossy().contains("blog"));
        assert!(path.to_string_lossy().ends_with("post.md"));

        Ok(())
    }

    #[test]
    fn test_create_output_path_with_extension() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://example.com/page.html")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.to_string_lossy().ends_with("page.md"));
        assert!(!path.to_string_lossy().contains(".html"));

        Ok(())
    }

    #[test]
    fn test_create_output_path_trailing_slash() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://example.com/blog/")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.to_string_lossy().contains("blog"));
        assert!(path.to_string_lossy().ends_with("index.md"));

        Ok(())
    }

    #[test]
    fn test_create_output_path_complex() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://subdomain.example.com/path/to/deep/resource.php?query=123")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.to_string_lossy().contains("subdomain.example.com"));
        assert!(path.to_string_lossy().contains("path"));
        assert!(path.to_string_lossy().contains("to"));
        assert!(path.to_string_lossy().contains("deep"));
        assert!(path.to_string_lossy().ends_with("resource.md"));

        Ok(())
    }

    #[test]
    fn test_create_output_path_no_host() -> Result<()> {
        let temp_dir = TempDir::new()?;
        // File URLs have no host
        let url = Url::parse("file:///local/path/file.html")?;
        let path = create_output_path(&url, temp_dir.path())?;

        assert!(path.to_string_lossy().contains("unknown"));
        assert!(
            !path.exists(),
            "Path should not exist until content is written"
        );
        assert!(
            !path.parent().unwrap().exists(),
            "Directories are created lazily during write operations"
        );

        Ok(())
    }

    #[test]
    fn test_create_output_directories() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let url = Url::parse("https://example.com/a/b/c/file")?;
        let path = create_output_path(&url, temp_dir.path())?;

        // Verify computed structure without actually touching the filesystem yet
        assert!(
            path.to_string_lossy()
                .ends_with("example.com/a/b/c/file.md"),
            "Path should mirror URL structure"
        );
        assert!(
            !path.parent().unwrap().exists(),
            "Directories are created just-in-time during file writes"
        );

        Ok(())
    }
}

#[cfg(test)]
mod url_validation_tests {
    #[allow(unused_imports)]
    use super::*;
    use twars_url2md::url::extract_urls_from_text;

    #[test]
    fn test_url_normalization() {
        // Test that bare domain URLs have trailing slash removed
        let text = "https://example.com/ https://example.com";
        let urls = extract_urls_from_text(text, None);

        // Both should normalize to the same URL without trailing slash
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "https://example.com");
    }

    #[test]
    fn test_url_with_fragments() {
        let text = "https://example.com#section https://example.com#another";
        let urls = extract_urls_from_text(text, None);

        // LinkFinder strips fragments, so both URLs become the same
        // and deduplication results in a single URL
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "https://example.com");
    }

    #[test]
    fn test_url_with_query_params() {
        let text = r#"
            https://api.example.com/search?q=rust&page=1
            https://api.example.com/search?q=rust&page=2&sort=date
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 2);
        assert!(urls.iter().all(|u| u.contains("?")));
    }

    #[test]
    fn test_internationalized_domains() {
        let text = r#"
            https://例え.jp
            https://münchen.de
            https://xn--fsq.jp
        "#;

        let urls = extract_urls_from_text(text, None);
        // LinkFinder should handle IDN domains
        assert!(!urls.is_empty());
    }

    #[test]
    fn test_urls_with_ports() {
        let text = r#"
            https://example.com:8080
            http://localhost:3000/api
            https://192.168.1.1:443/admin
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 3);
        assert!(urls.iter().any(|u| u.contains(":8080")));
        assert!(urls.iter().any(|u| u.contains(":3000")));
    }

    #[test]
    fn test_urls_with_authentication() {
        let text = r#"
            https://user:pass@example.com/private
            https://token@api.example.com/v1
        "#;

        let urls = extract_urls_from_text(text, None);
        assert!(!urls.is_empty());
        // URLs with auth should be preserved
    }

    #[test]
    fn test_malformed_urls_rejected() {
        let text = r#"
            https://example.com/path with spaces
            https://example.com/<script>
            https://example.com/"quotes"
            https://example.com/`backticks`
            https://example.com/(parentheses)
            https://example.com/[brackets]
            https://example.com/{braces}
        "#;

        let urls = extract_urls_from_text(text, None);

        // Ensure none of the returned URLs contain characters that typically denote malformed input
        assert!(
            urls.iter().all(|u| {
                !u.contains(' ')
                    && !u.contains('<')
                    && !u.contains('"')
                    && !u.contains('`')
                    && !u.contains('(')
                    && !u.contains(')')
                    && !u.contains('[')
                    && !u.contains(']')
                    && !u.contains('{')
                    && !u.contains('}')
            }),
            "Returned URLs should be well-formed: {:?}",
            urls
        );
    }

    #[test]
    fn test_url_encoding() {
        let text = r#"
            https://example.com/path%20with%20spaces
            https://example.com/unicode/%E2%9C%93
            https://example.com/special/%2Fslash%2F
        "#;

        let urls = extract_urls_from_text(text, None);
        assert_eq!(urls.len(), 3);
        // URL-encoded characters should be preserved
        assert!(urls.iter().any(|u| u.contains("%20")));
        assert!(urls.iter().any(|u| u.contains("%E2%9C%93")));
        assert!(urls.iter().any(|u| u.contains("%2F")));
    }

    #[test]
    fn test_edge_case_urls() {
        let text = r#"
            https://
            http://a
            https://example.com:
            https://example.com//double//slash
            https://example.com/./dot/./path
            https://example.com/../parent/../path
        "#;

        let urls = extract_urls_from_text(text, None);
        // Some may be valid, some not - verify we handle them gracefully
        for url in &urls {
            assert!(url.starts_with("http://") || url.starts_with("https://"));
        }
    }

    #[test]
    fn test_very_long_urls() {
        let long_path = "a/".repeat(100);
        let text = format!("https://example.com/{}", long_path);

        let urls = extract_urls_from_text(&text, None);
        assert_eq!(urls.len(), 1);
        assert!(urls[0].len() > 200);
    }

    #[test]
    fn test_urls_with_unicode() {
        let text = r#"
            https://example.com/测试
            https://example.com/café
            https://example.com/🦀
            https://example.com/путь/файл
        "#;

        let urls = extract_urls_from_text(text, None);
        // URLs with unicode should be found (though they may be encoded)
        assert!(!urls.is_empty());
    }
}
