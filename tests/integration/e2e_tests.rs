// this_file: tests/integration/e2e_tests.rs

use anyhow::Result;
use tempfile::TempDir;
use twars_url2md::{process_urls, Config};

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    #[tokio::test]
    async fn test_single_url_processing() -> Result<()> {
        let _m = mockito::mock("GET", "/simple.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(r#"
                <html>
                    <body>
                        <h1>Simple Page</h1>
                        <p>This is a simple test page.</p>
                    </body>
                </html>
            "#)
            .create();

        let temp_dir = TempDir::new()?;
        let url = format!("{}/simple.html", mockito::server_url());
        
        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(vec![url.clone()], config).await?;
        assert!(errors.is_empty());

        // Check that the file was created
        let host = mockito::server_url().replace("http://", "");
        let expected_path = temp_dir.path().join(&host).join("simple.md");
        assert!(expected_path.exists());

        // Check content
        let content = tokio::fs::read_to_string(&expected_path).await?;
        assert!(content.contains("Simple Page"));
        assert!(content.contains("simple test page"));

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_urls_processing() -> Result<()> {
        let _m1 = mockito::mock("GET", "/page1.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Page 1</h1></body></html>")
            .create();

        let _m2 = mockito::mock("GET", "/page2.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Page 2</h1></body></html>")
            .create();

        let _m3 = mockito::mock("GET", "/page3.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Page 3</h1></body></html>")
            .create();

        let temp_dir = TempDir::new()?;
        let urls = vec![
            format!("{}/page1.html", mockito::server_url()),
            format!("{}/page2.html", mockito::server_url()),
            format!("{}/page3.html", mockito::server_url()),
        ];

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(urls, config).await?;
        assert!(errors.is_empty());

        // Check all files were created
        let host = mockito::server_url().replace("http://", "");
        for i in 1..=3 {
            let path = temp_dir.path().join(&host).join(format!("page{}.md", i));
            assert!(path.exists());
            let content = tokio::fs::read_to_string(&path).await?;
            assert!(content.contains(&format!("Page {}", i)));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_packed_output() -> Result<()> {
        let _m1 = mockito::mock("GET", "/doc1.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Document 1</h1><p>Content 1</p></body></html>")
            .create();

        let _m2 = mockito::mock("GET", "/doc2.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Document 2</h1><p>Content 2</p></body></html>")
            .create();

        let temp_dir = TempDir::new()?;
        let pack_file = temp_dir.path().join("packed.md");
        let urls = vec![
            format!("{}/doc1.html", mockito::server_url()),
            format!("{}/doc2.html", mockito::server_url()),
        ];

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: Some(pack_file.clone()),
        };

        let errors = process_urls(urls, config).await?;
        assert!(errors.is_empty());

        // Check packed file exists and contains both documents
        assert!(pack_file.exists());
        let content = tokio::fs::read_to_string(&pack_file).await?;
        assert!(content.contains("Document 1"));
        assert!(content.contains("Content 1"));
        assert!(content.contains("Document 2"));
        assert!(content.contains("Content 2"));

        Ok(())
    }

    #[tokio::test]
    async fn test_error_handling() -> Result<()> {
        let _m = mockito::mock("GET", "/error.html")
            .with_status(500)
            .with_body("Internal Server Error")
            .expect(4) // Initial try + 3 retries
            .create();

        let temp_dir = TempDir::new()?;
        let url = format!("{}/error.html", mockito::server_url());

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(vec![url.clone()], config).await?;
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].0, url);

        Ok(())
    }

    #[tokio::test]
    async fn test_mixed_success_and_failure() -> Result<()> {
        let _m1 = mockito::mock("GET", "/success.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Success</h1></body></html>")
            .create();

        let _m2 = mockito::mock("GET", "/failure.html")
            .with_status(404)
            .expect(4) // Initial try + 3 retries
            .create();

        let temp_dir = TempDir::new()?;
        let urls = vec![
            format!("{}/success.html", mockito::server_url()),
            format!("{}/failure.html", mockito::server_url()),
        ];

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(urls.clone(), config).await?;
        assert_eq!(errors.len(), 1);
        assert!(errors[0].0.contains("failure.html"));

        // Success file should exist
        let host = mockito::server_url().replace("http://", "");
        let success_path = temp_dir.path().join(&host).join("success.md");
        assert!(success_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_single_file_output() -> Result<()> {
        let _m = mockito::mock("GET", "/content.html")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Content</h1></body></html>")
            .create();

        let temp_dir = TempDir::new()?;
        let output_file = temp_dir.path().join("output.md");
        let url = format!("{}/content.html", mockito::server_url());

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: output_file.clone(),
            single_file: true,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(vec![url], config).await?;
        assert!(errors.is_empty());

        // Check file was created at specified path
        assert!(output_file.exists());
        let content = tokio::fs::read_to_string(&output_file).await?;
        assert!(content.contains("Content"));

        Ok(())
    }

    #[tokio::test]
    async fn test_non_html_content_skipped() -> Result<()> {
        let urls = vec![
            "https://example.com/image.jpg".to_string(),
            "https://example.com/document.pdf".to_string(),
            "https://example.com/video.mp4".to_string(),
        ];

        let temp_dir = TempDir::new()?;
        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: temp_dir.path().to_path_buf(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        // These should be skipped without errors
        let _errors = process_urls(urls, config).await?;
        
        // No files should be created for non-HTML content
        assert!(temp_dir.path().read_dir()?.next().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_local_file_processing() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let html_file = temp_dir.path().join("local.html");
        tokio::fs::write(&html_file, "<html><body><h1>Local File</h1></body></html>").await?;

        let url = format!("file://{}", html_file.display());
        let output_dir = temp_dir.path().join("output");

        let config = Config {
            verbose: false,
            max_retries: 3,
            output_base: output_dir.clone(),
            single_file: false,
            has_output: true,
            pack_file: None,
        };

        let errors = process_urls(vec![url], config).await?;
        assert!(errors.is_empty());

        // Check output was created
        let output_files: Vec<_> = std::fs::read_dir(&output_dir)?
            .filter_map(|e| e.ok())
            .collect();
        assert!(!output_files.is_empty());

        Ok(())
    }
}