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

/// Variable wind direction range reported alongside the main wind group.
///
/// Appears in METAR as `dddVddd`, e.g. `180V240` means the wind is veering
/// between 180° and 240°.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WindVariation {
    /// Minimum variable direction in degrees (0–360).
    pub min: u16,
    /// Maximum variable direction in degrees (0–360).
    pub max: u16,
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

    /// Variable wind direction range, when reported (e.g. `180V240`).
    pub variation: Option<WindVariation>,
}
