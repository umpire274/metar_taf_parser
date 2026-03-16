//! Module `trend`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// Enumerates the allowed values for MetarTrend.
pub enum MetarTrend {
    Nosig,
    Becmg,
    Tempo,
}
