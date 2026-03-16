use std::iter::Peekable;
use std::slice::Iter;

use crate::common::report_modifier::ReportModifier;
use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::visibility::{parse_split_statute_miles_to_meters, parse_visibility};
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;
use crate::taf::models::forecast::{TafForecast, TafForecastKind};
use crate::taf::models::time::TafPeriod;

/// Entry point: parse tutti i forecast TAF
pub fn parse_forecasts(tokens: &[String]) -> (Vec<TafForecast>, Vec<String>) {
    let mut forecasts = Vec::new();
    let mut unparsed_groups = Vec::new();

    let mut current = new_base_forecast();
    let mut visibility_context: Option<Visibility> = None;

    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        // -------- FM --------
        if let Some(from) = parse_fm_time(token) {
            forecasts.push(current);
            current = new_fm_forecast(from);
            visibility_context = None;
            continue;
        }

        // -------- BECMG --------
        if token == "BECMG"
            && let Some(period) = try_consume_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::BECMG, period, None);
            visibility_context = None;
            continue;
        }

        // -------- TEMPO --------
        if token == "TEMPO"
            && let Some(period) = try_consume_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::TEMPO, period, None);
            visibility_context = None;
            continue;
        }

        // -------- PROB30 / PROB40 --------
        if let Some(prob) = parse_prob(token)
            && let Some(period) = try_consume_prob_period(&mut iter)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::PROB, period, Some(prob));
            visibility_context = None;
            continue;
        }

        // -------- Wind --------
        if let Some(wind) = parse_wind(token) {
            current.wind = Some(wind);
            continue;
        }

        // -------- Visibility --------
        if token.chars().all(|c| c.is_ascii_digit())
            && let Some(next) = iter.peek()
            && let Some(prevailing) = parse_split_statute_miles_to_meters(token, next)
        {
            let vis = Visibility::Single { prevailing };
            visibility_context = Some(vis.clone());
            current.visibility = Some(vis);
            iter.next();
            continue;
        }

        if let Some(vis) = parse_visibility(token, &fake_metar(visibility_context.clone())) {
            visibility_context = Some(vis.clone());
            current.visibility = Some(vis);
            continue;
        }

        // -------- Clouds --------
        if let Some(cloud) = parse_cloud(token) {
            current.clouds.push(cloud);
            continue;
        }

        // -------- Weather --------
        if let Some(weather) = parse_weather(token) {
            current.weather.push(weather);
            continue;
        }

        unparsed_groups.push(token.to_string());
    }

    forecasts.push(current);
    (forecasts, unparsed_groups)
}

// ===== Forecast builders =====

fn new_base_forecast() -> TafForecast {
    TafForecast {
        kind: TafForecastKind::Base,
        from: None,
        period: None,
        probability: None,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
    }
}

fn new_fm_forecast(from: (u8, u8, u8)) -> TafForecast {
    TafForecast {
        kind: TafForecastKind::FM,
        from: Some(from),
        period: None,
        probability: None,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
    }
}

fn new_period_forecast(
    kind: TafForecastKind,
    period: TafPeriod,
    probability: Option<u8>,
) -> TafForecast {
    TafForecast {
        kind,
        from: None,
        period: Some(period),
        probability,
        wind: None,
        visibility: None,
        weather: Vec::new(),
        clouds: Vec::new(),
    }
}

// ===== Helpers =====

fn parse_fm_time(token: &str) -> Option<(u8, u8, u8)> {
    if !token.starts_with("FM") || token.len() != 8 {
        return None;
    }

    parse_day_hour_min(&token[2..])
}

fn parse_becmg_period(token: &str) -> Option<TafPeriod> {
    let (from, to) = token.split_once('/')?;

    let from = parse_day_hour(from)?;
    let to = parse_day_hour(to)?;

    Some(TafPeriod {
        from: (from.0, from.1, 0),
        to: (to.0, to.1, 0),
    })
}

fn parse_prob(token: &str) -> Option<u8> {
    match token {
        "PROB30" => Some(30),
        "PROB40" => Some(40),
        _ => None,
    }
}

fn try_consume_prob_period(iter: &mut Peekable<Iter<'_, String>>) -> Option<TafPeriod> {
    let mut lookahead = iter.clone();

    if let Some(token) = lookahead.next() {
        if token == "TEMPO" {
            let period_token = lookahead.next()?;
            let period = parse_becmg_period(period_token)?;
            iter.next(); // TEMPO
            iter.next(); // period
            return Some(period);
        }

        let period = parse_becmg_period(token)?;
        iter.next(); // period
        return Some(period);
    }

    None
}

fn try_consume_period(iter: &mut Peekable<Iter<'_, String>>) -> Option<TafPeriod> {
    let mut lookahead = iter.clone();
    let period_token = lookahead.next()?;
    let period = parse_becmg_period(period_token)?;
    iter.next(); // period
    Some(period)
}

fn parse_day_hour(value: &str) -> Option<(u8, u8)> {
    if value.len() != 4 || !value.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = value[0..2].parse().ok()?;
    let hour: u8 = value[2..4].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 24 {
        return None;
    }

    Some((day, hour))
}

fn parse_day_hour_min(value: &str) -> Option<(u8, u8, u8)> {
    if value.len() != 6 || !value.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let day: u8 = value[0..2].parse().ok()?;
    let hour: u8 = value[2..4].parse().ok()?;
    let min: u8 = value[4..6].parse().ok()?;

    if !(1..=31).contains(&day) || hour > 23 || min > 59 {
        return None;
    }

    Some((day, hour, min))
}

fn fake_metar(visibility: Option<Visibility>) -> crate::metar::models::Metar {
    use crate::metar::models::Metar;

    Metar {
        station: String::new(),
        time: None,
        modifier: ReportModifier::Normal,
        wind: None,
        visibility,
        clouds: Vec::new(),
        temperature: None,
        pressure: None,
        weather: Vec::new(),
        rmk: None,
        runway_state: Vec::new(),
        trend: None,
        unparsed_groups: Vec::new(),
        raw: String::new(),
    }
}
