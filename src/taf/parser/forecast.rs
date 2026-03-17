//! Module `forecast`.
//!
//! Contains types and parsing logic implemented for this crate.
use std::iter::Peekable;
use std::slice::Iter;

use crate::common::report_modifier::ReportModifier;
use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::visibility::{parse_split_statute_miles_to_meters, parse_visibility};
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;
use crate::taf::models::forecast::{TafForecast, TafForecastKind};
use crate::taf::models::icing::{Icing, IcingIntensity};
use crate::taf::models::temperature::TafTemperature;
use crate::taf::models::time::TafPeriod;
use crate::taf::models::turbulence::{Turbulence, TurbulenceIntensity};
use crate::taf::models::wind_shear::TafWindShear;

/// Entry point: parse tutti i forecast TAF
pub fn parse_forecasts(tokens: &[String]) -> (Vec<TafForecast>, Vec<String>) {
    let mut forecasts = Vec::new();
    let mut unparsed_groups = Vec::new();

    let mut current = new_base_forecast();
    let mut visibility_context: Option<Visibility> = None;

    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        // -------- FM --------
        if let Some(from) = parse_fm_time(token) {
            forecasts.push(current);
            current = new_fm_forecast(from);
            visibility_context = None;
            continue;
        }

        // -------- BECMG --------
        if token == "BECMG"
            && let Some(period) = try_consume_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::BECMG, period, None);
            visibility_context = None;
            continue;
        }

        // -------- TEMPO --------
        if token == "TEMPO"
            && let Some(period) = try_consume_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::TEMPO, period, None);
            visibility_context = None;
            continue;
        }

        // -------- PROB30 / PROB40 --------
        if let Some(prob) = parse_prob(token)
            && let Some(period) = try_consume_prob_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::PROB, period, Some(prob));
            visibility_context = None;
            continue;
        }

        // -------- Wind --------
        if let Some(wind) = parse_wind(token) {
            current.wind = Some(wind);
            continue;
        }

        // -------- Visibility --------
        if token.chars().all(|c| c.is_ascii_digit())
            && let Some(next) = iter.peek()
            && let Some(prevailing) = parse_split_statute_miles_to_meters(token, next)
        {
            let vis = Visibility::Single {
                prevailing,
                qualifier: None,
                ndv: false,
            };
            visibility_context = Some(vis.clone());
            current.visibility = Some(vis);
            iter.next();
            continue;
        }

        if let Some(vis) = parse_visibility(token, &fake_metar(visibility_context.clone())) {
            visibility_context = Some(vis.clone());
            // CAVOK replaces visibility, weather and cloud groups — clear any
            // groups already parsed in this forecast block.
            if matches!(vis, Visibility::CAVOK) {
                current.clouds.clear();
                current.weather.clear();
            }
            current.visibility = Some(vis);
            continue;
        }

        // -------- Wind shear (WS) --------
        if let Some(ws) = parse_taf_wind_shear(token) {
            current.wind_shear = Some(ws);
            continue;
        }

        // -------- Icing (6ABBBC) --------
        if let Some(icing) = parse_icing(token) {
            current.icing.push(icing);
            continue;
        }

        // -------- Turbulence (5ABBBC) --------
        if let Some(turb) = parse_turbulence(token) {
            current.turbulence.push(turb);
            continue;
        }

        // -------- TAF temperatures (TX/TN) --------
        if let Some((is_max, temp)) = parse_taf_temperature(token) {
            if is_max {
                current.max_temperature = Some(temp);
            } else {
                current.min_temperature = Some(temp);
            }
            continue;
        }

        // -------- Clouds --------
        if let Some(cloud) = parse_cloud(token) {
            current.clouds.push(cloud);
            continue;
        }

        // -------- Weather --------
        if let Some(weather) = parse_weather(token) {
            current.weather.push(weather);
            continue;
        }

        unparsed_groups.push(token.to_string());
    }

    forecasts.push(current);
    (forecasts, unparsed_groups)
}

// ===== Forecast builders =====

