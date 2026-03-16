//! Module `model`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
/// Defines the Airport domain model used by the parser.
pub struct Airport {
    pub icao: String,
    pub name: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_ft: Option<i32>,
}
