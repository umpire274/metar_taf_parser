use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Airport {
    pub icao: String,
    pub name: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_ft: Option<i32>,
}
