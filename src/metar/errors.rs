use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetarError {
    #[error("invalid METAR format")]
    InvalidFormat,

    #[error("unsupported or unknown group: {0}")]
    UnknownGroup(String),
}
