use metar_taf_core::parse_taf;

#[test]
fn taf_basic_header_and_times() {
    let input = "TAF LIRF 121100Z 1212/1318 18010KT CAVOK";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.station, "LIRF");
    assert_eq!(taf.issued_at.day, 12);
    assert_eq!(taf.validity.from_day, 12);
    assert_eq!(taf.validity.to_day, 13);
}
