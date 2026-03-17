//! Natural language description of parsed METAR and TAF reports.
//!
//! This module provides [`describe_metar`] and [`describe_taf`], which convert
//! strongly-typed parsed structs into human-readable descriptions.
//!
//! The output language is selected via the [`Language`] enum. Only English
//! is supported in the current version; additional locales can be added by
//! implementing the [`locale::Locale`] trait.
//!
//! # Example
//!
//! ```rust
//! use metar_taf_parser::{parse_metar, describe_metar, Language};
//!
//! let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
//! let desc = describe_metar(&metar, Language::En);
//!
//! assert_eq!(desc.station, "LIRF");
//! assert!(desc.wind.unwrap().contains("180°"));
//! assert!(desc.visibility.unwrap().contains("greater than 10 km"));
//! assert!(!desc.clouds.is_empty());
//! ```

pub mod locale;
mod fields;

use crate::common::describe::fields::{
    describe_cloud, describe_pressure, describe_trend_detail, describe_visibility,
    describe_weather, describe_wind, describe_wind_shear,
};
use crate::common::describe::locale::en::En;
use crate::common::describe::locale::Locale;
use crate::common::report_modifier::ReportModifier;
use crate::metar::models::metar::Metar;
use crate::taf::models::forecast::TafForecast;
use crate::taf::models::taf::Taf;
use crate::taf::models::time::TafPeriod;

/// Supported output languages for natural language METAR/TAF descriptions.
///
/// Additional variants can be added as new locales are implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// English (default).
    #[default]
    En,
}

/// Natural language description of a parsed [`Metar`] report.
///
/// Each field is `None` when the corresponding group is absent from the original message.
#[derive(Debug, Default)]
pub struct MetarDescription {
    /// ICAO station identifier.
    pub station: String,
    /// Observation time, e.g. `"Day 12 at 12:50Z"`.
    pub time: Option<String>,
    /// Report modifier, e.g. `"automated report"` or `"corrected report"`.
    pub modifier: Option<String>,
    /// Wind conditions, e.g. `"wind from 180° at 10 kt"`.
    pub wind: Option<String>,
    /// Visibility conditions, e.g. `"visibility greater than 10 km"`.
    pub visibility: Option<String>,
    /// Present weather phenomena, one entry per weather group.
    pub weather: Vec<String>,
    /// Cloud layers, one entry per parsed cloud group.
    pub clouds: Vec<String>,
    /// Temperature and dew point, e.g. `"temperature 18°C, dew point 12°C"`.
    pub temperature: Option<String>,
    /// Pressure setting, e.g. `"QNH 1015 hPa"`.
    pub pressure: Option<String>,
    /// METAR trend (TEMPO / BECMG / NOSIG), if present.
    pub trend: Option<String>,
    /// Raw RMK section text, if present.
    pub remarks: Option<String>,
}

/// Natural language description of a single TAF forecast block.
#[derive(Debug, Default)]
pub struct ForecastDescription {
    /// Forecast block type, e.g. `"Base forecast"`, `"Temporary"`.
    pub kind: String,
    /// Validity period, e.g. `"12/12Z to 13/18Z"`.
    pub period: Option<String>,
    /// Probability, e.g. `"30%"` for PROB30.
    pub probability: Option<String>,
    /// Wind conditions.
    pub wind: Option<String>,
    /// Visibility conditions.
    pub visibility: Option<String>,
    /// Present weather phenomena.
    pub weather: Vec<String>,
    /// Cloud layers.
    pub clouds: Vec<String>,
    /// Maximum temperature, e.g. `"maximum temperature 18°C on day 12 at 14:00Z"`.
    pub max_temperature: Option<String>,
    /// Minimum temperature, e.g. `"minimum temperature 8°C on day 13 at 04:00Z"`.
    pub min_temperature: Option<String>,
    /// Wind shear, e.g. `"wind shear at 2000 ft: 250° at 40 kt"`.
    pub wind_shear: Option<String>,
}

/// Natural language description of a parsed [`Taf`] report.
#[derive(Debug, Default)]
pub struct TafDescription {
    /// ICAO station identifier.
    pub station: String,
    /// Issuance time, e.g. `"Day 12 at 11:00Z"`.
    pub issued_at: Option<String>,
    /// Validity period, e.g. `"12/12Z to 13/18Z"`.
    pub validity: Option<String>,
    /// Report modifier, e.g. `"amended forecast"`.
    pub modifier: Option<String>,
    /// Forecast blocks in order of appearance.
    pub forecasts: Vec<ForecastDescription>,
}

/// Returns a natural language description of a parsed [`Metar`] report.
///
/// # Arguments
///
/// * `metar` - Reference to the parsed METAR.
/// * `lang`  - The desired output language.
///
/// # Example
///
/// ```rust
/// use metar_taf_parser::{parse_metar, describe_metar, Language};
///
/// let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
/// let desc = describe_metar(&metar, Language::En);
/// assert!(desc.wind.is_some());
/// ```
pub fn describe_metar(metar: &Metar, lang: Language) -> MetarDescription {
    match lang {
        Language::En => describe_metar_with_locale(metar, &En),
    }
}

