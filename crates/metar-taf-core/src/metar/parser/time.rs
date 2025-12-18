use crate::metar::models::time::MetarTime;

pub fn parse_time(token: &str) -> Option<MetarTime> {
    // formato: DDHHMMZ
    if token.len() != 7 || !token.ends_with('Z') {
        return None;
    }

    let day: u8 = token[0..2].parse().ok()?;
    let hour: u8 = token[2..4].parse().ok()?;
    let minute: u8 = token[4..6].parse().ok()?;

    Some(MetarTime { day, hour, minute })
}
