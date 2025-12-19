use metar_taf_core::parse_metar;

#[test]
fn parse_nosing() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert!(parsed.trend.is_some());
}
