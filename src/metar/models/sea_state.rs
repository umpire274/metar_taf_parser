//! Sea state group for weather stations located at sea.
//!
//! Stations at sea may report the sea water temperature and wave state
//! using a token of the form `W[TT]/[S|H][HH]`, e.g. `W12/S8`.
use serde::Serialize;

/// Indicates how the wave height is encoded.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum WaveHeightKind {
    /// WMO state-of-sea code 0–9 (`S` prefix). Each digit corresponds to a
    /// qualitative sea state (0 = glassy, 9 = phenomenal).
    StateCode,
    /// Significant wave height in decimetres (`H` prefix).
    HeightDm,
}

/// Sea state reported by an offshore weather station.
///
/// Parsed from a token of the form:
/// - `W12/S8`  — water 12 °C, WMO state code 8 (very rough)
/// - `WM2/S3`  — water −2 °C, WMO state code 3 (slight)
/// - `W25/H15` — water 25 °C, significant wave height 15 dm
/// - `W//S/`   — temperature and wave state not available
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SeaState {
    /// Sea water temperature in °C. `None` when reported as `//`.
    pub water_temperature: Option<i8>,
    /// How the wave value is encoded.
    pub wave_kind: WaveHeightKind,
    /// Wave state code (0–9) or significant wave height in dm.
    /// `None` when reported as `/`.
    pub wave_value: Option<u8>,
}

