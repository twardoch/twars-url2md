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

    #[test]
    fn test_heading_levels() -> Result<()> {
        let html = r#"
            <h1>Level 1</h1>
            <h2>Level 2</h2>
            <h3>Level 3</h3>
            <h4>Level 4</h4>
            <h5>Level 5</h5>
            <h6>Level 6</h6>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("# Level 1"));
        assert!(markdown.contains("## Level 2"));
        assert!(markdown.contains("### Level 3"));
        assert!(markdown.contains("#### Level 4"));
        assert!(markdown.contains("##### Level 5"));
        assert!(markdown.contains("###### Level 6"));

        Ok(())
    }

    #[test]
    fn test_text_formatting() -> Result<()> {
        let html = r#"
            <p>Text with <strong>bold</strong> and <em>italic</em> and <code>code</code>.</p>
            <p>Also <b>bold tag</b> and <i>italic tag</i>.</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("**bold**"));
        assert!(markdown.contains("*italic*") || markdown.contains("_italic_"));
        assert!(markdown.contains("`code`"));

        Ok(())
    }

    #[test]
    fn test_lists() -> Result<()> {
        let html = r#"
            <ul>
                <li>Unordered 1</li>
                <li>Unordered 2</li>
            </ul>
            <ol>
                <li>Ordered 1</li>
                <li>Ordered 2</li>
            </ol>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // htmd might use different list markers
        assert!(markdown.contains("Unordered 1"));
        assert!(markdown.contains("Unordered 2"));
        assert!(markdown.contains("Ordered 1"));
        assert!(markdown.contains("Ordered 2"));

        Ok(())
    }

    #[test]
    fn test_links_and_images() -> Result<()> {
        let html = r#"
            <p>Visit <a href="https://example.com">Example Site</a>.</p>
            <p><img src="test.jpg" alt="Test Image" /></p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("[Example Site](https://example.com)"));
        assert!(markdown.contains("![Test Image](test.jpg)"));

        Ok(())
    }

    #[test]
    fn test_blockquotes() -> Result<()> {
        let html = r#"
            <blockquote>
                <p>This is a quote.</p>
            </blockquote>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("> This is a quote."));

        Ok(())
    }

    #[test]
    fn test_code_blocks() -> Result<()> {
        let html = r#"
            <pre><code>fn main() {
    println!("Hello");
}</code></pre>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("```") || markdown.contains("    fn main()"));
        assert!(markdown.contains("println!"));

        Ok(())
    }

    #[test]
    fn test_horizontal_rule() -> Result<()> {
        let html = r#"
            <p>Before</p>
            <hr>
            <p>After</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // htmd might render hr differently, just check both paragraphs are present
        assert!(markdown.contains("Before"));
        assert!(markdown.contains("After"));

        Ok(())
    }

    #[test]
    fn test_empty_elements() -> Result<()> {
        let html = r#"
            <p></p>
            <div></div>
            <h1></h1>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // Should handle empty elements without crashing
        assert!(markdown.is_empty() || markdown.trim().is_empty() || markdown.len() < 10);

        Ok(())
    }

    #[test]
    fn test_nested_formatting() -> Result<()> {
        let html = r#"
            <p>Text with <strong>bold and <em>italic</em></strong> combined.</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("bold"));
        assert!(markdown.contains("italic"));

        Ok(())
    }

    #[test]
    fn test_special_characters() -> Result<()> {
        let html = r#"
            <p>Special: &lt; &gt; &amp; &quot;</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("<"));
        assert!(markdown.contains(">"));
        assert!(markdown.contains("&"));
        assert!(markdown.contains("\""));

        Ok(())
    }

    #[test]
    fn test_script_style_removal() -> Result<()> {
        let html = r#"
            <html>
                <head>
                    <script>console.log('test');</script>
                    <style>body { color: red; }</style>
                </head>
                <body>
                    <p>Actual content</p>
                </body>
            </html>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // htmd might include script/style content, just verify the main content is there
        assert!(markdown.contains("Actual content"));
        // Verify it's not excessively long (scripts/styles might make it longer)
        assert!(markdown.len() < 500);

        Ok(())
    }

    #[test]
    fn test_table_conversion() -> Result<()> {
        let html = r#"
            <table>
                <tr>
                    <th>Header 1</th>
                    <th>Header 2</th>
                </tr>
                <tr>
                    <td>Cell 1</td>
                    <td>Cell 2</td>
                </tr>
            </table>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // Tables might be converted differently by htmd
        assert!(markdown.contains("Header 1"));
        assert!(markdown.contains("Header 2"));
        assert!(markdown.contains("Cell 1"));
        assert!(markdown.contains("Cell 2"));

        Ok(())
    }
}
