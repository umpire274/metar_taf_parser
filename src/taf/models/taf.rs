//! Module `taf`.
//!
//! Contains types and parsing logic implemented for this crate.
use super::forecast::TafForecast;
use super::time::{TafTime, TafValidity};
use crate::common::report_modifier::ReportModifier;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Represents a parsed Taf report with typed fields.
pub struct Taf {
    pub station: String,
    pub issued_at: Option<TafTime>,
    pub validity: Option<TafValidity>,
    pub modifier: ReportModifier,
    pub forecasts: Vec<TafForecast>,
    pub unparsed_groups: Vec<String>,
}
