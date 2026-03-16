//! Module `metar`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::common::report_modifier::ReportModifier;
use crate::common::tokenizer::Tokenizer;
use crate::metar::errors::MetarError;
use crate::metar::models::Metar;
use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::pressure::parse_pressure;
use crate::metar::parser::runway_state::parse_runway_state;
use crate::metar::parser::rvr::parse_rvr;
use crate::metar::parser::temperature::parse_temperature;
use crate::metar::parser::time::parse_time;
use crate::metar::parser::trend::{parse_trend, parse_trend_detail};
use crate::metar::parser::visibility::{parse_split_statute_miles_to_meters, parse_visibility};
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;

/// Parses input tokens into typed data for `parse_metar`.
pub fn parse_metar(input: &str) -> Result<Metar, MetarError> {
    parse_metar_with_mode(input, false)
}

/// Parses input tokens into typed data for `parse_metar_strict`.
pub fn parse_metar_strict(input: &str) -> Result<Metar, MetarError> {
    parse_metar_with_mode(input, true)
}

/// Parses input tokens into typed data for `parse_metar_with_mode`.
fn parse_metar_with_mode(input: &str, strict: bool) -> Result<Metar, MetarError> {
    let mut tokenizer = Tokenizer::new(input);

    // Optional leading "METAR"
    let first = tokenizer.next().ok_or(MetarError::InvalidFormat)?;
    let token = if first == "METAR" {
        tokenizer.next()
    } else {
        Some(first)
    }
    .ok_or(MetarError::InvalidFormat)?;

    // NIL report: may appear as "METAR NIL" (or sometimes "NIL" after METAR/COR)
    if token == "NIL" {
        let mut metar = Metar::new("", input);
        metar.modifier = ReportModifier::Nil;
        return Ok(metar);
    }

    // Optional COR
    let (modifier, station) = if token == "COR" {
        let s = tokenizer.next().ok_or(MetarError::InvalidFormat)?;
        (ReportModifier::Correction, s)
    } else {
        (ReportModifier::Normal, token)
    };

    let mut metar = Metar::new(&station, input);
    metar.modifier = modifier;

    let mut rmk_started = false;
    let mut remaining_tokens: Vec<String> = Vec::new();

    while let Some(token) = tokenizer.next() {
        if token == "RMK" {
            rmk_started = true;
            continue;
        }

        if rmk_started {
            remaining_tokens.push(token.to_string());
            continue;
        }

        // AUTO may appear after time group (but we accept it anywhere before RMK)
        if token == "AUTO" {
            metar.modifier = ReportModifier::Auto;
            continue;
        }

        if token == "NIL" {
            // If NIL appears after station/time in some feeds, treat as NIL and stop
            metar.modifier = ReportModifier::Nil;
            break;
        }

        if let Some(v) = parse_time(&token) {
            metar.time = Some(v);
            continue;
        }

        if let Some(v) = parse_wind(&token) {
            metar.wind = Some(v);
            continue;
        }

        if token.chars().all(|c| c.is_ascii_digit())
            && let Some(next) = tokenizer.peek()
            && let Some(prevailing) = parse_split_statute_miles_to_meters(&token, next)
        {
            metar.visibility = Some(Visibility::Single { prevailing });
            let _ = tokenizer.next();
            continue;
        }

        if let Some(v) = parse_visibility(&token, &metar) {
            metar.visibility = Some(v);

            if matches!(metar.visibility, Some(Visibility::CAVOK)) {
                metar.clouds.clear();
            }
            continue;
        }

        if let Some(rvr) = parse_rvr(&token) {
            metar.runway_visual_range.push(rvr);
            continue;
        }

        if let Some(rs) = parse_runway_state(&token) {
            metar.runway_state.push(rs);
            continue;
        }

        if let Some(v) = parse_cloud(&token) {
            metar.clouds.push(v);
            continue;
        }

        if let Some(v) = parse_temperature(&token) {
            metar.temperature = Some(v);
            continue;
        }

        if let Some(v) = parse_pressure(&token) {
            metar.pressure = Some(v);
            continue;
        }

        if let Some(t) = parse_trend(&token) {
            metar.trend = Some(t.clone());
            if !matches!(t, crate::metar::models::trend::MetarTrend::Nosig) {
                metar.trend_detail = Some(parse_trend_detail(t, &mut tokenizer));
            }
            continue;
        }

        if let Some(v) = parse_weather(&token) {
            metar.weather.push(v);
            continue;
        }

        metar.unparsed_groups.push(token.to_string());
    }

    if rmk_started && !remaining_tokens.is_empty() {
        metar.rmk = Some(remaining_tokens.join(" "));
    }

    if strict {
        let mut unsupported = metar.unparsed_groups.clone();
        if let Some(detail) = &metar.trend_detail {
            unsupported.extend(detail.unparsed_groups.clone());
        }

        if !unsupported.is_empty() {
            return Err(MetarError::UnknownGroup(unsupported.join(" ")));
        }
    }

    Ok(metar)
}
