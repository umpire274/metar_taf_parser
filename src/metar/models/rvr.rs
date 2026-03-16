//! Module `rvr`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Enumerates the allowed values for RvrQualifier.
pub enum RvrQualifier {
    Above,
    Below,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Enumerates the allowed values for RvrTendency.
pub enum RvrTendency {
    Upward,
    Downward,
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Enumerates the allowed values for RvrUnit.
pub enum RvrUnit {
    Meters,
    Feet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Defines the RvrValue domain model used by the parser.
pub struct RvrValue {
    pub value: u16,
    pub qualifier: Option<RvrQualifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
/// Defines the RunwayVisualRange domain model used by the parser.
pub struct RunwayVisualRange {
    pub runway_designator: String,
    pub min: RvrValue,
    pub max: Option<RvrValue>,
    pub tendency: Option<RvrTendency>,
    pub unit: RvrUnit,
}
