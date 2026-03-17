use metar_taf_parser::metar::models::cloud::{CloudAmount, CloudType};
use metar_taf_parser::{describe_metar, parse_metar, Language};

// ---------------------------------------------------------------------------
// Standard layers
// ---------------------------------------------------------------------------

#[test]
fn metar_multiple_cloud_layers() {
    let m = parse_metar(
        "LIRF 121250Z 18010KT 9999 FEW007 BKN014CB BKN017 18/12 Q1015",
    )
    .unwrap();
    // Manual example: FEW007 BKN014CB BKN017
    assert_eq!(m.clouds.len(), 3);
    assert_eq!(m.clouds[0].amount, CloudAmount::FEW);
    assert_eq!(m.clouds[0].altitude_ft, Some(700));
    assert_eq!(m.clouds[1].amount, CloudAmount::BKN);
    assert_eq!(m.clouds[1].altitude_ft, Some(1400));
    assert_eq!(m.clouds[1].cloud_type, Some(CloudType::CB));
    assert_eq!(m.clouds[2].amount, CloudAmount::BKN);
    assert_eq!(m.clouds[2].altitude_ft, Some(1700));
    assert_eq!(m.clouds[2].cloud_type, None);
}

#[test]
fn metar_sct_with_tcu() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 SCT050TCU 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].cloud_type, Some(CloudType::TCU));
}

#[test]
fn metar_ovc_unknown_height() {
    // OVC/// — overcast, height not measurable
    let m = parse_metar("LIRF 121250Z 18010KT 9999 OVC/// 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::OVC);
    assert_eq!(m.clouds[0].altitude_ft, None);
}

// ---------------------------------------------------------------------------
// BKN013/// — height known, type not measurable
// ---------------------------------------------------------------------------

#[test]
fn metar_cloud_known_height_unmeasurable_type() {
    // BKN013/// — broken at 1300 ft, cloud type cannot be measured
    let m = parse_metar("LIRF 121250Z 18010KT 9999 BKN013/// 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::BKN);
    assert_eq!(m.clouds[0].altitude_ft, Some(1300));
    assert_eq!(m.clouds[0].cloud_type, None); // not measurable → None
}

// ---------------------------------------------------------------------------
// Vertical visibility
// ---------------------------------------------------------------------------

#[test]
fn metar_vertical_visibility() {
    // VV001 — vertical view of 100 ft
    let m = parse_metar("LIRF 121250Z 18010KT 9999 VV001 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::VV);
    assert_eq!(m.clouds[0].altitude_ft, Some(100));
}

#[test]
fn metar_vertical_visibility_unknown_height() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 VV/// 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::VV);
    assert_eq!(m.clouds[0].altitude_ft, None);
}

// ---------------------------------------------------------------------------
// Special codes
// ---------------------------------------------------------------------------

#[test]
fn metar_nsc() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 NSC 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::NSC);
}

#[test]
fn metar_skc() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 SKC 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::SKC);
}

#[test]
fn metar_ncd() {
    // NCD — no clouds detected (automatic station)
    let m = parse_metar("LIRF 121250Z 18010KT 9999 NCD 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::NCD);
    assert_eq!(m.clouds[0].altitude_ft, None);
}

#[test]
fn metar_clr() {
    // CLR — no cloud below 12,000 ft (automatic station)
    let m = parse_metar("LIRF 121250Z 18010KT 9999 CLR 18/12 Q1015").unwrap();
    assert_eq!(m.clouds[0].amount, CloudAmount::CLR);
    assert_eq!(m.clouds[0].altitude_ft, None);
}

// ---------------------------------------------------------------------------
// Rejection
// ---------------------------------------------------------------------------

#[test]
fn reject_invalid_cloud_suffix() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 SCT050ABC 18/12 Q1015").unwrap();
    assert!(m.clouds.is_empty());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_cloud_few_with_altitude() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW007 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert!(desc.clouds[0].contains("700 ft"), "{}", desc.clouds[0]);
}

#[test]
fn describe_cloud_bkn_cb() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 BKN014CB 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let cloud = &desc.clouds[0];
    assert!(cloud.contains("broken"), "{}", cloud);
    assert!(cloud.contains("1400 ft"), "{}", cloud);
    assert!(cloud.contains("cumulonimbus"), "{}", cloud);
}

#[test]
fn describe_cloud_ncd() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 NCD 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.clouds[0], "no clouds detected");
}

#[test]
fn describe_cloud_clr() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 CLR 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert_eq!(desc.clouds[0], "no cloud below 12,000 ft");
}

#[test]
fn describe_cloud_vv() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 VV001 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert!(desc.clouds[0].contains("100 ft"), "{}", desc.clouds[0]);
}

#[test]
fn describe_cloud_ovc_unknown_height() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 OVC/// 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert!(desc.clouds[0].contains("not available"), "{}", desc.clouds[0]);
}
