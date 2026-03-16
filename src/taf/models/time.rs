//! Module `time`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
/// Stores normalized time-related data for TafTime.
pub struct TafTime {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
/// Stores normalized time-related data for TafValidity.
pub struct TafValidity {
    pub from_day: u8,
    pub from_hour: u8,
    pub to_day: u8,
    pub to_hour: u8,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
/// Stores normalized time-related data for TafPeriod.
pub struct TafPeriod {
    pub from: (u8, u8, u8), // day, hour, minute
    pub to: (u8, u8, u8),
}
