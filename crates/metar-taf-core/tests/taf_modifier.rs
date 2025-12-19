use metar_taf_core::common::report_modifier::ReportModifier;
use metar_taf_core::parse_taf;

#[test]
fn parse_taf_amd() {
    let t = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    assert_eq!(t.modifier, ReportModifier::Amendment);
    assert_eq!(t.station, "EDDF");
}

#[test]
fn parse_taf_nil() {
    let t = parse_taf("TAF NIL").unwrap();
    assert_eq!(t.modifier, ReportModifier::Nil);
}
