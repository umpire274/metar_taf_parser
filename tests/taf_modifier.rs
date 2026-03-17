use metar_taf_parser::common::report_modifier::ReportModifier;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ---------------------------------------------------------------------------
// Parsing — prefix and modifier combinations
// ---------------------------------------------------------------------------

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

#[test]
fn parse_taf_without_prefix_normal() {
    // TAF body without leading TAF token — station comes first.
    let t = parse_taf("LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.modifier, ReportModifier::Normal);
    assert_eq!(t.station, "LIRF");
}

#[test]
fn parse_taf_amd_nil() {
    // TAF AMD NIL — amended TAF with no data. Must not treat NIL as station.
    let t = parse_taf("TAF AMD NIL").unwrap();
    assert_eq!(t.modifier, ReportModifier::Nil);
    assert_eq!(t.station, "");
    assert!(t.issued_at.is_none());
    assert!(t.validity.is_none());
    assert!(t.forecasts.is_empty());
}

#[test]
fn parse_taf_cor_nil() {
    // TAF COR NIL — corrected TAF with no data. Must not treat NIL as station.
    let t = parse_taf("TAF COR NIL").unwrap();
    assert_eq!(t.modifier, ReportModifier::Nil);
    assert_eq!(t.station, "");
    assert!(t.forecasts.is_empty());
}

// ---------------------------------------------------------------------------
// Describe — modifier labels
// ---------------------------------------------------------------------------

#[test]
fn describe_taf_modifier_amd() {
    let t = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("amended forecast"));
}

#[test]
fn describe_taf_modifier_cor() {
    let t = parse_taf("TAF COR LIRF 121100Z 1212/1318 18010KT CAVOK").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("corrected report"));
}

#[test]
fn describe_taf_modifier_nil() {
    let t = parse_taf("TAF NIL").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("no data available"));
}

#[test]
fn describe_taf_modifier_normal_is_absent() {
    // Normal TAF produces no modifier entry.
    let t = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert!(desc.modifier.is_none());
}

