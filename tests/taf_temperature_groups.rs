use metar_taf_parser::parse_taf;

#[test]
fn parse_taf_tx_tn_groups_in_base_forecast() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 TX18/1214Z TN08/1304Z")
        .expect("TAF should parse");

    let base = &taf.forecasts[0];
    let tx = base.max_temperature.as_ref().expect("TX should be parsed");
    let tn = base.min_temperature.as_ref().expect("TN should be parsed");

    assert_eq!(tx.value, 18);
    assert_eq!(tx.day, 12);
    assert_eq!(tx.hour, 14);

    assert_eq!(tn.value, 8);
    assert_eq!(tn.day, 13);
    assert_eq!(tn.hour, 4);
}

#[test]
fn parse_taf_negative_tn_group() {
    let taf = parse_taf("TAF UUEE 121100Z 1212/1318 18010KT 9999 FEW030 TNM05/1303Z")
        .expect("TAF should parse");

    let base = &taf.forecasts[0];
    let tn = base.min_temperature.as_ref().expect("TN should be parsed");
    assert_eq!(tn.value, -5);
    assert_eq!(tn.day, 13);
    assert_eq!(tn.hour, 3);
}

#[test]
fn malformed_taf_temperature_group_is_unparsed() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 TX1/1214Z")
        .expect("TAF should parse");

    assert!(taf.unparsed_groups.iter().any(|g| g == "TX1/1214Z"));
}
