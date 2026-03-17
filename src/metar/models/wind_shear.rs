//! Wind shear runway groups for METAR reports.
//!
//! METAR wind shear differs from TAF wind shear: it identifies only the
//! affected runway(s) without specifying altitude, direction, or speed.
use serde::Serialize;

/// Identifies the runway(s) affected by wind shear in a METAR report.
///
/// Parsed from two- or three-token groups:
/// - `WS R23`      → [`MetarWindShearRunway::Runway`]`("23")`
/// - `WS R23L`     → [`MetarWindShearRunway::Runway`]`("23L")`
/// - `WS ALL RWY`  → [`MetarWindShearRunway::AllRunways`]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MetarWindShearRunway {
    /// Wind shear reported for a specific runway designator (e.g. `"23"`, `"06R"`).
    Runway(String),
    /// Wind shear reported for all runways (`WS ALL RWY`).
    AllRunways,
}
