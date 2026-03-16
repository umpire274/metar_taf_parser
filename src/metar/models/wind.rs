//! Module `wind`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Enumerates the allowed values for WindUnit.
pub enum WindUnit {
    KT,
    MPS,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Stores parsed wind-related values for Wind.
pub struct Wind {
    /// Wind direction in degrees, None for VRB
    pub direction: Option<u16>,

    /// Wind speed in the given unit
    pub speed: u16,

    /// Gust speed in the given unit
    pub gust: Option<u16>,

    /// Unit of measure (KT or MPS)
    pub unit: WindUnit,
}
