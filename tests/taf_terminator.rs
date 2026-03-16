use metar_taf_parser::parse_taf;

#[test]
fn taf_trailing_equals_keeps_last_group_parseable() {
    let taf = "TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030=";
    let parsed = parse_taf(taf).expect("TAF should parse");

    assert_eq!(parsed.forecasts.len(), 1);
    assert_eq!(parsed.forecasts[0].clouds.len(), 1);
}
