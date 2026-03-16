use metar_taf_core::common::report_modifier::ReportModifier;
use metar_taf_core::parse_taf;

#[test]
fn parse_taf_amd() {
    let t = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    assert_eq!(t.modifier, ReportModifier::Amendment);
    assert_eq!(t.station, "EDDF");
}

#[test]
fn parse_taf_cor() {
    let t = parse_taf("TAF COR LIRF 121100Z 1212/1318 18010KT CAVOK").unwrap();
    assert_eq!(t.modifier, ReportModifier::Correction);
    assert_eq!(t.station, "LIRF");
}

#[test]
fn parse_taf_without_taf_prefix_with_cor() {
    let t = parse_taf("COR EGLL 121100Z 1212/1318 22012KT 9999 SCT020").unwrap();
    assert_eq!(t.modifier, ReportModifier::Correction);
    assert_eq!(t.station, "EGLL");
}

#[test]
fn parse_taf_nil() {
    let t = parse_taf("TAF NIL").unwrap();
    assert_eq!(t.modifier, ReportModifier::Nil);
}
