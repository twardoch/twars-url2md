// this_file: tests/common/mod.rs

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Create a temporary directory with test files
pub fn setup_test_dir() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path().to_path_buf();
    (temp_dir, base_path)
}

/// Create a test file with given content
pub fn create_test_file(dir: &Path, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

/// Read fixture file content
pub fn read_fixture(fixture_path: &str) -> String {
    let path = PathBuf::from("tests/fixtures").join(fixture_path);
    fs::read_to_string(path).expect("Failed to read fixture file")
}

/// Compare markdown output, ignoring minor formatting differences
pub fn assert_markdown_equivalent(actual: &str, expected: &str) {
    let normalize = |s: &str| {
        s.trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    };

    let actual_normalized = normalize(actual);
    let expected_normalized = normalize(expected);

    assert_eq!(
        actual_normalized, 
        expected_normalized,
        "\nActual:\n{}\n\nExpected:\n{}\n",
        actual,
        expected
    );
}

/// Create a mock HTTP response for testing
pub fn mock_html_response() -> String {
    r#"<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body>
    <h1>Test Page</h1>
    <p>This is a test page.</p>
</body>
</html>"#.to_string()
}

/// Get test URLs for integration testing
pub fn test_urls() -> Vec<String> {
    vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
        "https://github.com/test/repo".to_string(),
    ]
}

/// Verify that a file exists and contains expected content
pub fn assert_file_contains(path: &Path, expected_content: &str) {
    assert!(path.exists(), "File does not exist: {}", path.display());
    let content = fs::read_to_string(path).expect("Failed to read file");
    assert!(
        content.contains(expected_content),
        "File {} does not contain expected content: {}",
        path.display(),
        expected_content
    );
}