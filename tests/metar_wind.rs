use metar_taf_parser::metar::models::wind::WindUnit;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// Standard cases
// ---------------------------------------------------------------------------

#[test]
fn metar_variable_wind_with_gust() {
    let m = parse_metar("LIMC 121250Z VRB03G15KT 9999 SCT020 15/10 Q1018").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, None);
    assert_eq!(wind.speed, 3);
    assert_eq!(wind.gust, Some(15));
    assert!(!wind.indeterminate);
}

#[test]
fn parse_wind_in_mps() {
    let m = parse_metar("METAR XXXX 181200Z 08002MPS CAVOK 10/05 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(80));
    assert_eq!(wind.speed, 2);
    assert_eq!(wind.unit, WindUnit::MPS);
    assert!(!wind.indeterminate);
}

#[test]
fn parse_calm_wind() {
    let m = parse_metar("METAR XXXX 181200Z 00000KT CAVOK 10/05 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(0));
    assert_eq!(wind.speed, 0);
    assert_eq!(wind.gust, None);
    assert_eq!(wind.unit, WindUnit::KT);
    assert!(!wind.indeterminate);
}

#[test]
fn parse_wind_with_gust() {
    // 10009G19KT — example from the manual
    let m = parse_metar("LIRF 121250Z 10009G19KT 9999 FEW030 18/12 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(100));
    assert_eq!(wind.speed, 9);
    assert_eq!(wind.gust, Some(19));
    assert_eq!(wind.unit, WindUnit::KT);
}

#[test]
fn parse_wind_direction_360() {
    // 360 is a valid encoding for north (same as 000)
    let m = parse_metar("LIRF 121250Z 36008KT 9999 FEW030 18/12 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(360));
}

// ---------------------------------------------------------------------------
// MPH
// ---------------------------------------------------------------------------

#[test]
fn parse_wind_in_mph() {
    let m = parse_metar("METAR XXXX 181200Z 18012MPH CAVOK 10/05 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(180));
    assert_eq!(wind.speed, 12);
    assert_eq!(wind.unit, WindUnit::MPH);
    assert!(!wind.indeterminate);
}

// ---------------------------------------------------------------------------
// Indeterminate: /////KT
// ---------------------------------------------------------------------------

#[test]
fn parse_indeterminate_wind() {
    let m = parse_metar("METAR XXXX 181200Z /////KT CAVOK 10/05 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert!(wind.indeterminate);
    assert_eq!(wind.direction, None);
    assert_eq!(wind.speed, 0);
    assert_eq!(wind.unit, WindUnit::KT);
}

#[test]
fn parse_indeterminate_wind_mps() {
    let m = parse_metar("METAR XXXX 181200Z /////MPS CAVOK 10/05 Q1015").unwrap();
    let wind = m.wind.unwrap();
    assert!(wind.indeterminate);
    assert_eq!(wind.unit, WindUnit::MPS);
}

// ---------------------------------------------------------------------------
// Rejection
// ---------------------------------------------------------------------------

#[test]
fn reject_invalid_direction_above_360() {
    let m = parse_metar("METAR XXXX 181200Z 36110KT CAVOK 10/05 Q1015").unwrap();
    assert!(m.wind.is_none());
}

#[test]
fn reject_invalid_non_numeric_wind_speed() {
    let m = parse_metar("METAR XXXX 181200Z 180ABKT CAVOK 10/05 Q1015").unwrap();
    assert!(m.wind.is_none());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_wind_directional_with_gust() {
    let m = parse_metar("LIRF 121250Z 10009G19KT 060V130 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let wind = desc.wind.unwrap();
    assert!(wind.contains("100°"), "direction: {}", wind);
    assert!(wind.contains("9 kt"), "speed: {}", wind);
    assert!(wind.contains("gusting 19 kt"), "gust: {}", wind);
    assert!(wind.contains("variable 60 to 130°"), "variation: {}", wind);
}

#[test]
fn describe_calm_wind() {
    let m = parse_metar("LIRF 121250Z 00000KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let wind = desc.wind.unwrap();
    assert!(wind.contains("0°") || wind.contains("0 kt"), "{}", wind);
}

#[test]
fn describe_vrb_wind() {
    let m = parse_metar("LIRF 121250Z VRB05KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let wind = desc.wind.unwrap();
    assert!(wind.contains("variable direction"), "{}", wind);
    assert!(wind.contains("5 kt"), "{}", wind);
}

#[test]
fn describe_indeterminate_wind() {
    let m = parse_metar("METAR XXXX 181200Z /////KT CAVOK 10/05 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let wind = desc.wind.unwrap();
    assert_eq!(wind, "wind direction and speed not available");
}

#[test]
fn describe_wind_mph() {
    let m = parse_metar("METAR XXXX 181200Z 18012MPH CAVOK 10/05 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let wind = desc.wind.unwrap();
    assert!(wind.contains("mph"), "{}", wind);
}
