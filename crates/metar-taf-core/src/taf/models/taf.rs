use super::forecast::TafForecast;
use super::time::{TafTime, TafValidity};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Taf {
    pub station: String,
    pub issued_at: TafTime,
    pub validity: TafValidity,
    pub forecasts: Vec<TafForecast>,
}
