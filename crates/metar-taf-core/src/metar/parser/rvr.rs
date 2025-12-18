use crate::metar::models::rvr::{Rvr, RvrTrend};

pub fn parse_rvr(token: &str) -> Option<Rvr> {
    // Must start with R and contain exactly ONE '/'
    if !token.starts_with('R') {
        return None;
    }

    let slash_count = token.matches('/').count();
    if slash_count != 1 {
        // This is NOT RVR (likely runway state group)
        return None;
    }

    let (runway_part, rest) = token[1..].split_once('/')?;

    let runway = runway_part.to_string();

    let mut rest = rest;

    // Trend
    let trend = match rest.chars().last()? {
        'U' => {
            rest = &rest[..rest.len() - 1];
            Some(RvrTrend::Up)
        }
        'D' => {
            rest = &rest[..rest.len() - 1];
            Some(RvrTrend::Down)
        }
        'N' => {
            rest = &rest[..rest.len() - 1];
            Some(RvrTrend::NoChange)
        }
        _ => None,
    };

    // Variable visibility
    let (min, max) = if let Some((a, b)) = rest.split_once('V') {
        (a.parse().ok()?, Some(b.parse().ok()?))
    } else {
        (rest.parse().ok()?, None)
    };

    Some(Rvr {
        runway,
        min,
        max,
        trend,
    })
}
