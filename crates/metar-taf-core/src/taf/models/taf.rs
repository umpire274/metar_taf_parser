use super::forecast::TafForecast;
use super::time::{TafTime, TafValidity};

#[derive(Debug)]
pub struct Taf {
    pub station: String,
    pub issued_at: TafTime,
    pub validity: TafValidity,
    pub forecasts: Vec<TafForecast>,
}
