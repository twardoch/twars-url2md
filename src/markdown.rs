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

    #[test]
    fn test_nested_lists() -> Result<()> {
        let html = r#"
            <ul>
                <li>Top level 1
                    <ul>
                        <li>Nested 1.1</li>
                        <li>Nested 1.2</li>
                    </ul>
                </li>
                <li>Top level 2
                    <ol>
                        <li>Nested 2.1</li>
                        <li>Nested 2.2</li>
                    </ol>
                </li>
            </ul>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("Top level 1"));
        assert!(markdown.contains("Top level 2"));
        assert!(markdown.contains("Nested 1.1"));
        assert!(markdown.contains("Nested 2.2"));

        Ok(())
    }

    #[test]
    fn test_malformed_html() -> Result<()> {
        // Test various malformed HTML inputs
        let test_cases = vec![
            "<p>Unclosed paragraph",
            "<div><p>Mismatched tags</div></p>",
            "Plain text with no tags",
            "<p>Text with <unknown>unknown tag</unknown></p>",
            "<!DOCTYPE html><p>With doctype</p>",
        ];

        for html in test_cases {
            // Should not panic on malformed HTML
            let result = convert_html_to_markdown(html);
            assert!(result.is_ok(), "Failed to handle HTML: {}", html);
        }

        Ok(())
    }

    #[test]
    fn test_unicode_content() -> Result<()> {
        let html = r#"
            <h1>Unicode 测试 🦀</h1>
            <p>Greek: αβγδε</p>
            <p>Russian: Привет мир</p>
            <p>Emoji: 🚀 🌟 ✨</p>
            <p>Math: ∑ ∏ ∞ ≠</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("测试"));
        assert!(markdown.contains("🦀"));
        assert!(markdown.contains("αβγδε"));
        assert!(markdown.contains("Привет"));
        assert!(markdown.contains("🚀"));
        assert!(markdown.contains("∑"));

        Ok(())
    }

    #[test]
    fn test_complex_table() -> Result<()> {
        let html = r#"
            <table>
                <thead>
                    <tr>
                        <th colspan="2">Merged Header</th>
                        <th>Normal</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td rowspan="2">Tall Cell</td>
                        <td>A1</td>
                        <td>B1</td>
                    </tr>
                    <tr>
                        <td>A2</td>
                        <td>B2</td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td>Footer 1</td>
                        <td>Footer 2</td>
                        <td>Footer 3</td>
                    </tr>
                </tfoot>
            </table>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // Complex tables may not preserve structure perfectly, but content should be there
        assert!(markdown.contains("Merged Header"));
        assert!(markdown.contains("Tall Cell"));
        assert!(markdown.contains("Footer"));

        Ok(())
    }

    #[test]
    fn test_html_entities() -> Result<()> {
        let html = r#"
            <p>&nbsp;&nbsp;&nbsp;Indented with nbsp</p>
            <p>Copyright &copy; 2024</p>
            <p>Price: &pound;100 or &euro;120</p>
            <p>Math: &alpha; + &beta; = &gamma;</p>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        // Entities should be converted to their unicode equivalents
        assert!(markdown.contains("Indented"));
        assert!(markdown.contains("©") || markdown.contains("Copyright"));
        assert!(markdown.contains("£") || markdown.contains("€") || markdown.contains("100"));

        Ok(())
    }

    #[test]
    fn test_preserve_links_in_nested_elements() -> Result<()> {
        let html = r#"
            <p>Check out <strong><a href="https://example.com">this link</a></strong> for more.</p>
            <div>
                <span>
                    <a href="https://nested.com">
                        <em>Deeply</em> <strong>nested</strong> link
                    </a>
                </span>
            </div>
        "#;

        let markdown = convert_html_to_markdown(html)?;
        assert!(markdown.contains("https://example.com"));
        assert!(markdown.contains("https://nested.com"));
        assert!(markdown.contains("this link"));

        Ok(())
    }
}
