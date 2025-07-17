// this_file: tests/benchmarks.rs

#[cfg(test)]
mod benchmark_tests {
    use anyhow::Result;
    use std::time::Instant;
    use tempfile::TempDir;
    use twars_url2md::url::{extract_urls_from_html, extract_urls_from_text};

    #[test]
    fn bench_url_extraction_large_text() -> Result<()> {
        // Create a large text with many URLs
        let mut large_text = String::new();
        for i in 0..1000 {
            large_text.push_str(&format!("Visit https://example{}.com for more info.\n", i));
        }

        let start = Instant::now();
        let urls = extract_urls_from_text(&large_text, None);
        let duration = start.elapsed();

        println!("Extracted {} URLs in {:?}", urls.len(), duration);
        assert_eq!(urls.len(), 1000);
        assert!(
            duration.as_millis() < 1000,
            "Should extract URLs within 1 second"
        );

        Ok(())
    }

    #[test]
    fn bench_html_extraction_complex() -> Result<()> {
        // Create complex HTML with many elements
        let mut html = String::from("<html><body>");
        for i in 0..500 {
            html.push_str(&format!(
                r#"<div>
                    <a href="https://link{}.com">Link {}</a>
                    <img src="https://img{}.com/pic.jpg" data-src="https://lazy{}.com/pic.jpg" />
                    <script src="https://script{}.com/app.js"></script>
                </div>"#,
                i, i, i, i, i
            ));
        }
        html.push_str("</body></html>");

        let start = Instant::now();
        let urls = extract_urls_from_html(&html, None)?;
        let duration = start.elapsed();

        println!(
            "Extracted {} URLs from complex HTML in {:?}",
            urls.len(),
            duration
        );
        assert!(urls.len() >= 1500); // At least 3 URLs per iteration
        assert!(
            duration.as_millis() < 2000,
            "Should extract URLs within 2 seconds"
        );

        Ok(())
    }

    #[test]
    fn bench_duplicate_detection() -> Result<()> {
        // Create text with many duplicate URLs
        let mut text = String::new();
        for _i in 0..100 {
            text.push_str("https://example.com\n");
            text.push_str("https://test.org\n");
            text.push_str("https://rust-lang.org\n");
        }

        let start = Instant::now();
        let urls = extract_urls_from_text(&text, None);
        let duration = start.elapsed();

        println!("Deduplicated {} URLs in {:?}", urls.len(), duration);
        assert_eq!(urls.len(), 3);
        assert!(
            duration.as_millis() < 100,
            "Should deduplicate within 100ms"
        );

        Ok(())
    }

    #[test]
    fn bench_mixed_content_extraction() -> Result<()> {
        // Create mixed content with text, HTML, and markdown
        let mut content = String::new();
        for i in 0..200 {
            content.push_str(&format!("Plain text URL: https://plain{}.com\n", i));
            content.push_str(&format!("[Markdown Link](https://md{}.com)\n", i));
            content.push_str(&format!(
                "<a href=\"https://html{}.com\">HTML Link</a>\n",
                i
            ));
        }

        let start = Instant::now();
        let urls = extract_urls_from_text(&content, None);
        let duration = start.elapsed();

        println!(
            "Extracted {} URLs from mixed content in {:?}",
            urls.len(),
            duration
        );
        assert!(urls.len() >= 500); // At least 3 URLs per iteration
        assert!(
            duration.as_millis() < 500,
            "Should extract URLs within 500ms"
        );

        Ok(())
    }

    #[test]
    fn bench_memory_usage() -> Result<()> {
        // Test with very large input to check memory efficiency
        let mut large_content = String::new();
        for i in 0..10000 {
            large_content.push_str(&format!(
                "Here's a URL: https://site{}.com/path/to/resource?id={}&type=test\n",
                i, i
            ));
        }

        let start = Instant::now();
        let urls = extract_urls_from_text(&large_content, None);
        let duration = start.elapsed();

        println!(
            "Memory test: {} URLs extracted in {:?}",
            urls.len(),
            duration
        );
        assert_eq!(urls.len(), 10000);
        assert!(
            duration.as_millis() < 5000,
            "Should handle large input within 5 seconds"
        );

        Ok(())
    }

