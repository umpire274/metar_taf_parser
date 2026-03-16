use crate::metar::models::temperature::Temperature;

fn parse_single_temp(s: &str) -> Option<i8> {
    if s.is_empty() {
        return None;
    }

    let (negative, digits) = if let Some(rest) = s.strip_prefix('M') {
        (true, rest)
    } else {
        (false, s)
    };

    // METAR groups are 2 digits for temperature/dew-point
    if digits.len() != 2 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let mut value: i8 = digits.parse().ok()?;
    if negative {
        value = -value;
    }

    // Conservative meteorological range validation
    if !(-80..=60).contains(&value) {
        return None;
    }

    Some(value)
}

pub fn parse_temperature(token: &str) -> Option<Temperature> {
    let (temp_str, dew_str) = token.split_once('/')?;

    let temperature = parse_single_temp(temp_str)?;
    let dew_point = parse_single_temp(dew_str)?;

    Some(Temperature {
        temperature,
        dew_point,
    })
}
