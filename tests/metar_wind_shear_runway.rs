use metar_taf_parser::metar::models::wind_shear::MetarWindShearRunway;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// WS R<runway> — wind shear on a specific runway
// ---------------------------------------------------------------------------

#[test]
fn parse_wind_shear_single_runway() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS R23",
    )
    .unwrap();

    assert_eq!(m.wind_shear.len(), 1);
    assert_eq!(m.wind_shear[0], MetarWindShearRunway::Runway("23".to_string()));
}

#[test]
fn parse_wind_shear_runway_with_suffix() {
    // WS R06R — runway 06 Right
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS R06R",
    )
    .unwrap();

    assert_eq!(m.wind_shear[0], MetarWindShearRunway::Runway("06R".to_string()));
}

// ---------------------------------------------------------------------------
// WS ALL RWY — wind shear on all runways
// ---------------------------------------------------------------------------

#[test]
fn parse_wind_shear_all_runways() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS ALL RWY",
    )
    .unwrap();

    assert_eq!(m.wind_shear.len(), 1);
    assert_eq!(m.wind_shear[0], MetarWindShearRunway::AllRunways);
}

// ---------------------------------------------------------------------------
// Multiple WS groups
// ---------------------------------------------------------------------------

#[test]
fn parse_multiple_wind_shear_groups() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS R23 WS R05",
    )
    .unwrap();

    assert_eq!(m.wind_shear.len(), 2);
    assert_eq!(m.wind_shear[0], MetarWindShearRunway::Runway("23".to_string()));
    assert_eq!(m.wind_shear[1], MetarWindShearRunway::Runway("05".to_string()));
}

// ---------------------------------------------------------------------------
// No wind shear in an ordinary METAR
// ---------------------------------------------------------------------------

#[test]
fn no_wind_shear_in_ordinary_metar() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(m.wind_shear.is_empty());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_wind_shear_single_runway() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS R23",
    )
    .unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.wind_shear.len(), 1);
    assert!(desc.wind_shear[0].contains("23"), "{}", desc.wind_shear[0]);
    assert!(
        desc.wind_shear[0].to_lowercase().contains("wind shear"),
        "{}",
        desc.wind_shear[0]
    );
}

#[test]
fn describe_wind_shear_all_runways() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS ALL RWY",
    )
    .unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.wind_shear.len(), 1);
    assert!(
        desc.wind_shear[0].to_lowercase().contains("all"),
        "{}",
        desc.wind_shear[0]
    );
}

#[test]
fn format_metar_includes_wind_shear_line() {
    let m = parse_metar(
        "METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS ALL RWY",
    )
    .unwrap();
    let text = metar_taf_parser::format_metar(&m, Language::En);
    assert!(text.contains("Wind shear:"), "{}", text);
}

