use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::visibility::parse_visibility;
use crate::metar::parser::wind::parse_wind;
use crate::taf::models::forecast::{TafForecast, TafForecastKind};
use crate::taf::models::time::TafPeriod;

/// Entry point: parse tutti i forecast TAF
pub fn parse_forecasts(tokens: &[String]) -> Vec<TafForecast> {
    let mut forecasts = Vec::new();

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
            && let Some(period_token) = iter.next()
            && let Some(period) = parse_becmg_period(period_token)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::BECMG, period, None);
            visibility_context = None;
            continue;
        }

        // -------- TEMPO --------
        if token == "TEMPO"
            && let Some(period_token) = iter.next()
            && let Some(period) = parse_becmg_period(period_token)
        {
            forecasts.push(current);
            current = new_period_forecast(TafForecastKind::TEMPO, period, None);
            visibility_context = None;
            continue;
        }

        // -------- PROB30 / PROB40 --------
        if let Some(prob) = parse_prob(token) {
            let mut period: Option<TafPeriod> = None;

            if let Some(next) = iter.peek() {
                if *next == "TEMPO" {
                    iter.next(); // consuma TEMPO
                    if let Some(p) = iter.next() {
                        period = parse_becmg_period(p);
                    }
                } else if let Some(p) = iter.next() {
                    period = parse_becmg_period(p);
                }
            }

            if let Some(period) = period {
                forecasts.push(current);
                current = new_period_forecast(TafForecastKind::PROB, period, Some(prob));
                visibility_context = None;
                continue;
            }
        }

        // -------- Wind --------
        if let Some(wind) = parse_wind(token) {
            current.wind = Some(wind);
            continue;
        }

        // -------- Visibility --------
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
    }

    forecasts.push(current);
    forecasts
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
        clouds: Vec::new(),
    }
}

// ===== Helpers =====

fn parse_fm_time(token: &str) -> Option<(u8, u8, u8)> {
    if !token.starts_with("FM") || token.len() != 8 {
        return None;
    }

    Some((
        token[2..4].parse().ok()?,
        token[4..6].parse().ok()?,
        token[6..8].parse().ok()?,
    ))
}

fn parse_becmg_period(token: &str) -> Option<TafPeriod> {
    let (from, to) = token.split_once('/')?;

    Some(TafPeriod {
        from: (
            from.get(0..2)?.parse().ok()?,
            from.get(2..4)?.parse().ok()?,
            0,
        ),
        to: (to.get(0..2)?.parse().ok()?, to.get(2..4)?.parse().ok()?, 0),
    })
}

fn parse_prob(token: &str) -> Option<u8> {
    match token {
        "PROB30" => Some(30),
        "PROB40" => Some(40),
        _ => None,
    }
}

fn fake_metar(visibility: Option<Visibility>) -> crate::metar::models::Metar {
    use crate::metar::models::Metar;

    Metar {
        station: String::new(),
        time: None,
        automated: false,
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
