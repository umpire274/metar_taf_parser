//! Module `weather`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for WeatherIntensity.
pub enum WeatherIntensity {
    Light,
    Moderate,
    Heavy,
    /// Recent phenomenon (prefix `RE`), no longer occurring at observation time.
    Recent,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for WeatherDescriptor.
pub enum WeatherDescriptor {
    Shallow,      // MI
    Partial,      // PR
    Patches,      // BC
    LowDrifting,  // DR
    Blowing,      // BL
    Showers,      // SH
    Thunderstorm, // TS
    Freezing,     // FZ
    Vicinity,     // VC
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for WeatherPhenomenon.
pub enum WeatherPhenomenon {
    Rain,                   // RA
    Snow,                   // SN
    Drizzle,                // DZ
    Thunder,                // TS (standalone)
    Fog,                    // FG
    Mist,                   // BR
    Haze,                   // HZ
    Smoke,                  // FU
    Hail,                   // GR
    SmallHail,              // GS
    IcePellets,             // PL (and legacy PE)
    IceCrystals,            // IC
    SnowGrains,             // SG
    SandWhirls,             // PO
    Squalls,                // SQ
    FunnelCloud,            // FC
    Sand,                   // SA
    Dust,                   // DU
    DustStorm,              // DS
    SandStorm,              // SS
    Spray,                  // PY
    VolcanicAsh,            // VA
    UnknownPrecipitation,   // UP
    NoSignificantWeather,   // NSW
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Defines the Weather domain model used by the parser.
pub struct Weather {
    pub intensity: Option<WeatherIntensity>,
    pub descriptors: Vec<WeatherDescriptor>,
    pub phenomena: Vec<WeatherPhenomenon>,
}
