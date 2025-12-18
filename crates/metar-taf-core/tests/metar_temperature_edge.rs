use metar_taf_core::parse_metar;

#[test]
fn metar_temperature_negative_and_zero() {
    let input = "LIMC 121250Z 02005KT 9999 SCT020 M01/00 Q1020";

    let metar = parse_metar(input).expect("METAR should parse");

    let temp = metar.temperature.expect("temperature missing");

    assert_eq!(temp.air, -1);
    assert_eq!(temp.dew_point, 0);
}
