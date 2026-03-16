//! Module `errors`.
//!
//! Contains types and parsing logic implemented for this crate.
use thiserror::Error;

#[derive(Debug, Error)]
/// Enumerates the allowed values for MetarError.
pub enum MetarError {
    #[error("invalid METAR format")]
    InvalidFormat,

    #[error("unsupported or unknown group: {0}")]
    UnknownGroup(String),
}
