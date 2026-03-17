//! Module `weather`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::weather::*;

/// Parses a weather group token into a [`Weather`] value.
///
/// Handles intensity prefixes (`+`, `-`, `RE`), descriptor codes
/// (e.g. `SH`, `TS`, `FZ`) and phenomenon codes (e.g. `RA`, `SN`, `FG`).
/// Multiple phenomena may appear in a single token (e.g. `RASN`).
/// Returns `None` for empty or malformed tokens.
pub fn parse_weather(token: &str) -> Option<Weather> {
    if token.is_empty() {
        return None;
    }

    if token == "NSW" {
        return Some(Weather {
            intensity: None,
            descriptors: Vec::new(),
            phenomena: vec![WeatherPhenomenon::NoSignificantWeather],
        });
    }

    let mut s = token;

    // ---- intensity ----
    let intensity = if let Some(rest) = s.strip_prefix('+') {
        s = rest;
        Some(WeatherIntensity::Heavy)
    } else if let Some(rest) = s.strip_prefix('-') {
        s = rest;
        Some(WeatherIntensity::Light)
    } else if let Some(rest) = s.strip_prefix("RE") {
        s = rest;
        Some(WeatherIntensity::Recent)
    } else {
        None
    };

    let mut descriptors = Vec::new();
    let mut phenomena = Vec::new();

    // ---- descriptors (2-char codes) ----
    while s.len() >= 2 {
        let code = &s[0..2];
        let desc = match code {
            "MI" => WeatherDescriptor::Shallow,
            "PR" => WeatherDescriptor::Partial,
            "BC" => WeatherDescriptor::Patches,
            "DR" => WeatherDescriptor::LowDrifting,
            "BL" => WeatherDescriptor::Blowing,
            "SH" => WeatherDescriptor::Showers,
            "TS" => WeatherDescriptor::Thunderstorm,
            "FZ" => WeatherDescriptor::Freezing,
            "VC" => WeatherDescriptor::Vicinity,
            _ => break,
        };
        descriptors.push(desc);
        s = &s[2..];
    }

    // ---- phenomena (2-char codes, repeatable) ----
    while s.len() >= 2 {
        let code = &s[0..2];
        let phen = match code {
            "RA" => WeatherPhenomenon::Rain,
            "SN" => WeatherPhenomenon::Snow,
            "DZ" => WeatherPhenomenon::Drizzle,
            "FG" => WeatherPhenomenon::Fog,
            "BR" => WeatherPhenomenon::Mist,
            "HZ" => WeatherPhenomenon::Haze,
            "FU" => WeatherPhenomenon::Smoke,
            "GR" => WeatherPhenomenon::Hail,
            "GS" => WeatherPhenomenon::SmallHail,
            "PL" | "PE" => WeatherPhenomenon::IcePellets,  // PE is legacy ICAO code
            "IC" => WeatherPhenomenon::IceCrystals,
            "SG" => WeatherPhenomenon::SnowGrains,
            "PO" => WeatherPhenomenon::SandWhirls,
            "SQ" => WeatherPhenomenon::Squalls,
            "FC" => WeatherPhenomenon::FunnelCloud,
            "SA" => WeatherPhenomenon::Sand,
            "DU" => WeatherPhenomenon::Dust,
            "DS" => WeatherPhenomenon::DustStorm,
            "SS" => WeatherPhenomenon::SandStorm,
            "PY" => WeatherPhenomenon::Spray,
            "VA" => WeatherPhenomenon::VolcanicAsh,
            "UP" => WeatherPhenomenon::UnknownPrecipitation,
            "TS" => WeatherPhenomenon::Thunder,
            _ => WeatherPhenomenon::Unknown(code.to_string()),
        };
        phenomena.push(phen);
        s = &s[2..];
    }

    // malformed trailing fragment (odd length)
    if !s.is_empty() {
        return None;
    }

    // TS captured as descriptor but no phenomena follow → standalone thunderstorm
    if phenomena.is_empty() {
        if let Some(pos) = descriptors
            .iter()
            .position(|d| matches!(d, WeatherDescriptor::Thunderstorm))
        {
            descriptors.remove(pos);
            phenomena.push(WeatherPhenomenon::Thunder);
        }
    }

    if descriptors.is_empty() && phenomena.is_empty() {
        None
    } else {
        Some(Weather { intensity, descriptors, phenomena })
    }
}
