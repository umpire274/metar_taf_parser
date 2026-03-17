//! Module `metar`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::common::report_modifier::ReportModifier;
use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::pressure::Pressure;
use crate::metar::models::remark::Remarks;
use crate::metar::models::runway_state::RunwayState;
use crate::metar::models::rvr::RunwayVisualRange;
use crate::metar::models::temperature::Temperature;
use crate::metar::models::trend::{MetarTrend, MetarTrendDetail};
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Represents a parsed Metar report with typed fields.
pub struct Metar {
    pub station: String,
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
    pub trend: Option<MetarTrend>,
    pub trend_detail: Option<MetarTrendDetail>,
    pub unparsed_groups: Vec<String>,
    pub raw: String,
}

impl Metar {
    /// Creates a new [`Metar`] with normalized defaults.
    pub fn new(station: &str, raw: &str) -> Self {
        Self {
            station: station.to_string(),
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
            trend: None,
            trend_detail: None,
            unparsed_groups: Vec::new(),
            raw: raw.to_string(),
        }
    }
}
