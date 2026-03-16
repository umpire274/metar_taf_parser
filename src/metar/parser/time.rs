use crate::metar::models::time::MetarTime;

pub fn parse_time(token: &str) -> Option<MetarTime> {
    // format: DDHHMMZ
    if token.len() != 7 || !token.ends_with('Z') {
        return None;
    }

    let body = &token[..6];
    if !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = token[0..2].parse().ok()?;
    let hour: u8 = token[2..4].parse().ok()?;
    let minute: u8 = token[4..6].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 23 || minute > 59 {
        return None;
    }

    Some(MetarTime { day, hour, minute })
}
