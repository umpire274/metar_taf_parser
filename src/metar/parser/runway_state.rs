//! Module `runway_state`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::metar::models::runway_state::RunwayState;

/// Parses a runway state token into a [`RunwayState`] value.
///
/// Accepts the standard ICAO form `R[RR[D]]/DCTTBB` and the special
/// closure token `R/SNOCLO` (airfield closed due to snow or ice).
///
/// `RR` is a two-digit runway number; the optional `D` suffix is one of
/// `L`, `R`, or `C` (Left, Right, Centre).
pub fn parse_runway_state(token: &str) -> Option<RunwayState> {
    if !token.starts_with('R') {
        return None;
    }

    let after_r = &token[1..];
    let (rwy_str, data) = after_r.split_once('/')?;

    // Special case: R/SNOCLO — airfield closed due to snow/ice
    if rwy_str.is_empty() && data == "SNOCLO" {
        return Some(RunwayState {
            runway_designator: String::new(),
            snoclo: true,
            deposit_type: None,
            coverage: None,
            thickness: None,
            braking_action: None,
        });
    }

    // Validate runway designator: 2 digits optionally followed by L, R, or C
    if !is_valid_designator(rwy_str) {
        return None;
    }

    let runway_designator = rwy_str.to_string();

    // Data must be exactly 6 chars, only digits or '/'
    if data.len() != 6 || !data.chars().all(|c| c.is_ascii_digit() || c == '/') {
        return None;
    }

    let chars: Vec<char> = data.chars().collect();

    let parse_digit = |c: char| {
        if c == '/' {
            None
        } else {
            c.to_digit(10).map(|v| v as u8)
        }
    };

    let parse_two_str = |a: char, b: char| {
        if a == '/' && b == '/' {
            None
        } else {
            Some(format!("{a}{b}"))
        }
    };

    Some(RunwayState {
        runway_designator,
        snoclo: false,
        deposit_type: parse_digit(chars[0]),
        coverage: parse_digit(chars[1]),
        thickness: parse_two_str(chars[2], chars[3]),
        braking_action: parse_two_str(chars[4], chars[5]),
    })
}

/// Returns `true` for valid runway designators: `"05"`, `"23L"`, `"06R"`, `"18C"`.
fn is_valid_designator(s: &str) -> bool {
    match s.len() {
        2 => s.chars().all(|c| c.is_ascii_digit()),
        3 => {
            s[0..2].chars().all(|c| c.is_ascii_digit())
                && matches!(s.chars().nth(2), Some('L' | 'R' | 'C'))
        }
        _ => false,
    }
}
