//! Integration tests for TAF Gruppo 4 — visibilità.
//!
//! Casi previsti dalla specifica:
//! - `dddd` (4 cifre, metri): visibilità prevalente prevista; se la visibilità non
//!   è uniforme nelle direzioni, indica la visibilità minima.
//! - `9999`: visibilità prevista ≥ 10 km.
//! - `CAVOK`: sostituisce i gruppi visibilità, fenomeni e nubi quando si prevedono
//!   visibilità ≥ 10 km, assenza di nubi operativamente significative e assenza di
//!   fenomeni meteorologici significativi.

use metar_taf_parser::metar::models::visibility::Visibility;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ---------------------------------------------------------------------------
// Visibilità in metri (4 cifre)
// ---------------------------------------------------------------------------

#[test]
fn visibility_8000m() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 8000 SCT020").unwrap();
    match t.forecasts[0].visibility.as_ref().unwrap() {
        Visibility::Single { prevailing, qualifier, ndv } => {
            assert_eq!(*prevailing, 8000);
            assert!(qualifier.is_none());
            assert!(!ndv);
        }
        _ => panic!("expected Single visibility"),
    }
}

#[test]
fn visibility_5000m() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 BKN010").unwrap();
    match t.forecasts[0].visibility.as_ref().unwrap() {
        Visibility::Single { prevailing, .. } => assert_eq!(*prevailing, 5000),
        _ => panic!("expected Single visibility"),
    }
}

#[test]
fn visibility_minimum_0000m() {
    // 0000 — visibilità nulla (nebbia fitta)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0000 FG OVC001").unwrap();
    match t.forecasts[0].visibility.as_ref().unwrap() {
        Visibility::Single { prevailing, .. } => assert_eq!(*prevailing, 0),
        _ => panic!("expected Single visibility"),
    }
}

#[test]
fn visibility_9999_means_10km_or_more() {
    // 9999 → visibilità ≥ 10 km
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    match t.forecasts[0].visibility.as_ref().unwrap() {
        Visibility::Single { prevailing, .. } => assert_eq!(*prevailing, 9999),
        _ => panic!("expected Single visibility"),
    }
}

// ---------------------------------------------------------------------------
// CAVOK
// ---------------------------------------------------------------------------

#[test]
fn cavok_sets_visibility_variant() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT CAVOK").unwrap();
    assert!(matches!(
        t.forecasts[0].visibility.as_ref().unwrap(),
        Visibility::CAVOK
    ));
}

#[test]
fn cavok_implies_no_clouds() {
    // Quando CAVOK è presente non devono esserci nubi nel blocco
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT CAVOK").unwrap();
    assert!(t.forecasts[0].clouds.is_empty());
}

#[test]
fn cavok_implies_no_weather() {
    // Quando CAVOK è presente non devono esserci fenomeni nel blocco
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT CAVOK").unwrap();
    assert!(t.forecasts[0].weather.is_empty());
}

#[test]
fn cavok_clears_clouds_parsed_before_it() {
    // Se per qualsiasi motivo nubi sono state aggiunte prima di CAVOK nello stesso
    // blocco, CAVOK deve rimuoverle (comportamento difensivo, TAF malformato).
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT SCT020 CAVOK").unwrap();
    assert!(
        t.forecasts[0].clouds.is_empty(),
        "CAVOK deve azzerare le nubi precedenti nello stesso blocco"
    );
}

#[test]
fn cavok_clears_weather_parsed_before_it() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT -RA CAVOK").unwrap();
    assert!(
        t.forecasts[0].weather.is_empty(),
        "CAVOK deve azzerare i fenomeni precedenti nello stesso blocco"
    );
}

#[test]
fn cavok_in_fm_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 5000 BKN010 FM251800 18005KT CAVOK",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 2);
    assert!(matches!(
        t.forecasts[1].visibility.as_ref().unwrap(),
        Visibility::CAVOK
    ));
    assert!(t.forecasts[1].clouds.is_empty());
}

#[test]
fn cavok_in_becmg_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 5000 BKN010 BECMG 2516/2518 CAVOK",
    )
    .unwrap();
    let becmg = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::BECMG))
        .unwrap();
    assert!(matches!(becmg.visibility.as_ref().unwrap(), Visibility::CAVOK));
}

#[test]
fn cavok_in_tempo_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 8000 SCT020 TEMPO 2514/2516 CAVOK",
    )
    .unwrap();
    let tempo = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::TEMPO))
        .unwrap();
    assert!(matches!(tempo.visibility.as_ref().unwrap(), Visibility::CAVOK));
    assert!(tempo.clouds.is_empty());
}

// ---------------------------------------------------------------------------
// Visibilità assente
// ---------------------------------------------------------------------------

#[test]
fn visibility_absent_is_none() {
    // Un blocco TEMPO che non modifica la visibilità non deve avere il campo
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2514/2516 02015G25KT",
    )
    .unwrap();
    let tempo = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::TEMPO))
        .unwrap();
    assert!(tempo.visibility.is_none());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_visibility_metres() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 8000 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let vis = desc.forecasts[0].visibility.as_ref().unwrap();
    assert!(vis.contains("8000"), "expected metres value in: {vis}");
}

#[test]
fn describe_visibility_9999() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let vis = desc.forecasts[0].visibility.as_ref().unwrap();
    assert!(
        vis.contains("10 km") || vis.contains("greater"),
        "expected >10km description in: {vis}"
    );
}

#[test]
fn describe_visibility_cavok() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT CAVOK").unwrap();
    let desc = describe_taf(&t, Language::En);
    let vis = desc.forecasts[0].visibility.as_ref().unwrap();
    assert!(vis.contains("CAVOK"), "expected CAVOK in: {vis}");
}

