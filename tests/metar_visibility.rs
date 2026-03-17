use metar_taf_parser::metar::models::visibility::{
    Visibility, VisibilityDirection, VisibilityQualifier,
};
use metar_taf_parser::parse_metar;

#[test]
fn metar_visibility_with_direction() {
    let m = parse_metar("LIRF 121250Z 18010KT 5000 2000SW FEW030 18/12 Q1015").unwrap();
    match m.visibility.unwrap() {
        Visibility::WithMinimum {
            prevailing,
            minimum,
            direction,
        } => {
            assert_eq!(prevailing, 5000);
            assert_eq!(minimum, 2000);
            assert_eq!(direction, VisibilityDirection::SW);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn metar_visibility_metric_no_fields() {
    let m = parse_metar("LIRF 121250Z 18010KT 5000 FEW030 18/12 Q1015").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single {
            prevailing,
            qualifier,
            ndv,
        } => {
            assert_eq!(prevailing, 5000);
            assert!(qualifier.is_none());
            assert!(!ndv);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn metar_visibility_ndv() {
    let m = parse_metar("LIRF 121250Z 18010KT 5000NDV FEW030 18/12 Q1015").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single {
            prevailing, ndv, ..
        } => {
            assert_eq!(prevailing, 5000);
            assert!(ndv);
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn metar_visibility_9999() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single { prevailing, .. } => assert_eq!(prevailing, 9999),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn metar_visibility_low_zero_padded() {
    let m = parse_metar("LIRF 121250Z 18010KT 0800 FEW030 18/12 Q1015").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single { prevailing, .. } => assert_eq!(prevailing, 800),
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn metar_visibility_sm_above_qualifier() {
    let m = parse_metar("KJFK 121251Z 18010KT P6SM FEW020 25/17 A2992").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single {
            qualifier: Some(VisibilityQualifier::Above),
            ..
        } => {}
        other => panic!("expected Above qualifier, got: {:?}", other),
    }
}

#[test]
fn metar_visibility_sm_below_qualifier() {
    let m = parse_metar("KJFK 121251Z 18010KT M1/4SM FEW020 25/17 A2992").unwrap();
    match m.visibility.unwrap() {
        Visibility::Single {
            qualifier: Some(VisibilityQualifier::Below),
            ..
        } => {}
        other => panic!("expected Below qualifier, got: {:?}", other),
    }
}
