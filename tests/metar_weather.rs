use metar_taf_parser::metar::models::weather::{
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
};
use metar_taf_parser::metar::parser::weather::parse_weather;

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

#[test]
fn parse_freezing_fog() {
    let wx = parse_weather("FZFG").unwrap();

    assert!(wx.descriptors.contains(&WeatherDescriptor::Freezing));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Fog));
}

#[test]
fn parse_vicinity_mist() {
    let wx = parse_weather("VCBR").unwrap();

    assert!(wx.descriptors.contains(&WeatherDescriptor::Vicinity));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Mist));
}

#[test]
fn parse_unknown_phenomena_pair() {
    let wx = parse_weather("BRXX").unwrap();

    assert!(wx.phenomena.contains(&WeatherPhenomenon::Mist));
    assert!(
        wx.phenomena
            .contains(&WeatherPhenomenon::Unknown("XX".to_string()))
    );
}

#[test]
fn reject_malformed_odd_weather_token() {
    assert!(parse_weather("RAX").is_none());
}
