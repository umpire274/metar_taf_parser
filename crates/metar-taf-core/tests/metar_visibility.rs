use metar_taf_core::metar::models::visibility::{Visibility, VisibilityDirection};
use metar_taf_core::parse_metar;

#[test]
fn metar_visibility_with_direction() {
    let input = "LIRF 121250Z 18010KT 5000 2000SW FEW030 18/12 Q1015";

    let metar = parse_metar(input).expect("METAR should parse");

    let visibility = metar.visibility.expect("visibility missing");

    match visibility {
        Visibility::WithMinimum {
            prevailing,
            minimum,
            direction,
        } => {
            assert_eq!(prevailing, 5000);
            assert_eq!(minimum, 2000);
            assert_eq!(direction, VisibilityDirection::SW);
        }
        _ => panic!("unexpected visibility variant"),
    }
}
