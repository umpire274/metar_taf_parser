use metar_taf_core::metar::models::cloud::CloudAmount;
use metar_taf_core::parse_metar;

#[test]
fn metar_vertical_visibility() {
    let input = "LIMC 121250Z 00000KT 0800 VV003 10/09 Q1009";

    let metar = parse_metar(input).expect("METAR should parse");

    assert_eq!(metar.clouds.len(), 1);

    let cloud = &metar.clouds[0];
    assert!(matches!(cloud.amount, CloudAmount::VV));
    assert_eq!(cloud.altitude_ft, Some(300));
}
