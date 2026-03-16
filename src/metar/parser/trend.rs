//! Module `trend`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::common::tokenizer::Tokenizer;
use crate::metar::models::trend::{
    MetarTrend, MetarTrendDetail, MetarTrendTime, MetarTrendTimeKind,
};
use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::visibility::{parse_split_statute_miles_to_meters, parse_visibility};
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;

/// Parses input tokens into typed data for `parse_trend`.
pub fn parse_trend(token: &str) -> Option<MetarTrend> {
    match token {
        "NOSIG" => Some(MetarTrend::Nosig),
        "BECMG" => Some(MetarTrend::Becmg),
        "TEMPO" => Some(MetarTrend::Tempo),
        _ => None,
    }
}

/// Parses input tokens into typed data for `parse_trend_detail`.
pub fn parse_trend_detail(kind: MetarTrend, tokenizer: &mut Tokenizer) -> MetarTrendDetail {
    let mut detail = MetarTrendDetail {
        kind,
        times: Vec::new(),
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
        raw_tokens: Vec::new(),
        unparsed_groups: Vec::new(),
    };

    while let Some(next) = tokenizer.peek() {
        if next == "RMK" || parse_trend(next).is_some() {
            break;
        }

        let token = match tokenizer.next() {
            Some(t) => t,
            None => break,
        };
        detail.raw_tokens.push(token.clone());

        if let Some(t) = parse_trend_time(&token) {
            detail.times.push(t);
            continue;
        }

        if let Some(v) = parse_wind(&token) {
            detail.wind = Some(v);
            continue;
        }

        if token.chars().all(|c| c.is_ascii_digit())
            && let Some(next) = tokenizer.peek()
            && let Some(prevailing) = parse_split_statute_miles_to_meters(&token, next)
        {
            detail.visibility = Some(Visibility::Single { prevailing });
            let split = tokenizer.next().unwrap_or_default();
            detail.raw_tokens.push(split);
            continue;
        }

        if let Some(v) = parse_visibility(
            &token,
            &fake_metar_with_visibility(detail.visibility.clone()),
        ) {
            detail.visibility = Some(v);
            continue;
        }

        if let Some(v) = parse_cloud(&token) {
            detail.clouds.push(v);
            continue;
        }

        if let Some(v) = parse_weather(&token) {
            detail.weather.push(v);
            continue;
        }

        detail.unparsed_groups.push(token);
    }

    detail
}

/// Parses input tokens into typed data for `parse_trend_time`.
fn parse_trend_time(token: &str) -> Option<MetarTrendTime> {
    if token.len() != 6 {
        return None;
    }

    let (prefix, hhmm) = token.split_at(2);
    let kind = match prefix {
        "FM" => MetarTrendTimeKind::From,
        "TL" => MetarTrendTimeKind::Until,
        "AT" => MetarTrendTimeKind::At,
        _ => return None,
    };

    let hour: u8 = hhmm[0..2].parse().ok()?;
    let minute: u8 = hhmm[2..4].parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }

    Some(MetarTrendTime { kind, hour, minute })
}

/// Creates a new `fake_metar_with_visibility` value with normalized defaults.
fn fake_metar_with_visibility(visibility: Option<Visibility>) -> crate::metar::models::Metar {
    let mut metar = crate::metar::models::Metar::new("", "");
    metar.visibility = visibility;
    metar
}
