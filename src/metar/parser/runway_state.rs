use crate::metar::models::runway_state::RunwayState;

pub fn parse_runway_state(token: &str) -> Option<RunwayState> {
    if !token.starts_with('R') {
        return None;
    }

    let (rwy_str, data) = token[1..].split_once('/')?;

    // Runway designator: exactly 2 numeric characters
    if rwy_str.len() != 2 || !rwy_str.chars().all(|c| c.is_ascii_digit()) {
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
