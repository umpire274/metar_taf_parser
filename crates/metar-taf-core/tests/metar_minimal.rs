use metar_taf_core::parse_metar;

#[test]
fn metar_minimal_valid() {
    let input = "LIRF 121250Z 00000KT CAVOK 20/10 Q1012";

    let metar = parse_metar(input).expect("METAR should parse");

    assert_eq!(metar.station, "LIRF");
    assert!(metar.wind.is_some());
    assert!(metar.visibility.is_some());
    assert!(metar.temperature.is_some());
    assert!(metar.pressure.is_some());
}
