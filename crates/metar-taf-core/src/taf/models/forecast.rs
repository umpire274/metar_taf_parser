use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::wind::Wind;
use crate::taf::models::time::TafPeriod;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TafForecastKind {
    Base,
    FM,
    BECMG,
    TEMPO,
    PROB,
}

#[derive(Debug, Serialize)]
pub struct TafForecast {
    pub kind: TafForecastKind,

    // Temporal information
    pub from: Option<(u8, u8, u8)>, // FM
    pub period: Option<TafPeriod>,  // BECMG / TEMPO / PROB

    // Probability (PROB30 / PROB40)
    pub probability: Option<u8>,

    // Weather
    pub wind: Option<Wind>,
    pub visibility: Option<Visibility>,
    pub clouds: Vec<CloudLayer>,
}
