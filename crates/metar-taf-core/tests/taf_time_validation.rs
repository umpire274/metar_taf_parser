use metar_taf_core::parse_taf;

#[test]
fn parses_valid_taf_header_time_and_validity() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT CAVOK").expect("TAF should parse");
    let issued_at = taf.issued_at.expect("issue time should be parsed");
    let validity = taf.validity.expect("validity should be parsed");

    assert_eq!(issued_at.day, 12);
    assert_eq!(issued_at.hour, 11);
    assert_eq!(issued_at.minute, 0);
    assert_eq!(validity.from_day, 12);
    assert_eq!(validity.from_hour, 12);
    assert_eq!(validity.to_day, 13);
    assert_eq!(validity.to_hour, 18);
}

#[test]
fn rejects_invalid_taf_issue_time_ranges() {
    assert!(parse_taf("TAF LIRF 001100Z 1212/1318 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 122460Z 1212/1318 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 122359Z 1212/1318 18010KT CAVOK").is_ok());
}

#[test]
fn rejects_invalid_taf_validity_format_and_ranges() {
    assert!(parse_taf("TAF LIRF 121100Z 122/1318 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 121100Z 0012/1318 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 121100Z 1230/1318 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 121100Z 1212/1325 18010KT CAVOK").is_err());
    assert!(parse_taf("TAF LIRF 121100Z 1212/1324 18010KT CAVOK").is_ok());
}
