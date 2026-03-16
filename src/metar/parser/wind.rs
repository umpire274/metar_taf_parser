//! Module `wind`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::wind::{Wind, WindUnit};

/// Normalizes token/text input for `strip_unit` processing.
fn strip_unit(token: &str) -> Option<(&str, WindUnit)> {
    if let Some(rest) = token.strip_suffix("KT") {
        Some((rest, WindUnit::KT))
    } else if let Some(rest) = token.strip_suffix("MPS") {
        Some((rest, WindUnit::MPS))
    } else {
        None
    }
}

/// Parses input tokens into typed data for `parse_wind`.
pub fn parse_wind(token: &str) -> Option<Wind> {
    let (body, unit) = strip_unit(token)?;

    // Calm wind
    if body == "00000" {
        return Some(Wind {
            direction: Some(0),
            speed: 0,
            gust: None,
            unit,
        });
    }

    let (direction, rest) = if let Some(rest) = body.strip_prefix("VRB") {
        (None, rest)
    } else {
        let dir_str = body.get(0..3)?;
        if !dir_str.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        let dir: u16 = dir_str.parse().ok()?;
        if dir > 360 {
            return None;
        }

        (Some(dir), body.get(3..)?)
    };

    if rest.is_empty() {
        return None;
    }

    let (speed, gust) = if let Some((speed_str, gust_str)) = rest.split_once('G') {
        if speed_str.is_empty() || gust_str.is_empty() {
            return None;
        }

        if !speed_str.chars().all(|c| c.is_ascii_digit())
            || !gust_str.chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }

        (speed_str.parse().ok()?, Some(gust_str.parse().ok()?))
    } else {
        if !rest.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        (rest.parse().ok()?, None)
    };

    Some(Wind {
        direction,
        speed,
        gust,
        unit,
    })
}
