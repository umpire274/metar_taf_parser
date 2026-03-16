//! Module `trend`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::trend::MetarTrend;

/// Parses input tokens into typed data for `parse_trend`.
pub fn parse_trend(token: &str) -> Option<MetarTrend> {
    match token {
        "NOSIG" => Some(MetarTrend::Nosig),
        "BECMG" => Some(MetarTrend::Becmg),
        "TEMPO" => Some(MetarTrend::Tempo),
        _ => None,
    }
}
