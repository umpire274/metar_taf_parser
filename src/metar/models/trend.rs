//! Module `trend`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Enumerates the allowed values for MetarTrend.
pub enum MetarTrend {
    Nosig,
    Becmg,
    Tempo,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Enumerates the allowed values for MetarTrendTimeKind.
pub enum MetarTrendTimeKind {
    From,
    Until,
    At,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Stores normalized time-related data for MetarTrendTime.
pub struct MetarTrendTime {
    pub kind: MetarTrendTimeKind,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Defines the MetarTrendDetail domain model used by the parser.
pub struct MetarTrendDetail {
    pub kind: MetarTrend,
    pub times: Vec<MetarTrendTime>,
    pub wind: Option<Wind>,
    pub visibility: Option<Visibility>,
    pub weather: Vec<Weather>,
    pub clouds: Vec<CloudLayer>,
    pub raw_tokens: Vec<String>,
    pub unparsed_groups: Vec<String>,
}
