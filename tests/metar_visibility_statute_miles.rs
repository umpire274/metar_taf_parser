use metar_taf_parser::metar::models::visibility::Visibility;
use metar_taf_parser::parse_metar;

#[test]
fn metar_visibility_statute_miles_single_token() {
    let input = "KJFK 121251Z 18010KT 10SM FEW020 25/17 A2992";

    let metar = parse_metar(input).expect("METAR should parse");

    let visibility = metar.visibility.expect("visibility missing");

    match visibility {
        Visibility::Single { prevailing, .. } => {
            assert_eq!(prevailing, 16093);
        }
        _ => panic!("unexpected visibility variant"),
    }
}

#[test]
fn metar_visibility_statute_miles_split_token() {
    let input = "KJFK 121251Z 18010KT 1 1/2SM FEW020 25/17 A2992";

    let metar = parse_metar(input).expect("METAR should parse");

    let visibility = metar.visibility.expect("visibility missing");

    match visibility {
        Visibility::Single { prevailing, .. } => {
            assert_eq!(prevailing, 2414);
        }
        _ => panic!("unexpected visibility variant"),
    }
}

#[test]
fn metar_visibility_statute_miles_prefixed_token() {
    let input = "KJFK 121251Z 18010KT P6SM FEW020 25/17 A2992";

    let metar = parse_metar(input).expect("METAR should parse");

    let visibility = metar.visibility.expect("visibility missing");

    match visibility {
        Visibility::Single { prevailing, .. } => {
            assert_eq!(prevailing, 9656);
        }
        _ => panic!("unexpected visibility variant"),
    }
}
