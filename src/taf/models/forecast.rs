//! Module `forecast`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::cloud::CloudLayer;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use crate::taf::models::temperature::TafTemperature;
use crate::taf::models::time::TafPeriod;
use crate::taf::models::wind_shear::TafWindShear;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
/// Enumerates the allowed values for TafForecastKind.
pub enum TafForecastKind {
    Base,
    FM,
    BECMG,
    TEMPO,
    PROB,
}

#[derive(Debug, Serialize)]
/// Represents a parsed forecast block in a TAF message.
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
    pub weather: Vec<Weather>,
    pub clouds: Vec<CloudLayer>,
    pub max_temperature: Option<TafTemperature>,
    pub min_temperature: Option<TafTemperature>,
    pub wind_shear: Option<TafWindShear>,
}
