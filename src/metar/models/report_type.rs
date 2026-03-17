//! Report type indicator for METAR/SPECI messages.
use serde::Serialize;

/// Distinguishes between a routine METAR and a special SPECI observation.
///
/// The `SPECI` observation is issued outside the regular schedule when
/// significant meteorological changes occur (e.g. sudden visibility drop,
/// wind shift, or ceiling change). Most modern stations issue observations
/// every 30 minutes, making SPECI less common than in the past.
///
/// When neither `METAR` nor `SPECI` appears as a leading token, the report
/// is treated as a routine [`MetarReportType::Metar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum MetarReportType {
    /// Routine scheduled METAR observation.
    #[default]
    Metar,
    /// Special (non-routine) SPECI observation.
    Speci,
}

