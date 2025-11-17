use thiserror::Error;

#[derive(Error, Debug)]
pub enum RssIngestionError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("RSS parse error: {0}")]
    Parse(#[from] RssParseError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Unexpected error: {0}")]
    Other(String),
}

#[derive(Error, Debug)]
pub enum RssParseError {
    #[error("XML parsing error: {0}")]
    Xml(String),

    #[error("Date parsing error: {0}")]
    InvalidDate(String),
}
