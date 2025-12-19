use crate::metar::models::runway_state::RunwayState;

pub fn parse_runway_state(token: &str) -> Option<RunwayState> {
    if !token.starts_with('R') {
        return None;
    }

    let (rwy_str, data) = token[1..].split_once('/')?;

    // Runway designator: exactly 2 digits
    if rwy_str.len() != 2 {
        return None;
    }
    let runway_designator: u8 = rwy_str.parse().ok()?;

    // Data must be exactly 6 characters
    if data.len() != 6 {
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

        // Group 2
        deposit_type: parse_digit(chars[0]),

        // Group 3
        contamination_extent: parse_digit(chars[1]),

        // Group 4 (string, ICAO code)
        deposit_depth: parse_two_str(chars[2], chars[3]),

        // Group 5 (string, ICAO code)
        braking_action: parse_two_str(chars[4], chars[5]),
    })
}
