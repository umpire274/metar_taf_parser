use metar_taf_core::common::report_modifier::ReportModifier;
use metar_taf_core::parse_metar;

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
