use crate::metar::models::wind::{Wind, WindUnit};

fn strip_unit(token: &str) -> Option<(&str, WindUnit)> {
    if let Some(rest) = token.strip_suffix("KT") {
        Some((rest, WindUnit::KT))
    } else if let Some(rest) = token.strip_suffix("MPS") {
        Some((rest, WindUnit::MPS))
    } else {
        None
    }
}

pub fn parse_wind(token: &str) -> Option<Wind> {
    // 1. Strip unit first
    let (body, unit) = strip_unit(token)?;

    // 2. Direction
    let (direction, rest) = if let Some(rest) = body.strip_prefix("VRB") {
        (None, rest)
    } else if body.len() >= 3 {
        let dir: u16 = body.get(0..3)?.parse().ok()?;
        (Some(dir), &body[3..])
    } else {
        return None;
    };

    // 3. Speed / Gust
    let (speed, gust) = if let Some((s, g)) = rest.split_once('G') {
        (s.parse().ok()?, Some(g.parse().ok()?))
    } else {
        (rest.parse().ok()?, None)
    };

    Some(Wind {
        direction,
        speed,
        gust,
        unit,
    })
}
