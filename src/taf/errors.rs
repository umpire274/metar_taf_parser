//! Module `errors`.
//!
//! Contains types and parsing logic implemented for this crate.
use thiserror::Error;

#[derive(Debug, Error)]
/// Enumerates the allowed values for TafError.
pub enum TafError {
    #[error("invalid TAF format")]
    InvalidFormat,

    #[error("unsupported group: {0}")]
    UnsupportedGroup(String),
}
