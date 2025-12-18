use crate::metar::models::wind::Wind;

pub fn parse_wind(token: &str) -> Option<Wind> {
    if !token.ends_with("KT") {
        return None;
    }

    let body = &token[..token.len() - 2];

    let (direction, rest) = if let Some(rest) = body.strip_prefix("VRB") {
        (None, rest)
    } else if body.len() >= 3 {
        let dir: u16 = body[0..3].parse().ok()?;
        (Some(dir), &body[3..])
    } else {
        return None;
    };

    let (speed, gust) = if let Some(idx) = rest.find('G') {
        (
            rest[..idx].parse().ok()?,
            Some(rest[idx + 1..].parse().ok()?),
        )
    } else {
        (rest.parse().ok()?, None)
    };

    Some(Wind {
        direction,
        speed_kt: speed,
        gust_kt: gust,
    })
}
