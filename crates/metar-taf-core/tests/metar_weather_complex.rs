use metar_taf_core::metar::models::weather::{
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
};
use metar_taf_core::metar::parser::weather::parse_weather;

#[test]
fn parse_thunderstorm_rain() {
    let wx = parse_weather("+TSRA").unwrap();

    assert!(matches!(wx.intensity, Some(WeatherIntensity::Heavy)));

    assert!(
        wx.descriptors.contains(&WeatherDescriptor::Thunderstorm),
        "expected TS descriptor"
    );

    assert!(
        wx.phenomena.contains(&WeatherPhenomenon::Rain),
        "expected RA phenomenon"
    );
}
