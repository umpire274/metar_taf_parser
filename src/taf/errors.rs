use thiserror::Error;

#[derive(Debug, Error)]
pub enum TafError {
    #[error("invalid TAF format")]
    InvalidFormat,

    #[error("unsupported group: {0}")]
    UnsupportedGroup(String),
}
