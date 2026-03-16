use crate::metar::models::rvr::{RunwayVisualRange, RvrQualifier, RvrTendency, RvrUnit, RvrValue};

pub fn parse_rvr(token: &str) -> Option<RunwayVisualRange> {
    if !token.starts_with('R') {
        return None;
    }

    let (runway, rest) = token[1..].split_once('/')?;

    if !is_valid_runway_designator(runway) {
        return None;
    }

    let (body, tendency) = parse_tendency(rest);
    let (body, unit) = parse_unit(body);

    let (min_part, max_part) = if let Some((a, b)) = body.split_once('V') {
        (a, Some(b))
    } else {
        (body, None)
    };

    let min = parse_rvr_value(min_part)?;
    let max = if let Some(part) = max_part {
        Some(parse_rvr_value(part)?)
    } else {
        None
    };

    Some(RunwayVisualRange {
        runway_designator: runway.to_string(),
        min,
        max,
        tendency,
        unit,
    })
}

fn is_valid_runway_designator(runway: &str) -> bool {
    if runway.len() != 2 && runway.len() != 3 {
        return false;
    }

    let base = &runway[0..2];
    if !base.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if runway.len() == 3 {
        matches!(&runway[2..3], "L" | "R" | "C")
    } else {
        true
    }
}

fn parse_tendency(rest: &str) -> (&str, Option<RvrTendency>) {
    if let Some(last) = rest.chars().last() {
        let tendency = match last {
            'U' => Some(RvrTendency::Upward),
            'D' => Some(RvrTendency::Downward),
            'N' => Some(RvrTendency::NoChange),
            _ => None,
        };

        if tendency.is_some() {
            let cut = rest.len() - last.len_utf8();
            return (&rest[..cut], tendency);
        }
    }

    (rest, None)
}

fn parse_unit(rest: &str) -> (&str, RvrUnit) {
    if let Some(v) = rest.strip_suffix("FT") {
        (v, RvrUnit::Feet)
    } else {
        (rest, RvrUnit::Meters)
    }
}

fn parse_rvr_value(value: &str) -> Option<RvrValue> {
    if value.len() != 4 && value.len() != 5 {
        return None;
    }

    let (qualifier, digits) = match &value[0..1] {
        "P" => (Some(RvrQualifier::Above), &value[1..]),
        "M" => (Some(RvrQualifier::Below), &value[1..]),
        _ => (None, value),
    };

    if digits.len() != 4 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let parsed = digits.parse().ok()?;

    Some(RvrValue {
        value: parsed,
        qualifier,
    })
}
