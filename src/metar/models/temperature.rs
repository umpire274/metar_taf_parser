//! Module `temperature`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Stores parsed temperature-related values for Temperature.
pub struct Temperature {
    pub temperature: i8,
    pub dew_point: i8,
}
