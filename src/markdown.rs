use anyhow::{Context, Result};

/// Convert HTML to Markdown
pub fn convert_html_to_markdown(html: &str) -> Result<String> {
    htmd::convert(html).context("Failed to convert HTML to Markdown")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html_to_markdown() -> Result<()> {
        let html = r#"
            <html>
                <body>
                    <h1>Test</h1>
                    <p>Hello, world!</p>
                </body>
            </html>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("# Test"));
        assert!(markdown.contains("Hello, world!"));

        Ok(())
    }
}
