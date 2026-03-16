//! Module `wind_shear`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Stores parsed wind-related values for TafWindShear.
pub struct TafWindShear {
    pub height_hundreds_ft: u16,
    pub direction: u16,
    pub speed_kt: u16,
}
