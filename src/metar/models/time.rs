//! Module `time`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
/// Represents a parsed MetarTime report with typed fields.
pub struct MetarTime {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}
