//! Module `runway_state`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Defines the RunwayState domain model used by the parser.
pub struct RunwayState {
    /// Runway designator (two digits)
    pub runway_designator: String,

    /// Deposit type (ICAO code: "0"–"9" or "/")
    pub deposit_type: Option<u8>,

    /// Extent of contamination coverage (ICAO code: "0"–"9" or "/")
    pub coverage: Option<u8>,

    /// Depth of deposit / thickness (ICAO code: "00"–"99" or "//")
    pub thickness: Option<String>,

    /// Friction coefficient or braking action
    pub braking_action: Option<String>,
}
