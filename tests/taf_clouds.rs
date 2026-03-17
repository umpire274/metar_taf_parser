//! Integration tests for TAF Gruppo 6 — nubi.
//!
//! Copertura nuvolosa:
//! - `FEW` 1–2 ottavi, `SCT` 3–4 ottavi, `BKN` 5–7 ottavi, `OVC` 8 ottavi
//! - Altitudine della base in centinaia di piedi AGL (es. `020` = 2000 ft)
//! - Tipo di nube opzionale: `CB` (cumulonembo) o `TCU` (cumulo torreggiante)
//! - `VV` visibilità verticale (cielo invisibile), altitudine in centinaia di piedi
//! - `NSC` No Significant Clouds (nessuna nube operativamente significativa)

use metar_taf_parser::metar::models::cloud::{CloudAmount, CloudType};
use metar_taf_parser::taf::models::forecast::TafForecastKind;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ---------------------------------------------------------------------------
// Copertura nuvolosa — tutti i codici
// ---------------------------------------------------------------------------

#[test]
fn cloud_few_coverage() {
    // FEW010 — 1-2 ottavi a 1000 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 FEW010").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::FEW);
    assert_eq!(c.altitude_ft, Some(1000));
    assert!(c.cloud_type.is_none());
}

#[test]
fn cloud_sct_coverage_and_altitude() {
    // SCT020 — 3-4 ottavi a 2000 ft (esempio dal manuale)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::SCT);
    assert_eq!(c.altitude_ft, Some(2000));
}

#[test]
fn cloud_bkn_coverage_and_altitude() {
    // BKN040 — 5-7 ottavi a 4000 ft (esempio dal manuale)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN040").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::BKN);
    assert_eq!(c.altitude_ft, Some(4000));
}

#[test]
fn cloud_ovc_coverage() {
    // OVC008 — 8 ottavi (cielo coperto) a 800 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 2000 OVC008").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::OVC);
    assert_eq!(c.altitude_ft, Some(800));
}

// ---------------------------------------------------------------------------
// Altitudine — calcolo corretto (centinaia di piedi)
// ---------------------------------------------------------------------------

#[test]
fn cloud_altitude_100ft() {
    // SCT001 = 100 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0300 SCT001").unwrap();
    assert_eq!(t.forecasts[0].clouds[0].altitude_ft, Some(100));
}

#[test]
fn cloud_altitude_10000ft() {
    // BKN100 = 10000 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN100").unwrap();
    assert_eq!(t.forecasts[0].clouds[0].altitude_ft, Some(10000));
}

#[test]
fn cloud_altitude_unknown() {
    // OVC/// — altitudine non disponibile
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 OVC///").unwrap();
    assert_eq!(t.forecasts[0].clouds[0].altitude_ft, None);
}

// ---------------------------------------------------------------------------
// Tipi di nube: CB e TCU
// ---------------------------------------------------------------------------

#[test]
fn cloud_type_cb_cumulonimbus() {
    // SCT020CB — nubi convettive significative
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 3000 TSRA SCT020CB").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::SCT);
    assert_eq!(c.altitude_ft, Some(2000));
    assert_eq!(c.cloud_type, Some(CloudType::CB));
}

#[test]
fn cloud_type_tcu_towering_cumulus() {
    // FEW015TCU
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 FEW015TCU").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.cloud_type, Some(CloudType::TCU));
    assert_eq!(c.altitude_ft, Some(1500));
}

#[test]
fn cloud_no_type_when_absent() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    assert!(t.forecasts[0].clouds[0].cloud_type.is_none());
}

// ---------------------------------------------------------------------------
// VV — visibilità verticale (cielo invisibile)
// ---------------------------------------------------------------------------

#[test]
fn cloud_vv_vertical_visibility() {
    // VV001 — visibilità verticale 100 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0100 FG VV001").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::VV);
    assert_eq!(c.altitude_ft, Some(100));
    assert!(c.cloud_type.is_none());
}

#[test]
fn cloud_vv_higher_value() {
    // VV010 = 1000 ft
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0500 FG VV010").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::VV);
    assert_eq!(c.altitude_ft, Some(1000));
}

#[test]
fn cloud_vv_unknown_altitude() {
    // VV/// — visibilità verticale non misurabile
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0100 FG VV///").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::VV);
    assert_eq!(c.altitude_ft, None);
}

// ---------------------------------------------------------------------------
// NSC — No Significant Clouds
// ---------------------------------------------------------------------------

