//! Module `metar`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::common::report_modifier::ReportModifier;
use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::color_code::MilitaryColor;
use crate::metar::models::pressure::Pressure;
use crate::metar::models::remark::Remarks;
use crate::metar::models::report_type::MetarReportType;
use crate::metar::models::runway_state::RunwayState;
use crate::metar::models::rvr::RunwayVisualRange;
use crate::metar::models::sea_state::SeaState;
use crate::metar::models::temperature::Temperature;
use crate::metar::models::trend::{MetarTrend, MetarTrendDetail};
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use crate::metar::models::wind_shear::MetarWindShearRunway;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Represents a parsed Metar report with typed fields.
pub struct Metar {
    pub station: String,
    /// Whether this is a routine METAR or a special SPECI observation.
    pub report_type: MetarReportType,
    pub time: Option<super::time::MetarTime>,
    pub modifier: ReportModifier,
    pub wind: Option<Wind>,
    pub visibility: Option<Visibility>,
    pub clouds: Vec<CloudLayer>,
    pub temperature: Option<Temperature>,
    pub pressure: Option<Pressure>,
    pub weather: Vec<Weather>,
    /// Structured parsed RMK section. Access raw text via `remarks.raw`.
    pub remarks: Remarks,
    /// Whether a NOSIG trend is present.
    pub nosig: bool,
    pub runway_state: Vec<RunwayState>,
    pub runway_visual_range: Vec<RunwayVisualRange>,
    /// Sea state reported by an offshore station, if present.
    pub sea_state: Option<SeaState>,
    /// Wind shear runway groups (`WS R23`, `WS ALL RWY`), if present.
    pub wind_shear: Vec<MetarWindShearRunway>,
    pub trend: Option<MetarTrend>,
    pub trend_detail: Option<MetarTrendDetail>,
    /// Current military flight-condition color code, if present.
    pub color_code: Option<MilitaryColor>,
    /// Implicit BECMG forecast color code (a second bare color code token
    /// appearing after `color_code` but without a `BECMG` keyword).
    pub color_code_forecast: Option<MilitaryColor>,
    pub unparsed_groups: Vec<String>,
    pub raw: String,
}

impl Metar {
    /// Creates a new [`Metar`] with normalized defaults.
    pub fn new(station: &str, raw: &str) -> Self {
        Self {
            station: station.to_string(),
            report_type: MetarReportType::default(),
            time: None,
            modifier: ReportModifier::Normal,
            wind: None,
            visibility: None,
            clouds: Vec::new(),
            temperature: None,
            pressure: None,
            weather: Vec::new(),
            remarks: Remarks::default(),
            nosig: false,
            runway_state: Vec::new(),
            runway_visual_range: Vec::new(),
            sea_state: None,
            wind_shear: Vec::new(),
            trend: None,
            trend_detail: None,
            color_code: None,
            color_code_forecast: None,
            unparsed_groups: Vec::new(),
            raw: raw.to_string(),
        }
    }
}
