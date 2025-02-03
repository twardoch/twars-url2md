use std::fmt;

/// Custom error type for URL processing
#[derive(Debug)]
pub enum Error {
    /// Error occurred while fetching URL
    Fetch(String),
    /// Error occurred while processing HTML
    Html(String),
    /// Error occurred while converting to Markdown
    Markdown(String),
    /// Error occurred while writing output
    Output(String),
    /// Other errors
    Other(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Fetch(msg) => write!(f, "Failed to fetch URL: {}", msg),
            Error::Html(msg) => write!(f, "Failed to process HTML: {}", msg),
            Error::Markdown(msg) => write!(f, "Failed to convert to Markdown: {}", msg),
            Error::Output(msg) => write!(f, "Failed to write output: {}", msg),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Fetch(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Output(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}
