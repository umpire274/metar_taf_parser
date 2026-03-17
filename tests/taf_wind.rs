//! Integration tests for TAF Gruppo 3 — vento al suolo.
//!
//! Formato: `dddffKT`, `dddffGggKT`, `00000KT`, `VRBffKT`, `dddffMPS`.
//! - `ddd` direzione vera arrotondata alla decina più vicina (010–360, o 000 per calma)
//! - `ff`  velocità media; `gg` velocità massima prevista (raffica, Gust)
//! - Unità: `KT` (nodi) o `MPS` (m/s)

use metar_taf_parser::metar::models::wind::WindUnit;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ---------------------------------------------------------------------------
// Vento standard
// ---------------------------------------------------------------------------

#[test]
fn wind_direction_and_speed() {
    // 02008KT — 020°, 8 nodi (esempio dal manuale)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(20));
    assert_eq!(w.speed, 8);
    assert_eq!(w.gust, None);
    assert_eq!(w.unit, WindUnit::KT);
}

#[test]
fn wind_with_gust() {
    // 02008G19KT — 020°, 8 nodi, raffica 19 nodi
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 02008G19KT 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(20));
    assert_eq!(w.speed, 8);
    assert_eq!(w.gust, Some(19));
    assert_eq!(w.unit, WindUnit::KT);
}

#[test]
fn wind_calm() {
    // 00000KT — intensità attesa < 1 kt: calma di vento
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(0));
    assert_eq!(w.speed, 0);
    assert_eq!(w.gust, None);
    assert!(!w.indeterminate);
}

#[test]
fn wind_variable_direction() {
    // VRB05KT — direzione variabile, 5 nodi
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 VRB05KT 9999 FEW020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, None); // VRB → no direction
    assert_eq!(w.speed, 5);
    assert_eq!(w.unit, WindUnit::KT);
}

#[test]
fn wind_variable_with_gust() {
    // VRB05G15KT
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 VRB05G15KT 9999 FEW020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, None);
    assert_eq!(w.speed, 5);
    assert_eq!(w.gust, Some(15));
}

#[test]
fn wind_unit_mps() {
    // 18005MPS — vento in m/s (stazioni russe/nord-europee)
    let t = parse_taf("TAF UUEE 251100Z 2512/2618 18005MPS 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(180));
    assert_eq!(w.speed, 5);
    assert_eq!(w.unit, WindUnit::MPS);
}

#[test]
fn wind_direction_360() {
    // 36015KT — nord (360°) è valido
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 36015KT 9999 SCT020").unwrap();
    assert_eq!(t.forecasts[0].wind.as_ref().unwrap().direction, Some(360));
}

#[test]
fn wind_high_speed() {
    // 31050G65KT — vento forte con raffica elevata
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 31050G65KT 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert_eq!(w.speed, 50);
    assert_eq!(w.gust, Some(65));
}

#[test]
fn wind_indeterminate() {
    // /////KT — direzione e velocità non disponibili
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 /////KT 9999 SCT020").unwrap();
    let w = t.forecasts[0].wind.as_ref().unwrap();
    assert!(w.indeterminate);
    assert_eq!(w.speed, 0);
    assert_eq!(w.direction, None);
}

// ---------------------------------------------------------------------------
// Vento nei gruppi di cambiamento
// ---------------------------------------------------------------------------

#[test]
fn wind_in_fm_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020 FM251800 18015KT 9999 FEW030",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 2);
    let fm = &t.forecasts[1];
    let w = fm.wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(180));
    assert_eq!(w.speed, 15);
}

#[test]
fn wind_in_becmg_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020 BECMG 2516/2518 VRB03KT",
    )
    .unwrap();
    let becmg = t.forecasts.iter().find(|f| {
        matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::BECMG)
    }).unwrap();
    let w = becmg.wind.as_ref().unwrap();
    assert_eq!(w.direction, None); // VRB
    assert_eq!(w.speed, 3);
}

#[test]
fn wind_in_tempo_group() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020 TEMPO 2514/2516 02015G25KT",
    )
    .unwrap();
    let tempo = t.forecasts.iter().find(|f| {
        matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::TEMPO)
    }).unwrap();
    let w = tempo.wind.as_ref().unwrap();
    assert_eq!(w.speed, 15);
    assert_eq!(w.gust, Some(25));
}

// ---------------------------------------------------------------------------
// Assenza del gruppo vento
// ---------------------------------------------------------------------------

#[test]
fn wind_absent_is_none() {
    // Il vento può essere assente in un blocco di cambiamento che non lo modifica
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020 TEMPO 2514/2516 4000 RASN",
    )
    .unwrap();
    let tempo = t.forecasts.iter().find(|f| {
        matches!(f.kind, metar_taf_parser::taf::models::forecast::TafForecastKind::TEMPO)
    }).unwrap();
    assert!(tempo.wind.is_none());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_wind_directional() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 02008KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wind = desc.forecasts[0].wind.as_ref().unwrap();
    assert!(wind.contains("20°"), "expected direction in: {wind}");
    assert!(wind.contains("8"), "expected speed in: {wind}");
    assert!(wind.contains("kt"), "expected unit in: {wind}");
}

#[test]
fn describe_wind_calm() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wind = desc.forecasts[0].wind.as_ref().unwrap();
    assert!(wind.contains("0°") || wind.contains("calm") || wind.contains("0 kt"),
        "expected calm wind description in: {wind}");
}

#[test]
fn describe_wind_variable() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 VRB05KT 9999 FEW020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wind = desc.forecasts[0].wind.as_ref().unwrap();
    assert!(wind.contains("variable"), "expected 'variable' in: {wind}");
}

#[test]
fn describe_wind_with_gust() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 02008G19KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wind = desc.forecasts[0].wind.as_ref().unwrap();
    assert!(wind.contains("gust") || wind.contains("19"), "expected gust in: {wind}");
}

