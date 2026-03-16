//! Module `pressure`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
/// Enumerates the allowed values for Pressure.
pub enum Pressure {
    QnhHpa(u16),
    AltimeterInHg(f32),
}
