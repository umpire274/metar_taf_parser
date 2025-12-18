use crate::common::tokenizer::Tokenizer;
use crate::metar::errors::MetarError;
use crate::metar::models::Metar;
use crate::metar::models::visibility::Visibility;
use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::pressure::parse_pressure;
use crate::metar::parser::rvr::parse_rvr;
use crate::metar::parser::temperature::parse_temperature;
use crate::metar::parser::time::parse_time;
use crate::metar::parser::visibility::parse_visibility;
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;

pub fn parse_metar(input: &str) -> Result<Metar, MetarError> {
    let mut tokenizer = Tokenizer::new(input);

    // Optional header: METAR / SPECI
    let first = tokenizer.next().ok_or(MetarError::InvalidFormat)?;
    let station = if first == "METAR" || first == "SPECI" {
        tokenizer.next().ok_or(MetarError::InvalidFormat)?
    } else {
        first
    };

    let mut metar = Metar::new(&station, input);
    let mut remaining_tokens: Vec<String> = Vec::new();
    let mut rmk_started = false;

    for token in tokenizer {
        if token == "RMK" {
            rmk_started = true;
            continue;
        }

        if rmk_started {
            remaining_tokens.push(token);
            continue;
        }

        if let Some(v) = parse_time(&token) {
            metar.time = Some(v);
            continue;
        }

        if let Some(v) = parse_wind(&token) {
            metar.wind = Some(v);
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
            metar.rvr.push(rvr);
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

        if let Some(v) = parse_weather(&token) {
            metar.weather.push(v);
            continue;
        }
    }

    // ---- RMK ----
    if rmk_started {
        metar.rmk = crate::metar::parser::rmk::parse_rmk(&remaining_tokens);
    }

    Ok(metar)
}
