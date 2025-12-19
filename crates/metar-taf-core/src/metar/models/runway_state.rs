use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RunwayState {
    /// Runway designator (two digits)
    pub runway_designator: String,

    /// Deposit type (ICAO code: "0"–"9" or "/")
    pub deposit_type: Option<u8>,

    /// Extent of contamination (ICAO code: "0"–"9" or "/")
    pub contamination_extent: Option<u8>,

    /// Depth of deposit (ICAO code: "00"–"99" or "//")
    pub deposit_depth: Option<String>,

    /// Friction coefficient or braking action
    pub braking_action: Option<String>,
}
