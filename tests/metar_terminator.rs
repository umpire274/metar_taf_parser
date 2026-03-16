use metar_taf_parser::metar::models::pressure::Pressure;
use metar_taf_parser::parse_metar;

#[test]
fn metar_trailing_equals_keeps_last_group_parseable() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG=";
    let parsed = parse_metar(metar).expect("METAR should parse");

    assert!(parsed.trend.is_some());

    match parsed.pressure {
        Some(Pressure::QnhHpa(v)) => assert_eq!(v, 1014),
        _ => panic!("expected QNH pressure"),
    }
}
