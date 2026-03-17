//! Module `mod`.
//!
//! Contains types and parsing logic implemented for this crate.
pub mod cloud;
/// Exposes the `color_code` module.
pub mod color_code;
/// Exposes the `metar` module.
pub mod metar;
/// Exposes the `pressure` module.
pub mod pressure;
/// Exposes the `remark` module.
pub mod remark;
/// Exposes the `report_type` module.
pub mod report_type;
pub(crate) mod runway_state;
/// Exposes the `rvr` module.
pub mod rvr;
/// Exposes the `sea_state` module.
pub mod sea_state;
/// Exposes the `temperature` module.
pub mod temperature;
/// Exposes the `time` module.
pub mod time;
/// Exposes the `trend` module.
pub mod trend;
/// Exposes the `visibility` module.
pub mod visibility;
/// Exposes the `weather` module.
pub mod weather;
/// Exposes the `wind` module.
pub mod wind;
/// Exposes the `wind_shear` module.
pub mod wind_shear;

pub use metar::Metar;
