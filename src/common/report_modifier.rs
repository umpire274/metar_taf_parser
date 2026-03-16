//! Module `report_modifier`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize)]
/// Enumerates the allowed values for ReportModifier.
pub enum ReportModifier {
    /// Standard manual report
    #[default]
    Normal,

    /// Automatically generated report (AUTO)
    Auto,

    /// Corrected report (COR)
    Correction,

    /// Amended forecast (AMD) – TAF only
    Amendment,

    /// Nil report (NIL)
    Nil,
}
