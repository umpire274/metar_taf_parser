//! Integration tests for the unified `parse` / `parse_strict` entry-points.

use metar_taf_parser::metar::models::report_type::MetarReportType;
use metar_taf_parser::{ParseError, ParsedReport, parse, parse_strict};

// ---------------------------------------------------------------------------
// Tolerant `parse` — dispatch by leading token
// ---------------------------------------------------------------------------

#[test]
fn parse_metar_prefix_returns_metar_variant() {
    let report = parse("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(matches!(report, ParsedReport::Metar(_)));
}

#[test]
fn parse_speci_prefix_returns_metar_variant_with_speci_type() {
    let report = parse("SPECI LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    match report {
        ParsedReport::Metar(m) => assert_eq!(m.report_type, MetarReportType::Speci),
        ParsedReport::Taf(_) => panic!("expected Metar variant"),
    }
}

#[test]
fn parse_taf_prefix_returns_taf_variant() {
    let report = parse("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert!(matches!(report, ParsedReport::Taf(_)));
}

#[test]
fn parse_no_prefix_defaults_to_metar() {
    // Without a leading token, parse falls back to the METAR parser.
    let report = parse("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(matches!(report, ParsedReport::Metar(_)));
}

#[test]
fn parse_metar_station_is_correct() {
    let report = parse("METAR EGLL 121250Z 24015KT 9999 SCT020 16/08 Q1008").unwrap();
    if let ParsedReport::Metar(m) = report {
        assert_eq!(m.station, "EGLL");
    } else {
        panic!("expected Metar variant");
    }
}

#[test]
fn parse_taf_station_is_correct() {
    let report = parse("TAF EGLL 121100Z 1212/1318 24015KT 9999 SCT020").unwrap();
    if let ParsedReport::Taf(t) = report {
        assert_eq!(t.station, "EGLL");
    } else {
        panic!("expected Taf variant");
    }
}

#[test]
fn parse_tolerates_unknown_groups_in_metar() {
    // Tolerant mode: unknown tokens end up in unparsed_groups, no error.
    let report = parse("METAR LIRF 121250Z 18010KT 9999 FEW030 UNKNOWN_TOKEN 18/12 Q1015").unwrap();
    if let ParsedReport::Metar(m) = report {
        assert!(m.unparsed_groups.contains(&"UNKNOWN_TOKEN".to_string()));
    } else {
        panic!("expected Metar variant");
    }
}

// ---------------------------------------------------------------------------
// Strict `parse_strict` — requires explicit prefix
// ---------------------------------------------------------------------------

#[test]
fn parse_strict_metar_prefix_ok() {
    let report = parse_strict("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(matches!(report, ParsedReport::Metar(_)));
}

#[test]
fn parse_strict_speci_prefix_ok() {
    let report = parse_strict("SPECI LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(matches!(report, ParsedReport::Metar(_)));
}

#[test]
fn parse_strict_taf_prefix_ok() {
    let report = parse_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert!(matches!(report, ParsedReport::Taf(_)));
}

#[test]
fn parse_strict_no_prefix_returns_unknown_report_type() {
    let err = parse_strict("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap_err();
    assert!(
        matches!(err, ParseError::UnknownReportType),
        "expected UnknownReportType, got: {}",
        err
    );
}

#[test]
fn parse_strict_rejects_unknown_metar_group() {
    let err = parse_strict("METAR LIRF 121250Z 18010KT 9999 FEW030 UNKNOWN_TOKEN 18/12 Q1015")
        .unwrap_err();
    assert!(
        matches!(err, ParseError::Metar(_)),
        "expected Metar error, got: {}",
        err
    );
}

#[test]
fn parse_strict_rejects_unknown_taf_group() {
    let err =
        parse_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 UNKNOWN_TOKEN").unwrap_err();
    assert!(
        matches!(err, ParseError::Taf(_)),
        "expected Taf error, got: {}",
        err
    );
}
