use metar_taf_parser::metar::models::weather::{
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
};
use metar_taf_parser::metar::parser::weather::parse_weather;

// ---------------------------------------------------------------------------
// Intensity
// ---------------------------------------------------------------------------

#[test]
fn parse_light_rain() {
    let wx = parse_weather("-RA").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Light)));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_heavy_snow() {
    let wx = parse_weather("+SN").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Heavy)));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Snow));
}

#[test]
fn parse_moderate_rain_no_prefix() {
    let wx = parse_weather("RA").unwrap();
    assert!(wx.intensity.is_none()); // moderate = no prefix
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_recent_rain() {
    let wx = parse_weather("RERA").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Recent)));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_recent_thunderstorm() {
    let wx = parse_weather("RETS").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Recent)));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Thunder));
}

// ---------------------------------------------------------------------------
// Descriptors
// ---------------------------------------------------------------------------

#[test]
fn parse_mist() {
    let wx = parse_weather("BR").unwrap();
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Mist));
}

#[test]
fn parse_freezing_fog() {
    let wx = parse_weather("FZFG").unwrap();
    assert!(wx.descriptors.contains(&WeatherDescriptor::Freezing));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Fog));
}

#[test]
fn parse_vicinity_showers() {
    let wx = parse_weather("VCSH").unwrap();
    assert!(wx.descriptors.contains(&WeatherDescriptor::Vicinity));
    assert!(
        wx.phenomena.contains(&WeatherPhenomenon::Hail)
            || wx.descriptors.contains(&WeatherDescriptor::Showers)
    );
}

#[test]
fn parse_heavy_showers_rain() {
    let wx = parse_weather("+SHRA").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Heavy)));
    assert!(wx.descriptors.contains(&WeatherDescriptor::Showers));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_thunderstorm_rain() {
    let wx = parse_weather("+TSRA").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Heavy)));
    assert!(wx.descriptors.contains(&WeatherDescriptor::Thunderstorm));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn parse_blowing_snow() {
    let wx = parse_weather("BLSN").unwrap();
    assert!(wx.descriptors.contains(&WeatherDescriptor::Blowing));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Snow));
}

#[test]
fn parse_low_drifting_sand() {
    let wx = parse_weather("DRSA").unwrap();
    assert!(wx.descriptors.contains(&WeatherDescriptor::LowDrifting));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Sand));
}

// ---------------------------------------------------------------------------
// All phenomena from the manual
// ---------------------------------------------------------------------------

#[test]
fn parse_haze() {
    assert!(
        parse_weather("HZ")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Haze)
    );
}

#[test]
fn parse_smoke() {
    assert!(
        parse_weather("FU")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Smoke)
    );
}

#[test]
fn parse_ice_crystals() {
    assert!(
        parse_weather("IC")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::IceCrystals)
    );
}

#[test]
fn parse_ice_pellets_pl() {
    assert!(
        parse_weather("PL")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::IcePellets)
    );
}

#[test]
fn parse_ice_pellets_pe_legacy() {
    // PE is the legacy ICAO code, must map to IcePellets
    assert!(
        parse_weather("PE")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::IcePellets)
    );
}

#[test]
fn parse_sand_whirls() {
    assert!(
        parse_weather("PO")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::SandWhirls)
    );
}

#[test]
fn parse_squalls() {
    assert!(
        parse_weather("SQ")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Squalls)
    );
}

#[test]
fn parse_funnel_cloud() {
    assert!(
        parse_weather("FC")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::FunnelCloud)
    );
}

#[test]
fn parse_sand() {
    assert!(
        parse_weather("SA")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Sand)
    );
}

#[test]
fn parse_dust() {
    assert!(
        parse_weather("DU")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Dust)
    );
}

#[test]
fn parse_dust_storm() {
    assert!(
        parse_weather("DS")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::DustStorm)
    );
}

#[test]
fn parse_sand_storm() {
    assert!(
        parse_weather("SS")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::SandStorm)
    );
}

#[test]
fn parse_volcanic_ash() {
    assert!(
        parse_weather("VA")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::VolcanicAsh)
    );
}

#[test]
fn parse_unknown_precipitation() {
    assert!(
        parse_weather("UP")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::UnknownPrecipitation)
    );
}

#[test]
fn parse_spray() {
    assert!(
        parse_weather("PY")
            .unwrap()
            .phenomena
            .contains(&WeatherPhenomenon::Spray)
    );
}

// ---------------------------------------------------------------------------
// Combined and edge cases
// ---------------------------------------------------------------------------

#[test]
fn parse_mixed_rain_snow() {
    // RASN — rain and snow, more rain than snow (per manual: most significant first)
    let wx = parse_weather("RASN").unwrap();
    assert_eq!(wx.phenomena[0], WeatherPhenomenon::Rain);
    assert_eq!(wx.phenomena[1], WeatherPhenomenon::Snow);
}

#[test]
fn parse_heavy_showers_hail() {
    let wx = parse_weather("+SHGR").unwrap();
    assert!(matches!(wx.intensity, Some(WeatherIntensity::Heavy)));
    assert!(wx.descriptors.contains(&WeatherDescriptor::Showers));
    assert!(wx.phenomena.contains(&WeatherPhenomenon::Hail));
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
