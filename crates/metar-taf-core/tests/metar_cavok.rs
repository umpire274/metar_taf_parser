use metar_taf_core::metar::models::visibility::Visibility;
use metar_taf_core::parse_metar;

#[test]
fn parse_metar_with_cavok() {
    let metar = "METAR XXXX 181200Z 00000KT CAVOK 15/10 Q1015";

    let parsed = parse_metar(metar).expect("METAR should parse");

    assert_eq!(parsed.visibility, Some(Visibility::CAVOK));
    assert!(parsed.clouds.is_empty());
}
