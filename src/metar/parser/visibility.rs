//! Module `visibility`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::Metar;
use crate::metar::models::visibility::{Visibility, VisibilityDirection, VisibilityQualifier};

const METERS_PER_STATUTE_MILE: f64 = 1609.344;

/// Parses a visibility token (or the first of a two-token statute-mile group)
/// into a [`Visibility`] variant.
pub fn parse_visibility(token: &str, metar: &Metar) -> Option<Visibility> {
    // CAVOK
    if token == "CAVOK" {
        return Some(Visibility::CAVOK);
    }

    // Statute miles (single token, possibly with P/M prefix): 3SM, 3/4SM, P6SM, M1/4SM
    if let Some((prevailing, qualifier)) = parse_statute_miles_to_meters(token) {
        return Some(Visibility::Single { prevailing, qualifier, ndv: false });
    }

    // Strip NDV suffix before metric parsing
    let (core, ndv) = if let Some(base) = token.strip_suffix("NDV") {
        (base, true)
    } else {
        (token, false)
    };

    // Prevailing metric visibility: exactly 4 digits (0000–9999)
    if core.len() == 4 && core.chars().all(|c| c.is_ascii_digit()) {
        let prevailing: u16 = core.parse().ok()?;
        return Some(Visibility::Single { prevailing, qualifier: None, ndv });
    }

    // Directional minimum (2000SW): 4+ chars, last 1-2 chars are a compass direction
    // NDV doesn't apply here; only metric single groups carry NDV
    if !ndv && token.len() >= 5 {
        let (dist_part, dir_part) = token.split_at(token.len() - 2);
        if let (Ok(minimum), Some(direction)) =
            (dist_part.parse::<u16>(), parse_visibility_direction(dir_part))
        {
            if let Some(Visibility::Single { prevailing, .. }) = metar.visibility {
                return Some(Visibility::WithMinimum { prevailing, minimum, direction });
            }
        }

        // Try 1-char direction suffix (N, E, S, W)
        let (dist_part, dir_part) = token.split_at(token.len() - 1);
        if let (Ok(minimum), Some(direction)) =
            (dist_part.parse::<u16>(), parse_visibility_direction(dir_part))
        {
            if let Some(Visibility::Single { prevailing, .. }) = metar.visibility {
                return Some(Visibility::WithMinimum { prevailing, minimum, direction });
            }
        }
    }

    None
}

/// Parses a two-token statute-mile group: integer whole miles + `n/dSM` fraction.
///
/// Example: `("1", "3/4SM")` → 1.75 statute miles → metres.
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

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Parses a single statute-mile token into metres and an optional qualifier.
///
/// Returns `(metres, qualifier)` or `None` if the token is not a SM group.
fn parse_statute_miles_to_meters(token: &str) -> Option<(u16, Option<VisibilityQualifier>)> {
    let cleaned = token.strip_suffix("SM")?;

    let (qualifier, numeric) = if let Some(rest) = cleaned.strip_prefix('P') {
        (Some(VisibilityQualifier::Above), rest)
    } else if let Some(rest) = cleaned.strip_prefix('M') {
        (Some(VisibilityQualifier::Below), rest)
    } else {
        (None, cleaned)
    };

    let meters = if let Ok(whole) = numeric.parse::<f64>() {
        miles_to_meters(whole)?
    } else {
        let fraction = parse_fraction(numeric)?;
        miles_to_meters(fraction)?
    };

    Some((meters, qualifier))
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
