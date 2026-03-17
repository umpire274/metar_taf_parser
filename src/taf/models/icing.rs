//! Icing forecast types for TAF messages.
//!
//! Represents an icing layer group encoded as `6ABBBC` in TAF forecasts,
//! where `A` is the intensity code, `BBB` is the base altitude in hundreds
//! of feet, and `C` is the layer thickness in thousands of feet.

use serde::Serialize;

/// Icing intensity, as defined by ICAO Annex 3.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum IcingIntensity {
    /// No icing (code 0).
    None,
    /// Light icing (code 1).
    Light,
    /// Moderate icing, mixed or rime (code 2).
    ModerateMixedOrRime,
    /// Moderate icing, glaze (code 3).
    ModerateGlaze,
    /// Severe icing (code 4).
    Severe,
    /// Reserved or unrecognised code.
    Unknown(u8),
}

impl IcingIntensity {
    /// Converts a single-digit ICAO intensity code to an [`IcingIntensity`] variant.
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => Self::None,
            1 => Self::Light,
            2 => Self::ModerateMixedOrRime,
            3 => Self::ModerateGlaze,
            4 => Self::Severe,
            other => Self::Unknown(other),
        }
    }
}

/// A parsed icing layer from a TAF forecast block.
///
/// Decoded from a 6-character group of the form `6ABBBC`:
/// - `6` – icing type indicator
/// - `A` – intensity code (see [`IcingIntensity`])
/// - `BBB` – base altitude in hundreds of feet
/// - `C` – layer thickness in thousands of feet
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Icing {
    /// Icing intensity classification.
    pub intensity: IcingIntensity,
    /// Base altitude in feet.
    pub base_ft: u16,
    /// Layer thickness in feet.
    pub thickness_ft: u16,
}
