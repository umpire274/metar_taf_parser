use crate::common::report_modifier::ReportModifier;
use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::pressure::Pressure;
use crate::metar::models::runway_state::RunwayState;
use crate::metar::models::temperature::Temperature;
use crate::metar::models::trend::MetarTrend;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use serde::Serialize;

#[derive(Debug, Serialize)]
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
    pub rmk: Option<String>,
    pub runway_state: Vec<RunwayState>,
    pub trend: Option<MetarTrend>,
    pub unparsed_groups: Vec<String>,
    pub raw: String,
}

impl Metar {
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
            rmk: None,
            runway_state: Vec::new(),
            trend: None,
            unparsed_groups: Vec::new(),
            raw: raw.to_string(),
        }
    }
}
