//! Module `wind`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::wind::{Wind, WindUnit, WindVariation};

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
            variation: None,
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
        variation: None,
    })
}

/// Parses a variable wind direction range token of the form `dddVddd`.
///
/// Returns `Some(WindVariation)` for tokens like `180V240`, otherwise `None`.
///
/// # Examples
///
/// ```
/// use metar_taf_parser::metar::parser::wind::parse_wind_variation;
///
/// let v = parse_wind_variation("180V240").unwrap();
/// assert_eq!(v.min, 180);
/// assert_eq!(v.max, 240);
///
/// assert!(parse_wind_variation("18010KT").is_none());
/// assert!(parse_wind_variation("VRB05KT").is_none());
/// ```
pub fn parse_wind_variation(token: &str) -> Option<WindVariation> {
    // Exactly 7 chars: 3 digits + 'V' + 3 digits
    if token.len() != 7 {
        return None;
    }
    let (min_str, max_str) = token.split_once('V')?;
    if min_str.len() != 3 || max_str.len() != 3 {
        return None;
    }
    if !min_str.chars().all(|c| c.is_ascii_digit())
        || !max_str.chars().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let min: u16 = min_str.parse().ok()?;
    let max: u16 = max_str.parse().ok()?;
    if min > 360 || max > 360 {
        return None;
    }
    Some(WindVariation { min, max })
}
