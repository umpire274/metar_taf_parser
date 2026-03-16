use crate::metar::models::Metar;
use crate::metar::models::visibility::{Visibility, VisibilityDirection};

const METERS_PER_STATUTE_MILE: f64 = 1609.344;

pub fn parse_visibility(token: &str, metar: &Metar) -> Option<Visibility> {
    // CAVOK
    if token == "CAVOK" {
        return Some(Visibility::CAVOK);
    }

    if let Some(prevailing) = parse_statute_miles_to_meters(token) {
        return Some(Visibility::Single { prevailing });
    }

    // Prevalente (5000)
    if token.len() == 4 && token.chars().all(|c| c.is_ascii_digit()) {
        let prevailing: u16 = token.parse().ok()?;
        return Some(Visibility::Single { prevailing });
    }

    // Minima direzionale (2000SW)
    if token.len() >= 5 {
        let (dist_part, dir_part) = token.split_at(token.len() - 2);

        let minimum: u16 = dist_part.parse().ok()?;
        let direction = parse_visibility_direction(dir_part)?;

        // Deve esistere una visibilità prevalente precedente
        if let Some(Visibility::Single { prevailing }) = metar.visibility {
            return Some(Visibility::WithMinimum {
                prevailing,
                minimum,
                direction,
            });
        }
    }

    None
}

pub fn parse_split_statute_miles_to_meters(
    whole_miles: &str,
    fraction_with_sm: &str,
) -> Option<u16> {
    if !whole_miles.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    if !fraction_with_sm.ends_with("SM") {
        return None;
    }

    let fraction_part = &fraction_with_sm[..fraction_with_sm.len().saturating_sub(2)];
    let whole: f64 = whole_miles.parse().ok()?;
    let fraction = parse_fraction(fraction_part)?;

    miles_to_meters(whole + fraction)
}

fn parse_statute_miles_to_meters(token: &str) -> Option<u16> {
    let cleaned = token.strip_suffix("SM")?;
    let numeric = cleaned
        .strip_prefix('P')
        .or_else(|| cleaned.strip_prefix('M'))
        .unwrap_or(cleaned);

    if let Ok(whole) = numeric.parse::<f64>() {
        return miles_to_meters(whole);
    }

    let fraction = parse_fraction(numeric)?;
    miles_to_meters(fraction)
}

fn parse_fraction(value: &str) -> Option<f64> {
    let (num, den) = value.split_once('/')?;
    let numerator: f64 = num.parse().ok()?;
    let denominator: f64 = den.parse().ok()?;

    if denominator == 0.0 {
        return None;
    }

    Some(numerator / denominator)
}

fn miles_to_meters(miles: f64) -> Option<u16> {
    if miles.is_sign_negative() {
        return None;
    }

    let meters = (miles * METERS_PER_STATUTE_MILE).round();

    if meters > u16::MAX as f64 {
        return None;
    }

    Some(meters as u16)
}

fn parse_visibility_direction(s: &str) -> Option<VisibilityDirection> {
    match s {
        "N" => Some(VisibilityDirection::N),
        "NE" => Some(VisibilityDirection::NE),
        "E" => Some(VisibilityDirection::E),
        "SE" => Some(VisibilityDirection::SE),
        "S" => Some(VisibilityDirection::S),
        "SW" => Some(VisibilityDirection::SW),
        "W" => Some(VisibilityDirection::W),
        "NW" => Some(VisibilityDirection::NW),
        _ => None,
    }
}