/// Creates a new `new_base_forecast` value with normalized defaults.
fn new_base_forecast() -> TafForecast {
    TafForecast {
        kind: TafForecastKind::Base,
        from: None,
        period: None,
        probability: None,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
        max_temperature: None,
        min_temperature: None,
        wind_shear: None,
        icing: Vec::new(),
        turbulence: Vec::new(),
    }
}

/// Creates a new `new_fm_forecast` value with normalized defaults.
fn new_fm_forecast(from: (u8, u8, u8)) -> TafForecast {
    TafForecast {
        kind: TafForecastKind::FM,
        from: Some(from),
        period: None,
        probability: None,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
        max_temperature: None,
        min_temperature: None,
        wind_shear: None,
        icing: Vec::new(),
        turbulence: Vec::new(),
    }
}

/// Creates a new `new_period_forecast` value with normalized defaults.
fn new_period_forecast(
    kind: TafForecastKind,
    period: TafPeriod,
    probability: Option<u8>,
) -> TafForecast {
    TafForecast {
        kind,
        from: None,
        period: Some(period),
        probability,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
        max_temperature: None,
        min_temperature: None,
        wind_shear: None,
        icing: Vec::new(),
        turbulence: Vec::new(),
    }
}

// ===== Helpers =====

/// Parses input tokens into typed data for `parse_fm_time`.
fn parse_fm_time(token: &str) -> Option<(u8, u8, u8)> {
    if !token.starts_with("FM") || token.len() != 8 {
        return None;
    }

    parse_day_hour_min(&token[2..])
}

/// Parses input tokens into typed data for `parse_becmg_period`.
fn parse_becmg_period(token: &str) -> Option<TafPeriod> {
    let (from, to) = token.split_once('/')?;

    let from = parse_day_hour(from)?;
    let to = parse_day_hour(to)?;

    Some(TafPeriod {
        from: (from.0, from.1, 0),
        to: (to.0, to.1, 0),
    })
}

/// Parses input tokens into typed data for `parse_prob`.
fn parse_prob(token: &str) -> Option<u8> {
    match token {
        "PROB30" => Some(30),
        "PROB40" => Some(40),
        _ => None,
    }
}

/// Helper function used by `try_consume_prob_period` parsing logic.
fn try_consume_prob_period(iter: &mut Peekable<Iter<'_, String>>) -> Option<TafPeriod> {
    let mut lookahead = iter.clone();

    if let Some(token) = lookahead.next() {
        if token == "TEMPO" {
            let period_token = lookahead.next()?;
            let period = parse_becmg_period(period_token)?;
            iter.next(); // TEMPO
            iter.next(); // period
            return Some(period);
        }

        let period = parse_becmg_period(token)?;
        iter.next(); // period
        return Some(period);
    }

    None
}

/// Helper function used by `try_consume_period` parsing logic.
fn try_consume_period(iter: &mut Peekable<Iter<'_, String>>) -> Option<TafPeriod> {
    let mut lookahead = iter.clone();
    let period_token = lookahead.next()?;
    let period = parse_becmg_period(period_token)?;
    iter.next(); // period
    Some(period)
}

/// Parses input tokens into typed data for `parse_day_hour`.
fn parse_day_hour(value: &str) -> Option<(u8, u8)> {
    if value.len() != 4 || !value.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = value[0..2].parse().ok()?;
    let hour: u8 = value[2..4].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 24 {
        return None;
    }

    Some((day, hour))
}

/// Parses input tokens into typed data for `parse_day_hour_min`.
fn parse_day_hour_min(value: &str) -> Option<(u8, u8, u8)> {
    if value.len() != 6 || !value.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = value[0..2].parse().ok()?;
    let hour: u8 = value[2..4].parse().ok()?;
    let min: u8 = value[4..6].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 23 || min > 59 {
        return None;
    }

    Some((day, hour, min))
}

