//! Module `runway_state`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Defines the RunwayState domain model used by the parser.
pub struct RunwayState {
    /// Runway designator (e.g. `"05"`, `"23L"`).
    /// Empty string when the special `R/SNOCLO` token is decoded.
    pub runway_designator: String,

    /// `true` when the token is `R/SNOCLO` (airfield closed due to snow/ice).
    /// All other fields are `None` in this case.
    pub snoclo: bool,

    /// Deposit type (ICAO code 0–9, or `None` for `/`).
    pub deposit_type: Option<u8>,

    /// Extent of contamination coverage (ICAO code 1/2/5/9, or `None` for `/`).
    pub coverage: Option<u8>,

    /// Depth of deposit / thickness (ICAO code `"00"`–`"99"`, or `None` for `"//"`).
    pub thickness: Option<String>,

    /// Friction coefficient or braking action code (or `None` for `"//"`).
    pub braking_action: Option<String>,
}
