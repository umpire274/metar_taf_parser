//! Shared field description helpers used by both METAR and TAF description logic.

use crate::common::describe::locale::Locale;
use crate::metar::models::cloud::{CloudAmount, CloudLayer};
use crate::metar::models::pressure::Pressure;
use crate::metar::models::trend::MetarTrendDetail;
use crate::metar::models::visibility::Visibility;
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use crate::taf::models::wind_shear::TafWindShear;

/// Describes a wind group in natural language.
///
/// Produces output such as `"wind from 180° at 10 kt"` or
/// `"wind from variable direction at 5 kt, gusting 15 kt"`.
pub fn describe_wind<L: Locale>(wind: &Wind, locale: &L) -> String {
    let dir = match wind.direction {
        None => "variable direction".to_string(),
        Some(d) => format!("{}°", d),
    };
    let unit = locale.wind_unit(&wind.unit);

    match wind.gust {
        None => format!("wind from {} at {} {}", dir, wind.speed, unit),
        Some(g) => format!(
            "wind from {} at {} {}, gusting {} {}",
            dir, wind.speed, unit, g, unit
        ),
    }
}

/// Describes a visibility group in natural language.
///
/// Handles CAVOK, prevailing visibility in metres, and directional minimum visibility.
pub fn describe_visibility<L: Locale>(vis: &Visibility, locale: &L) -> String {
    match vis {
        Visibility::CAVOK => "CAVOK (ceiling and visibility OK)".to_string(),
        Visibility::Single { prevailing } => {
            if *prevailing >= 9999 {
                "visibility greater than 10 km".to_string()
            } else {
                format!("visibility {} m", prevailing)
            }
        }
        Visibility::WithMinimum {
            prevailing,
            minimum,
            direction,
        } => {
            let dir = locale.visibility_direction(direction);
            format!(
                "visibility {} m, minimum {} m to the {}",
                prevailing, minimum, dir
            )
        }
    }
}

/// Describes a cloud layer in natural language.
///
/// Handles special values (NSC, SKC, VV) as well as standard layers with
/// optional altitude and cloud type.
pub fn describe_cloud<L: Locale>(cloud: &CloudLayer, locale: &L) -> String {
    match &cloud.amount {
        CloudAmount::NSC => "no significant clouds".to_string(),
        CloudAmount::SKC => "sky clear".to_string(),
        CloudAmount::VV => match cloud.altitude_ft {
            None => "vertical visibility not available".to_string(),
            Some(f) => format!("vertical visibility {} ft", f),
        },
        amount => {
            let amount_str = locale.cloud_amount(amount);
            match cloud.altitude_ft {
                None => format!("{} (height not available)", amount_str),
                Some(f) => {
                    let base = format!("{} at {} ft", amount_str, f);
                    match &cloud.cloud_type {
                        None => base,
                        Some(t) => format!("{} ({})", base, locale.cloud_type(t)),
                    }
                }
            }
        }
    }
}

/// Describes a present weather group in natural language.
///
/// Assembles intensity, descriptors and phenomena into a single phrase,
/// e.g. `"heavy showers of rain"` or `"light freezing drizzle"`.
pub fn describe_weather<L: Locale>(weather: &Weather, locale: &L) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(intensity) = &weather.intensity {
        parts.push(locale.weather_intensity(intensity).to_string());
    }
    for desc in &weather.descriptors {
        parts.push(locale.weather_descriptor(desc).to_string());
    }
    for phen in &weather.phenomena {
        parts.push(locale.weather_phenomenon(phen));
    }

    parts.join(" ")
}

/// Describes a pressure group in natural language.
///
/// Supports both QNH in hPa and altimeter in inHg.
pub fn describe_pressure(pressure: &Pressure) -> String {
    match pressure {
        Pressure::QnhHpa(v) => format!("QNH {} hPa", v),
        Pressure::AltimeterInHg(v) => format!("altimeter {:.2} inHg", v),
    }
}

/// Describes a METAR trend detail (TEMPO, BECMG, NOSIG) in natural language.
pub fn describe_trend_detail<L: Locale>(detail: &MetarTrendDetail, locale: &L) -> String {
    let kind = locale.metar_trend(&detail.kind);
    let mut parts = vec![kind.to_string()];

    for t in &detail.times {
        let kind_str = locale.metar_trend_time_kind(&t.kind);
        parts.push(format!("{} {:02}:{:02}Z", kind_str, t.hour, t.minute));
    }

    if let Some(wind) = &detail.wind {
        parts.push(describe_wind(wind, locale));
    }
    if let Some(vis) = &detail.visibility {
        parts.push(describe_visibility(vis, locale));
    }
    for w in &detail.weather {
        parts.push(describe_weather(w, locale));
    }
    for c in &detail.clouds {
        parts.push(describe_cloud(c, locale));
    }

    parts.join(", ")
}

/// Describes a TAF wind shear group in natural language.
pub fn describe_wind_shear(ws: &TafWindShear) -> String {
    format!(
        "wind shear at {} ft: {}° at {} kt",
        ws.height_hundreds_ft * 100,
        ws.direction,
        ws.speed_kt,
    )
}
