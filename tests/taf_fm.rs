use metar_taf_parser::parse_taf;

#[test]
fn taf_with_fm_groups() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
FM121800 20012KT 8000 SCT040
FM130600 15008KT 9999 BKN020";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 3);

    // Base forecast
    assert!(taf.forecasts[0].from.is_none());

    // First FM
    let fm1 = &taf.forecasts[1];
    assert_eq!(fm1.from, Some((12, 18, 0)));
    assert_eq!(fm1.wind.as_ref().unwrap().speed, 12);

    // Second FM
    let fm2 = &taf.forecasts[2];
    assert_eq!(fm2.from, Some((13, 6, 0)));
    assert_eq!(fm2.wind.as_ref().unwrap().speed, 8);
}

#[test]
fn taf_with_invalid_fm_time_does_not_start_new_forecast() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
FM129960 20012KT 8000 SCT040";

    let taf = parse_taf(input).expect("TAF should parse");

    // invalid FM time should be ignored; wind/clouds still parsed in base
    assert_eq!(taf.forecasts.len(), 1);
    let base = &taf.forecasts[0];
    assert_eq!(base.wind.as_ref().unwrap().speed, 12);
    assert!(!base.clouds.is_empty());
}
