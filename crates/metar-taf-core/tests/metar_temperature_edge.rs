use metar_taf_core::parse_metar;

#[test]
fn parse_positive_temperature() {
    let metar = "METAR LIRF 181200Z 18005KT 9999 SCT020 15/10 Q1015";
    let p = parse_metar(metar).unwrap();
    let t = p.temperature.expect("temperature missing");
    assert_eq!(t.temperature, 15);
    assert_eq!(t.dew_point, 10);
}

#[test]
fn parse_negative_temperature() {
    let metar = "METAR UOOO 181400Z 08002MPS CAVOK M25/M28 Q1014";
    let p = parse_metar(metar).unwrap();
    let t = p.temperature.expect("temperature missing");
    assert_eq!(t.temperature, -25);
    assert_eq!(t.dew_point, -28);
}
