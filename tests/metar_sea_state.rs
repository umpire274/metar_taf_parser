use metar_taf_parser::metar::models::sea_state::WaveHeightKind;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// Basic sea state parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_sea_state_standard() {
    // W12/S8 — 12°C, WMO state code 8
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W12/S8").unwrap();

    let ss = m.sea_state.unwrap();
    assert_eq!(ss.water_temperature, Some(12));
    assert_eq!(ss.wave_kind, WaveHeightKind::StateCode);
    assert_eq!(ss.wave_value, Some(8));
}

#[test]
fn parse_sea_state_zero() {
    // W00/S0 — 0°C, glassy sea
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 01/M01 Q1013 W00/S0").unwrap();

    let ss = m.sea_state.unwrap();
    assert_eq!(ss.water_temperature, Some(0));
    assert_eq!(ss.wave_value, Some(0));
}

#[test]
fn parse_sea_state_negative_temperature() {
    // WM2/S3 — −2°C, slight sea
    let m = parse_metar("METAR ENBO 120930Z 10005KT 9999 FEW020 M01/M04 Q1008 WM2/S3").unwrap();

    let ss = m.sea_state.unwrap();
    assert_eq!(ss.water_temperature, Some(-2));
    assert_eq!(ss.wave_kind, WaveHeightKind::StateCode);
    assert_eq!(ss.wave_value, Some(3));
}

#[test]
fn parse_sea_state_height_dm() {
    // W18/H25 — 18°C, significant wave height 25 dm
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 18/14 Q1013 W18/H25").unwrap();

    let ss = m.sea_state.unwrap();
    assert_eq!(ss.water_temperature, Some(18));
    assert_eq!(ss.wave_kind, WaveHeightKind::HeightDm);
    assert_eq!(ss.wave_value, Some(25));
}

#[test]
fn parse_sea_state_not_available() {
    // W//S/ — both values missing
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W//S/").unwrap();

    let ss = m.sea_state.unwrap();
    assert!(ss.water_temperature.is_none());
    assert_eq!(ss.wave_kind, WaveHeightKind::StateCode);
    assert!(ss.wave_value.is_none());
}

#[test]
fn no_sea_state_in_land_metar() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(m.sea_state.is_none());
}

// ---------------------------------------------------------------------------
// Sea state describe
// ---------------------------------------------------------------------------

#[test]
fn describe_sea_state_standard() {
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W12/S8").unwrap();
    let desc = describe_metar(&m, Language::En);
    let ss = desc.sea_state.unwrap();
    assert!(ss.contains("12°C"), "{}", ss);
    assert!(ss.contains('8'), "{}", ss);
}

#[test]
fn describe_sea_state_negative_temperature() {
    let m = parse_metar("METAR ENBO 120930Z 10005KT 9999 FEW020 M01/M04 Q1008 WM2/S3").unwrap();
    let desc = describe_metar(&m, Language::En);
    let ss = desc.sea_state.unwrap();
    assert!(ss.contains("-2°C"), "{}", ss);
}

#[test]
fn describe_sea_state_not_available() {
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W//S/").unwrap();
    let desc = describe_metar(&m, Language::En);
    let ss = desc.sea_state.unwrap();
    assert!(ss.to_lowercase().contains("not available"), "{}", ss);
}

#[test]
fn format_metar_includes_sea_state_line() {
    let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W12/S8").unwrap();
    let text = metar_taf_parser::format_metar(&m, Language::En);
    assert!(text.contains("Sea state:"), "{}", text);
}