/// Returns a natural language description of a parsed [`Taf`] report.
///
/// # Arguments
///
/// * `taf`  - Reference to the parsed TAF.
/// * `lang` - The desired output language.
///
/// # Example
///
/// ```rust
/// use metar_taf_parser::{parse_taf, describe_taf, Language};
///
/// let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
/// let desc = describe_taf(&taf, Language::En);
/// assert!(!desc.forecasts.is_empty());
/// ```
pub fn describe_taf(taf: &Taf, lang: Language) -> TafDescription {
    match lang {
        Language::En => describe_taf_with_locale(taf, &En),
    }
}

// ---------------------------------------------------------------------------
// Internal implementation
// ---------------------------------------------------------------------------

fn describe_metar_with_locale<L: Locale>(metar: &Metar, locale: &L) -> MetarDescription {
    let modifier = match metar.modifier {
        ReportModifier::Normal => None,
        ref m => locale.report_modifier(m).map(|s| s.to_string()),
    };

    MetarDescription {
        station:     metar.station.clone(),
        time:        metar.time.as_ref().map(|t| {
            format!("Day {} at {:02}:{:02}Z", t.day, t.hour, t.minute)
        }),
        modifier,
        wind:        metar.wind.as_ref().map(|w| describe_wind(w, locale)),
        visibility:  metar.visibility.as_ref().map(|v| describe_visibility(v, locale)),
        weather:     metar.weather.iter().map(|w| describe_weather(w, locale)).collect(),
        clouds:      metar.clouds.iter().map(|c| describe_cloud(c, locale)).collect(),
        temperature: metar.temperature.as_ref().map(|t| {
            format!("temperature {}°C, dew point {}°C", t.temperature, t.dew_point)
        }),
        pressure:    metar.pressure.as_ref().map(describe_pressure),
        // For NOSIG, only `metar.trend` is set (no trend_detail); handle both cases.
        trend:       metar.trend_detail.as_ref()
            .map(|td| describe_trend_detail(td, locale))
            .or_else(|| metar.trend.as_ref().map(|t| locale.metar_trend(t).to_string())),
        remarks:     metar.rmk.clone(),
    }
}

fn describe_taf_with_locale<L: Locale>(taf: &Taf, locale: &L) -> TafDescription {
    let modifier = match taf.modifier {
        ReportModifier::Normal => None,
        ref m => locale.report_modifier(m).map(|s| s.to_string()),
    };

    TafDescription {
        station:   taf.station.clone(),
        issued_at: taf.issued_at.as_ref().map(|t| {
            format!("Day {} at {:02}:{:02}Z", t.day, t.hour, t.minute)
        }),
        validity:  taf.validity.as_ref().map(|v| {
            format!("{}/{:02}Z to {}/{:02}Z", v.from_day, v.from_hour, v.to_day, v.to_hour)
        }),
        modifier,
        forecasts: taf.forecasts.iter().map(|f| describe_forecast(f, locale)).collect(),
    }
}

fn describe_forecast<L: Locale>(forecast: &TafForecast, locale: &L) -> ForecastDescription {
    let period = forecast
        .period
        .map(|p| describe_taf_period(&p))
        .or_else(|| {
            forecast.from.map(|(day, hour, min)| {
                format!("from day {} at {:02}:{:02}Z", day, hour, min)
            })
        });

    ForecastDescription {
        kind:            locale.forecast_kind(&forecast.kind).to_string(),
        period,
        probability:     forecast.probability.map(|p| format!("{}%", p)),
        wind:            forecast.wind.as_ref().map(|w| describe_wind(w, locale)),
        visibility:      forecast.visibility.as_ref().map(|v| describe_visibility(v, locale)),
        weather:         forecast.weather.iter().map(|w| describe_weather(w, locale)).collect(),
        clouds:          forecast.clouds.iter().map(|c| describe_cloud(c, locale)).collect(),
        max_temperature: forecast.max_temperature.as_ref().map(|t| {
            format!("maximum temperature {}°C on day {} at {:02}:00Z", t.value, t.day, t.hour)
        }),
        min_temperature: forecast.min_temperature.as_ref().map(|t| {
            format!("minimum temperature {}°C on day {} at {:02}:00Z", t.value, t.day, t.hour)
        }),
        wind_shear:      forecast.wind_shear.as_ref().map(describe_wind_shear),
    }
}

fn describe_taf_period(p: &TafPeriod) -> String {
    let (fd, fh, fm) = p.from;
    let (td, th, tm) = p.to;
    if fm == 0 && tm == 0 {
        format!("{}/{:02}Z to {}/{:02}Z", fd, fh, td, th)
    } else {
        format!("{}/{:02}:{:02}Z to {}/{:02}:{:02}Z", fd, fh, fm, td, th, tm)
    }
}

