//! Locale abstraction for natural language METAR/TAF descriptions.

pub mod en;

use crate::common::report_modifier::ReportModifier;
use crate::metar::models::cloud::{CloudAmount, CloudType};
use crate::metar::models::trend::{MetarTrend, MetarTrendTimeKind};
use crate::metar::models::visibility::VisibilityDirection;
use crate::metar::models::weather::{WeatherDescriptor, WeatherIntensity, WeatherPhenomenon};
use crate::metar::models::wind::WindUnit;
use crate::taf::models::forecast::TafForecastKind;

/// Provides locale-specific string mappings used to build natural language descriptions.
///
/// Implement this trait to add support for a new language. Each method maps a typed
/// domain value to its human-readable representation in the target locale.
pub trait Locale {
    /// Returns the natural language label for a cloud amount.
    fn cloud_amount(&self, amount: &CloudAmount) -> &'static str;

    /// Returns the natural language label for a cloud type.
    fn cloud_type(&self, t: &CloudType) -> &'static str;

    /// Returns the abbreviation or label for a wind speed unit.
    fn wind_unit(&self, unit: &WindUnit) -> &'static str;

    /// Returns the natural language label for a weather intensity.
    fn weather_intensity(&self, i: &WeatherIntensity) -> &'static str;

    /// Returns the natural language label for a weather descriptor.
    fn weather_descriptor(&self, d: &WeatherDescriptor) -> &'static str;

    /// Returns the natural language label for a weather phenomenon.
    ///
    /// Returns an owned `String` to accommodate variants that carry dynamic data
    /// (e.g. `WeatherPhenomenon::Unknown`).
    fn weather_phenomenon(&self, p: &WeatherPhenomenon) -> String;

    /// Returns the natural language label for a visibility direction.
    fn visibility_direction(&self, d: &VisibilityDirection) -> &'static str;

    /// Returns a human-readable label for a report modifier, or `None` for `Normal`.
    fn report_modifier(&self, m: &ReportModifier) -> Option<&'static str>;

    /// Returns the natural language label for a TAF forecast block kind.
    fn forecast_kind(&self, k: &TafForecastKind) -> &'static str;

    /// Returns the natural language label for a METAR trend type.
    fn metar_trend(&self, t: &MetarTrend) -> &'static str;

    /// Returns the natural language label for a METAR trend time indicator.
    fn metar_trend_time_kind(&self, k: &MetarTrendTimeKind) -> &'static str;
}

