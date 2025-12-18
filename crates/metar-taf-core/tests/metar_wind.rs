use metar_taf_core::parse_metar;

#[test]
fn metar_variable_wind_with_gust() {
    let input = "LIMC 121250Z VRB03G15KT 9999 SCT020 15/10 Q1018";

    let metar = parse_metar(input).expect("METAR should parse");

    let wind = metar.wind.expect("wind missing");

    assert_eq!(wind.direction, None); // VRB
    assert_eq!(wind.speed_kt, 3);
    assert_eq!(wind.gust_kt, Some(15));
}
