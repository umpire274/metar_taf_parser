//! Turbulence forecast types for TAF messages.
//!
//! Represents a turbulence layer group encoded as `5ABBBC` in TAF forecasts,
//! where `A` is the intensity code, `BBB` is the base altitude in hundreds
//! of feet, and `C` is the layer thickness in thousands of feet.

use serde::Serialize;

/// Turbulence intensity, as defined by ICAO Annex 3.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TurbulenceIntensity {
    /// No turbulence (code 0).
    None,
    /// Light turbulence (code 1).
    Light,
    /// Moderate turbulence, in-cloud (code 2).
    ModerateInCloud,
    /// Moderate turbulence, clear air (code 3).
    ModerateClearAir,
    /// Severe turbulence, in-cloud (code 4).
    SevereInCloud,
    /// Severe turbulence, clear air (code 5).
    SevereClearAir,
    /// Extreme turbulence (code 6).
    Extreme,
    /// Reserved or unrecognised code.
    Unknown(u8),
}

impl TurbulenceIntensity {
    /// Converts a single-digit ICAO intensity code to a [`TurbulenceIntensity`] variant.
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => Self::None,
            1 => Self::Light,
            2 => Self::ModerateInCloud,
            3 => Self::ModerateClearAir,
            4 => Self::SevereInCloud,
            5 => Self::SevereClearAir,
            6 => Self::Extreme,
            other => Self::Unknown(other),
        }
    }
}

/// A parsed turbulence layer from a TAF forecast block.
///
/// Decoded from a 6-character group of the form `5ABBBC`:
/// - `5` – turbulence type indicator
/// - `A` – intensity code (see [`TurbulenceIntensity`])
/// - `BBB` – base altitude in hundreds of feet
/// - `C` – layer thickness in thousands of feet
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Turbulence {
    /// Turbulence intensity classification.
    pub intensity: TurbulenceIntensity,
    /// Base altitude in feet.
    pub base_ft: u16,
    /// Layer thickness in feet.
    pub thickness_ft: u16,
}

