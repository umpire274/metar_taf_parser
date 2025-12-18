use metar_taf_core::metar::models::wind::WindUnit;
use metar_taf_core::parse_metar;

#[test]
fn metar_variable_wind_with_gust() {
    let input = "LIMC 121250Z VRB03G15KT 9999 SCT020 15/10 Q1018";

    let metar = parse_metar(input).expect("METAR should parse");

    let wind = metar.wind.expect("wind missing");

    assert_eq!(wind.direction, None); // VRB
    assert_eq!(wind.speed, 3);
    assert_eq!(wind.gust, Some(15));
}

#[test]
fn parse_wind_in_mps() {
    let metar = "METAR XXXX 181200Z 08002MPS CAVOK 10/05 Q1015";
    let parsed = parse_metar(metar).unwrap();
    let wind = parsed.wind.unwrap();

    assert_eq!(wind.direction, Some(80));
    assert_eq!(wind.speed, 2);
    assert_eq!(wind.unit, WindUnit::MPS);
}