#[test]
fn cloud_nsc_no_significant_clouds() {
    // NSC — nessuna nube operativamente significativa (senza CAVOK)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 NSC").unwrap();
    let c = &t.forecasts[0].clouds[0];
    assert_eq!(c.amount, CloudAmount::NSC);
    assert!(c.altitude_ft.is_none());
    assert!(c.cloud_type.is_none());
}

#[test]
fn cloud_nsc_in_fm_group() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 BKN010 FM251800 18005KT 9999 NSC")
        .unwrap();
    let fm = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::FM))
        .unwrap();
    assert_eq!(fm.clouds[0].amount, CloudAmount::NSC);
}

// ---------------------------------------------------------------------------
// Nubi multiple nello stesso blocco
// ---------------------------------------------------------------------------

#[test]
fn cloud_multiple_layers() {
    // FEW020 SCT040 BKN080 — tre strati
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 FEW020 SCT040 BKN080").unwrap();
    let clouds = &t.forecasts[0].clouds;
    assert_eq!(clouds.len(), 3);
    assert_eq!(clouds[0].amount, CloudAmount::FEW);
    assert_eq!(clouds[0].altitude_ft, Some(2000));
    assert_eq!(clouds[1].amount, CloudAmount::SCT);
    assert_eq!(clouds[1].altitude_ft, Some(4000));
    assert_eq!(clouds[2].amount, CloudAmount::BKN);
    assert_eq!(clouds[2].altitude_ft, Some(8000));
}

// ---------------------------------------------------------------------------
// Nubi nei gruppi di cambiamento
// ---------------------------------------------------------------------------

#[test]
fn cloud_in_fm_group() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251800 18005KT 9999 BKN010")
            .unwrap();
    let fm = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::FM))
        .unwrap();
    assert_eq!(fm.clouds[0].amount, CloudAmount::BKN);
    assert_eq!(fm.clouds[0].altitude_ft, Some(1000));
}

#[test]
fn cloud_in_tempo_group() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 FEW030 TEMPO 2514/2516 OVC010").unwrap();
    let tempo = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::TEMPO))
        .unwrap();
    assert_eq!(tempo.clouds[0].amount, CloudAmount::OVC);
    assert_eq!(tempo.clouds[0].altitude_ft, Some(1000));
}

#[test]
fn cloud_in_becmg_group() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 OVC015 BECMG 2516/2518 SCT030").unwrap();
    let becmg = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::BECMG))
        .unwrap();
    assert_eq!(becmg.clouds[0].amount, CloudAmount::SCT);
    assert_eq!(becmg.clouds[0].altitude_ft, Some(3000));
}

// ---------------------------------------------------------------------------
// CAVOK — nessuna nube
// ---------------------------------------------------------------------------

#[test]
fn cavok_produces_no_clouds() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT CAVOK").unwrap();
    assert!(t.forecasts[0].clouds.is_empty());
}

#[test]
fn cloud_absent_when_not_reported() {
    // Un blocco TEMPO che non riporta nubi deve avere il Vec vuoto
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT030 TEMPO 2514/2516 -RA").unwrap();
    let tempo = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::TEMPO))
        .unwrap();
    assert!(tempo.clouds.is_empty());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_cloud_sct020() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let cloud = &desc.forecasts[0].clouds[0];
    assert!(
        cloud.contains("scattered"),
        "expected 'scattered' in: {cloud}"
    );
    assert!(cloud.contains("2000 ft"), "expected altitude in: {cloud}");
}

#[test]
fn describe_cloud_bkn040() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN040").unwrap();
    let desc = describe_taf(&t, Language::En);
    let cloud = &desc.forecasts[0].clouds[0];
    assert!(cloud.contains("broken"), "expected 'broken' in: {cloud}");
    assert!(cloud.contains("4000 ft"), "expected altitude in: {cloud}");
}

#[test]
fn describe_cloud_cb() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 3000 TSRA SCT020CB").unwrap();
    let desc = describe_taf(&t, Language::En);
    let cloud = &desc.forecasts[0].clouds[0];
    assert!(
        cloud.contains("cumulonimbus"),
        "expected 'cumulonimbus' in: {cloud}"
    );
}

#[test]
fn describe_cloud_nsc() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 NSC").unwrap();
    let desc = describe_taf(&t, Language::En);
    let cloud = &desc.forecasts[0].clouds[0];
    assert!(
        cloud.contains("significant") || cloud.contains("NSC"),
        "expected NSC description in: {cloud}"
    );
}
