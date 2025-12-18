use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::pressure::Pressure;
use crate::metar::models::temperature::Temperature;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;

#[derive(Debug)]
pub struct Metar {
    pub station: String,
    pub time: Option<super::time::MetarTime>,
    pub wind: Option<Wind>,
    pub visibility: Option<Visibility>,
    pub clouds: Vec<CloudLayer>,
    pub temperature: Option<Temperature>,
    pub pressure: Option<Pressure>,
    pub weather: Vec<Weather>,
    pub raw: String,
}

impl Metar {
    pub fn new(station: &str, raw: &str) -> Self {
        Self {
            station: station.to_string(),
            time: None,
            wind: None,
            visibility: None,
            clouds: Vec::new(),
            temperature: None,
            pressure: None,
            weather: Vec::new(),
            raw: raw.to_string(),
        }
    }
}
