use metar_taf_parser::metar::models::wind::WindVariation;
use metar_taf_parser::parse_metar;

#[test]
fn wind_variation_parsed() {
    let m = parse_metar("LIRF 121250Z 18010KT 180V240 9999 FEW030 18/12 Q1015").unwrap();
    let variation = m.wind.as_ref().unwrap().variation.as_ref().unwrap();
    assert_eq!(variation.min, 180);
    assert_eq!(variation.max, 240);
}

#[test]
fn wind_variation_absent_when_not_present() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(m.wind.as_ref().unwrap().variation.is_none());
}

#[test]
fn wind_variation_with_vrb() {
    // VRB wind may also carry a variation group
    let m = parse_metar("LIRF 121250Z VRB05KT 010V080 9999 FEW030 18/12 Q1015").unwrap();
    let wind = m.wind.as_ref().unwrap();
    assert!(wind.direction.is_none()); // VRB
    let v = wind.variation.as_ref().unwrap();
    assert_eq!(v.min, 10);
    assert_eq!(v.max, 80);
}

#[test]
fn wind_variation_with_gust() {
    let m = parse_metar("LIRF 121250Z 20015G28KT 170V240 9999 FEW030 18/12 Q1015").unwrap();
    let wind = m.wind.as_ref().unwrap();
    assert_eq!(wind.gust, Some(28));
    let v = wind.variation.as_ref().unwrap();
    assert_eq!(v.min, 170);
    assert_eq!(v.max, 240);
}

#[test]
fn invalid_wind_variation_not_parsed() {
    // "18V240" has wrong format (2 digits before V) → should not be parsed as variation
    let m = parse_metar("LIRF 121250Z 18010KT 18V240 9999 FEW030 18/12 Q1015").unwrap();
    // The token "18V240" is 6 chars, not 7 → goes to unparsed
    assert!(m.wind.as_ref().unwrap().variation.is_none());
}

#[test]
fn wind_variation_unit_struct() {
    let v = WindVariation { min: 100, max: 200 };
    assert_eq!(v.min, 100);
    assert_eq!(v.max, 200);
}
