use metar_taf_core::parse_metar;

#[test]
fn parse_rmk_basic() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 RMK QFE746/0995";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.rmk.as_deref(), Some("QFE746/0995"));
}

#[test]
fn parse_rmk_multiple_tokens() {
    let metar = "METAR XXXX 181200Z 18005KT 9999 RMK TEST ONE TWO 123/456";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.rmk.as_deref(), Some("TEST ONE TWO 123/456"));
}
