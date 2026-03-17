use metar_taf_parser::metar::models::visibility::Visibility;
use metar_taf_parser::parse_taf;

#[test]
fn taf_visibility_statute_miles_single_token() {
    let input = "\
TAF KJFK 121100Z 1212/1318
18010KT 6SM FEW030";
    let taf = parse_taf(input).expect("TAF should parse");
    match taf.forecasts[0].visibility.as_ref().expect("visibility missing") {
        Visibility::Single { prevailing, .. } => assert_eq!(*prevailing, 9656),
        _ => panic!("unexpected visibility variant"),
    }
}

#[test]
fn taf_visibility_statute_miles_split_token() {
    let input = "\
TAF KJFK 121100Z 1212/1318
18010KT 1 1/2SM FEW030";
    let taf = parse_taf(input).expect("TAF should parse");
    match taf.forecasts[0].visibility.as_ref().expect("visibility missing") {
        Visibility::Single { prevailing, .. } => assert_eq!(*prevailing, 2414),
        _ => panic!("unexpected visibility variant"),
    }
}
