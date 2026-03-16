use metar_taf_parser::metar::models::rvr::{RvrQualifier, RvrTendency, RvrUnit};
use metar_taf_parser::parse_metar;

#[test]
fn parse_basic_rvr_group() {
    let metar = parse_metar("METAR LIRF 121250Z 18010KT 9999 R16/0800 FEW030 18/12 Q1015")
        .expect("METAR should parse");

    assert_eq!(metar.runway_visual_range.len(), 1);
    let rvr = &metar.runway_visual_range[0];
    assert_eq!(rvr.runway_designator, "16");
    assert_eq!(rvr.min.value, 800);
    assert!(rvr.min.qualifier.is_none());
    assert!(rvr.max.is_none());
    assert_eq!(rvr.unit, RvrUnit::Meters);
}

#[test]
fn parse_variable_qualified_rvr_with_tendency() {
    let metar = parse_metar("METAR LIRF 121250Z 18010KT 9999 R27/P1500V2000U FEW030 18/12 Q1015")
        .expect("METAR should parse");

    let rvr = &metar.runway_visual_range[0];
    assert_eq!(rvr.runway_designator, "27");
    assert_eq!(rvr.min.value, 1500);
    assert!(matches!(rvr.min.qualifier, Some(RvrQualifier::Above)));
    let max = rvr.max.as_ref().expect("max range should exist");
    assert_eq!(max.value, 2000);
    assert!(max.qualifier.is_none());
    assert!(matches!(rvr.tendency, Some(RvrTendency::Upward)));
}

#[test]
fn parse_feet_rvr_and_keep_runway_state_separate() {
    let metar = parse_metar(
        "METAR UOOO 191400Z 00000MPS CAVOK R19/450235 R25L/1200FTN M28/M31 Q1020 NOSIG",
    )
    .expect("METAR should parse");

    assert_eq!(metar.runway_state.len(), 1);
    assert_eq!(metar.runway_visual_range.len(), 1);

    let rvr = &metar.runway_visual_range[0];
    assert_eq!(rvr.runway_designator, "25L");
    assert_eq!(rvr.min.value, 1200);
    assert_eq!(rvr.unit, RvrUnit::Feet);
    assert!(matches!(rvr.tendency, Some(RvrTendency::NoChange)));
}

#[test]
fn malformed_rvr_is_not_parsed_as_rvr() {
    let metar = parse_metar("METAR LIRF 121250Z 18010KT 9999 R16/08A FEW030 18/12 Q1015")
        .expect("METAR should parse");

    assert!(metar.runway_visual_range.is_empty());
    assert!(metar.unparsed_groups.iter().any(|g| g == "R16/08A"));
}