/// Parses input tokens into typed data for `parse_taf_wind_shear`.
fn parse_taf_wind_shear(token: &str) -> Option<TafWindShear> {
    let body = token.strip_prefix("WS")?;
    let (height_part, wind_part) = body.split_once('/')?;

    if height_part.len() != 3 || !height_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let wind = wind_part.strip_suffix("KT")?;
    if wind.len() != 5 || !wind.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let height_hundreds_ft: u16 = height_part.parse().ok()?;
    let direction: u16 = wind[0..3].parse().ok()?;
    let speed_kt: u16 = wind[3..5].parse().ok()?;

    if direction > 360 || speed_kt == 0 {
        return None;
    }

    Some(TafWindShear {
        height_hundreds_ft,
        direction,
        speed_kt,
    })
}

/// Parses input tokens into typed data for `parse_taf_temperature`.
fn parse_taf_temperature(token: &str) -> Option<(bool, TafTemperature)> {
    let (is_max, body) = if let Some(v) = token.strip_prefix("TX") {
        (true, v)
    } else if let Some(v) = token.strip_prefix("TN") {
        (false, v)
    } else {
        return None;
    };

    if !body.ends_with('Z') {
        return None;
    }

    let core = &body[..body.len() - 1];
    let (temp_part, when_part) = core.split_once('/')?;

    let value = parse_signed_temp(temp_part)?;

    if when_part.len() != 4 || !when_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = when_part[0..2].parse().ok()?;
    let hour: u8 = when_part[2..4].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 24 {
        return None;
    }

    Some((is_max, TafTemperature { value, day, hour }))
}

/// Parses input tokens into typed data for `parse_signed_temp`.
fn parse_signed_temp(token: &str) -> Option<i8> {
    if token.len() != 2 && token.len() != 3 {
        return None;
    }

    let (negative, digits) = if let Some(v) = token.strip_prefix('M') {
        (true, v)
    } else {
        (false, token)
    };

    if digits.len() != 2 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let parsed: i8 = digits.parse().ok()?;
    Some(if negative { -parsed } else { parsed })
}

/// Parses a TAF icing group of the form `6ABBBC` into an [`Icing`] value.
fn parse_icing(token: &str) -> Option<Icing> {
    if token.len() != 6 {
        return None;
    }
    let bytes = token.as_bytes();
    if bytes[0] != b'6' {
        return None;
    }
    if !token[1..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let intensity = IcingIntensity::from_code(bytes[1] - b'0');
    let base_hundreds: u16 = token[2..5].parse().ok()?;
    let thickness_thousands: u16 = (bytes[5] - b'0') as u16;
    Some(Icing {
        intensity,
        base_ft: base_hundreds * 100,
        thickness_ft: thickness_thousands * 1000,
    })
}

/// Parses a TAF turbulence group of the form `5ABBBC` into a [`Turbulence`] value.
fn parse_turbulence(token: &str) -> Option<Turbulence> {
    if token.len() != 6 {
        return None;
    }
    let bytes = token.as_bytes();
    if bytes[0] != b'5' {
        return None;
    }
    if !token[1..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let intensity = TurbulenceIntensity::from_code(bytes[1] - b'0');
    let base_hundreds: u16 = token[2..5].parse().ok()?;
    let thickness_thousands: u16 = (bytes[5] - b'0') as u16;
    Some(Turbulence {
        intensity,
        base_ft: base_hundreds * 100,
        thickness_ft: thickness_thousands * 1000,
    })
}

/// Builds a minimal [`Metar`] used as context for visibility parsing inside TAF forecasts.
fn fake_metar(visibility: Option<Visibility>) -> crate::metar::models::Metar {
    use crate::metar::models::Metar;

    Metar {
        station: String::new(),
        report_type: crate::metar::models::report_type::MetarReportType::default(),
        time: None,
        modifier: ReportModifier::Normal,
        wind: None,
        visibility,
        clouds: Vec::new(),
        temperature: None,
        pressure: None,
        weather: Vec::new(),
        remarks: crate::metar::models::remark::Remarks::default(),
        nosig: false,
        runway_state: Vec::new(),
        runway_visual_range: Vec::new(),
        trend: None,
        trend_detail: None,
        color_code: None,
        color_code_forecast: None,
        sea_state: None,
        wind_shear: Vec::new(),
        unparsed_groups: Vec::new(),
        raw: String::new(),
    }
}
