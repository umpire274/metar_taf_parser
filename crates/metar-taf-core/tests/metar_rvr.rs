use metar_taf_core::parse_metar;

#[test]
fn parse_basic_rvr() {
    let metar = "METAR LIRF 181200Z 18005KT 9999 R16/0800U SCT020 15/10 Q1015";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.rvr.len(), 1);

    let rvr = &parsed.rvr[0];
    assert_eq!(rvr.runway, "16");
    assert_eq!(rvr.min, 800);
    assert_eq!(rvr.max, None);
    assert!(rvr.trend.is_some());
}

#[test]
fn ignore_runway_state_group() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 R01/39//37 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    // No RVR expected
    assert!(parsed.rvr.is_empty());
}
