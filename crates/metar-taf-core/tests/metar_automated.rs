use metar_taf_core::parse_metar;

#[test]
fn parse_metar_auto() {
    let metar = parse_metar("METAR LIRF 191350Z AUTO 18005KT CAVOK 20/12 Q1016").unwrap();

    assert!(metar.automated);
}

#[test]
fn parse_metar_manual() {
    let metar = parse_metar("METAR LIRF 191350Z 18005KT CAVOK 20/12 Q1016").unwrap();

    assert!(!metar.automated);
}
