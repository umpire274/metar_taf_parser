use metar_taf_parser::parse_metar;

#[test]
fn parse_valid_metar_time_group() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    let time = metar.time.expect("time should parse");
    assert_eq!(time.day, 12);
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 50);
}

#[test]
fn reject_invalid_metar_time_ranges() {
    let metar = parse_metar("LIRF 322460Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    assert!(metar.time.is_none());
}

#[test]
fn reject_non_numeric_metar_time() {
    let metar = parse_metar("LIRF 12AB50Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    assert!(metar.time.is_none());
}
