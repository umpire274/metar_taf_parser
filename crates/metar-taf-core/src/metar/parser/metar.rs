use crate::common::tokenizer::Tokenizer;
use crate::metar::errors::MetarError;
use crate::metar::models::Metar;

use crate::metar::parser::cloud::parse_cloud;
use crate::metar::parser::pressure::parse_pressure;
use crate::metar::parser::temperature::parse_temperature;
use crate::metar::parser::time::parse_time;
use crate::metar::parser::visibility::parse_visibility;
use crate::metar::parser::weather::parse_weather;
use crate::metar::parser::wind::parse_wind;

pub fn parse_metar(input: &str) -> Result<Metar, MetarError> {
    let mut tokenizer = Tokenizer::new(input);

    let station = tokenizer.next().ok_or(MetarError::InvalidFormat)?;

    let mut metar = Metar::new(&station, input);

    for token in tokenizer {
        if let Some(v) = parse_time(&token) {
            metar.time = Some(v);
        } else if let Some(v) = parse_wind(&token) {
            metar.wind = Some(v);
        } else if let Some(v) = parse_visibility(&token, &metar) {
            metar.visibility = Some(v);
        } else if let Some(v) = parse_cloud(&token) {
            metar.clouds.push(v);
        } else if let Some(v) = parse_temperature(&token) {
            metar.temperature = Some(v);
        } else if let Some(v) = parse_pressure(&token) {
            metar.pressure = Some(v);
        } else if let Some(v) = parse_weather(&token) {
            metar.weather.push(v);
        }
    }

    Ok(metar)
}
