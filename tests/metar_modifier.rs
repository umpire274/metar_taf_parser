use metar_taf_parser::common::report_modifier::ReportModifier;
use metar_taf_parser::metar::models::report_type::MetarReportType;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// Basic modifier parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_metar_cor() {
    let m = parse_metar("METAR COR LIRF 191350Z 18005KT CAVOK 20/12 Q1016").unwrap();
    assert_eq!(m.modifier, ReportModifier::Correction);
    assert_eq!(m.station, "LIRF");
}

#[test]
fn parse_metar_auto() {
    let m = parse_metar("METAR LIRF 191350Z AUTO 18005KT CAVOK 20/12 Q1016").unwrap();
    assert_eq!(m.modifier, ReportModifier::Auto);
}

#[test]
fn parse_metar_nil() {
    let m = parse_metar("METAR NIL").unwrap();
    assert_eq!(m.modifier, ReportModifier::Nil);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn parse_metar_cor_nil() {
    // COR followed by NIL instead of a station — whole report is Nil
    let m = parse_metar("METAR COR NIL").unwrap();
    assert_eq!(m.modifier, ReportModifier::Nil);
}

#[test]
fn parse_metar_auto_before_time() {
    // Some feeds place AUTO before the time group — parser should tolerate this
    let m = parse_metar("LIRF AUTO 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.modifier, ReportModifier::Auto);
    assert_eq!(m.station, "LIRF");
}

#[test]
fn parse_metar_nil_inline() {
    // NIL appearing after the station in the token stream
    let m = parse_metar("METAR LIRF 121250Z NIL").unwrap();
    assert_eq!(m.modifier, ReportModifier::Nil);
}

#[test]
fn parse_metar_normal_has_no_modifier() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.modifier, ReportModifier::Normal);
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_modifier_auto() {
    let m = parse_metar("LIRF 121250Z AUTO 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("automated report"));
}

#[test]
fn describe_modifier_cor() {
    let m = parse_metar("METAR COR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("corrected report"));
}

#[test]
fn describe_modifier_nil() {
    let m = parse_metar("METAR NIL").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.modifier.as_deref(), Some("no data available"));
}

#[test]
fn describe_modifier_normal_is_absent() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    // Normal modifier produces no description entry
    assert!(desc.modifier.is_none());
}

// ---------------------------------------------------------------------------
// Report type (METAR / SPECI)
// ---------------------------------------------------------------------------

#[test]
fn parse_speci_prefix_sets_report_type() {
    let m = parse_metar("SPECI LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.report_type, MetarReportType::Speci);
    assert_eq!(m.station, "LIRF");
}

#[test]
fn parse_metar_prefix_sets_report_type_metar() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.report_type, MetarReportType::Metar);
}

#[test]
fn parse_no_prefix_defaults_to_metar_report_type() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.report_type, MetarReportType::Metar);
}

#[test]
fn parse_speci_with_cor_modifier() {
    let m = parse_metar("SPECI COR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert_eq!(m.report_type, MetarReportType::Speci);
    assert_eq!(m.modifier, ReportModifier::Correction);
    assert_eq!(m.station, "LIRF");
}

#[test]
fn parse_speci_nil() {
    let m = parse_metar("SPECI NIL").unwrap();
    assert_eq!(m.report_type, MetarReportType::Speci);
    assert_eq!(m.modifier, ReportModifier::Nil);
}

#[test]
fn describe_report_type_metar() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.report_type, "METAR");
}

#[test]
fn describe_report_type_speci() {
    let m = parse_metar("SPECI LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.report_type, "SPECI");
}

#[test]
fn describe_report_type_default_is_metar() {
    // No prefix → defaults to METAR
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.report_type, "METAR");
}
