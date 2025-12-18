use metar_taf_core::metar::models::visibility::Visibility;
use metar_taf_core::parse_metar;

#[test]
fn metar_visibility_cavok() {
    let input = "LIRF 121250Z 18010KT CAVOK 18/12 Q1015";

    let metar = parse_metar(input).expect("METAR should parse");

    let visibility = metar.visibility.expect("visibility missing");

    assert!(matches!(visibility, Visibility::CAVOK));
}