    #[test]
    fn bench_concurrent_processing() -> Result<()> {
        use std::sync::Arc;
        use std::thread;

        let text = Arc::new(String::from(
            "Check out https://example.com and https://test.org for more info.",
        ));

        let start = Instant::now();
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let text = Arc::clone(&text);
                thread::spawn(move || {
                    for _i in 0..100 {
                        let _urls = extract_urls_from_text(&text, None);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let duration = start.elapsed();
        println!("Concurrent processing completed in {:?}", duration);
        assert!(
            duration.as_millis() < 5000,
            "Should handle concurrent processing within 5 seconds"
        );

        Ok(())
    }

    #[tokio::test]
    async fn bench_output_path_creation() -> Result<()> {
        use twars_url2md::url::create_output_path;
        use url::Url;

        let temp_dir = TempDir::new()?;
        let start = Instant::now();

        // Create many output paths
        for i in 0..1000 {
            let url = Url::parse(&format!("https://example{}.com/path/{}/file", i, i))?;
            let _path = create_output_path(&url, temp_dir.path())?;
        }

        let duration = start.elapsed();
        println!("Created 1000 output paths in {:?}", duration);
        assert!(
            duration.as_millis() < 1000,
            "Should create paths within 1 second"
        );

        Ok(())
    }

    #[test]
    fn bench_regex_performance() -> Result<()> {
        // Test regex performance on various inputs
        let test_cases = vec![
            "https://example.com/simple",
            "https://example.com/path/with/many/segments/and/file.html?param=value&other=123#fragment",
            "https://user:pass@subdomain.example.com:8080/complex/path/to/resource.php?query=test&id=123",
            "/local/file/path.html",
            "file:///absolute/path/to/file.html",
        ];

        let start = Instant::now();
        for _i in 0..1000 {
            for case in &test_cases {
                let _urls = extract_urls_from_text(case, None);
            }
        }
        let duration = start.elapsed();

        println!("Regex performance test completed in {:?}", duration);
        assert!(
            duration.as_millis() < 1000,
            "Should handle regex matching within 1 second"
        );

        Ok(())
    }

    #[test]
    fn bench_url_validation() -> Result<()> {
        // Test URL validation performance
        let test_urls = vec![
            "https://example.com",
            "http://test.org/path",
            "https://subdomain.example.com/path/to/resource",
            "https://example.com:8080/path",
            "https://user:pass@example.com/path",
            "https://example.com/path?query=value",
            "https://example.com/path#fragment",
            "ftp://invalid.com",
            "javascript:void(0)",
            "data:text/plain,test",
            "mailto:test@example.com",
            "not-a-url",
        ];

        let start = Instant::now();
        for _i in 0..1000 {
            for url in &test_urls {
                let _urls = extract_urls_from_text(url, None);
            }
        }
        let duration = start.elapsed();

        println!(
            "URL validation performance test completed in {:?}",
            duration
        );
        assert!(
            duration.as_millis() < 500,
            "Should validate URLs within 500ms"
        );

        Ok(())
    }

    #[test]
    fn bench_large_html_document() -> Result<()> {
        // Simulate a large HTML document
        let mut html = String::from(
            r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Large Document</title>
            <link rel="stylesheet" href="https://styles.example.com/main.css">
            <script src="https://scripts.example.com/app.js"></script>
        </head>
        <body>"#,
        );

        // Add many elements
        for i in 0..1000 {
            html.push_str(&format!(
                r#"<article>
                    <h2><a href="https://blog.example.com/post/{}">Post {}</a></h2>
                    <p>Content with <a href="https://ref{}.com">reference</a></p>
                    <img src="https://images.example.com/thumb/{}.jpg" 
                         srcset="https://images.example.com/small/{}.jpg 480w,
                                 https://images.example.com/medium/{}.jpg 800w,
                                 https://images.example.com/large/{}.jpg 1200w">
                    <footer>
                        <a href="https://social.example.com/share?url=post{}">Share</a>
                        <a href="https://comments.example.com/post/{}">Comments</a>
                    </footer>
                </article>"#,
                i, i, i, i, i, i, i, i, i
            ));
        }

        html.push_str("</body></html>");

        let start = Instant::now();
        let urls = extract_urls_from_html(&html, None)?;
        let duration = start.elapsed();

        println!(
            "Extracted {} URLs from large HTML document in {:?}",
            urls.len(),
            duration
        );
        assert!(urls.len() >= 8000); // At least 8 URLs per article
        assert!(
            duration.as_millis() < 5000,
            "Should handle large HTML within 5 seconds"
        );

        Ok(())
    }
}
