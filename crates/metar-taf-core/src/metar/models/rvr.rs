use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RvrTrend {
    Up,
    Down,
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rvr {
    /// Runway designator (e.g. "01", "25L")
    pub runway: String,

    /// Minimum visibility in meters
    pub min: u16,

    /// Maximum visibility in meters (if variable)
    pub max: Option<u16>,

    /// Trend (U/D/N)
    pub trend: Option<RvrTrend>,
}
