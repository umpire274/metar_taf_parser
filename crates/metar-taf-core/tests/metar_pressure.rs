use metar_taf_core::metar::models::pressure::Pressure;
use metar_taf_core::metar::parser::pressure::parse_pressure;
use metar_taf_core::parse_metar;

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
