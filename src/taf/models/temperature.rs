//! Module `temperature`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Stores parsed temperature-related values for TafTemperature.
pub struct TafTemperature {
    pub value: i8,
    pub day: u8,
    pub hour: u8,
}
