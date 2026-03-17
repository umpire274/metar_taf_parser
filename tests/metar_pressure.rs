use metar_taf_parser::metar::models::pressure::Pressure;
use metar_taf_parser::metar::parser::pressure::parse_pressure;
use metar_taf_parser::{Language, describe_metar, parse_metar};

#[test]
fn parse_qnh_hpa_group() {
    let p = parse_pressure("Q1015").expect("pressure should parse");

    match p {
        Pressure::QnhHpa(v) => assert_eq!(v, 1015),
        _ => panic!("expected QnhHpa"),
    }
}

#[test]
fn parse_altimeter_inhg_group() {
    let p = parse_pressure("A2992").expect("pressure should parse");

    match p {
        Pressure::AltimeterInHg(v) => assert!((v - 29.92).abs() < 0.001),
        _ => panic!("expected AltimeterInHg"),
    }
}

#[test]
fn reject_malformed_pressure_groups() {
    assert!(parse_pressure("Q101").is_none());
    assert!(parse_pressure("Q10155").is_none());
    assert!(parse_pressure("Q10A5").is_none());
    assert!(parse_pressure("A299").is_none());
    assert!(parse_pressure("A29920").is_none());
    assert!(parse_pressure("A29X2").is_none());
}

#[test]
fn metar_with_malformed_pressure_keeps_pressure_empty() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q10A5")
        .expect("METAR should still parse");

    assert!(metar.pressure.is_none());
}

#[test]
fn parse_manual_example_qnh() {
    // Q1001 — manual example
    let p = parse_pressure("Q1001").unwrap();
    assert!(matches!(p, Pressure::QnhHpa(1001)));
}

#[test]
fn parse_manual_example_inhg() {
    // A2994 — manual example: 29.94 inHg
    let p = parse_pressure("A2994").unwrap();
    match p {
        Pressure::AltimeterInHg(v) => assert!((v - 29.94).abs() < 0.001, "got {}", v),
        _ => panic!("expected AltimeterInHg"),
    }
}

#[test]
fn parse_low_and_high_qnh() {
    assert!(matches!(
        parse_pressure("Q0850").unwrap(),
        Pressure::QnhHpa(850)
    ));
    assert!(matches!(
        parse_pressure("Q1050").unwrap(),
        Pressure::QnhHpa(1050)
    ));
}

#[test]
fn describe_pressure_qnh() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1001").unwrap();
    let desc = describe_metar(&m, Language::En);
    let pressure = desc.pressure.unwrap();
    assert!(pressure.contains("1001 hPa"), "{}", pressure);
}

#[test]
fn describe_pressure_inhg() {
    let m = parse_metar("KJFK 121250Z 18010KT 9999 FEW030 18/12 A2994").unwrap();
    let desc = describe_metar(&m, Language::En);
    let pressure = desc.pressure.unwrap();
    assert!(pressure.contains("29.94 inHg"), "{}", pressure);
}
