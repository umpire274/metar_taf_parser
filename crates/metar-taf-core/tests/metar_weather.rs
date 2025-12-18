use metar_taf_core::metar::models::weather::{WeatherIntensity, WeatherPhenomenon};
use metar_taf_core::metar::parser::weather::parse_weather;

#[test]
fn parse_light_rain() {
    let rain = parse_weather("-RA").unwrap();

    assert!(matches!(rain.intensity, Some(WeatherIntensity::Light)));
    assert!(rain.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_mist() {
    let mist = parse_weather("BR").unwrap();

    assert!(mist.phenomena.contains(&WeatherPhenomenon::Mist));
}
