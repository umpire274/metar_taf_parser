//! Integration tests for TAF Gruppo 7 — gruppi evolutivi (FM, BECMG, TEMPO).
//!
//! - `FMDDHHmm` — From: inizio di una parte autonoma della previsione. Tutte le
//!   condizioni precedenti sono sostituite da quelle seguenti.
//! - `BECMG DDHH/DDHH` — Becoming: cambiamento graduale nel periodo indicato
//!   (max 4 ore). Gli elementi non descritti rimangono invariati.
//! - `TEMPO DDHH/DDHH` — Temporary: fluttuazioni temporanee nel periodo indicato.
//!   Ogni singola fluttuazione dura al massimo un'ora.

use metar_taf_parser::metar::models::weather::WeatherPhenomenon;
use metar_taf_parser::taf::models::forecast::TafForecastKind;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ===========================================================================
// FM — From (cambio significativo, blocco autonomo)
// ===========================================================================

#[test]
fn fm_creates_new_forecast_block() {
    // FM251800 — inizio blocco autonomo dal 25° giorno alle 18:00Z
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251800 22013KT 9999 FEW030")
            .unwrap();
    assert_eq!(t.forecasts.len(), 2);
    let fm = &t.forecasts[1];
    assert_eq!(fm.kind, TafForecastKind::FM);
    assert_eq!(fm.from, Some((25, 18, 0)));
}

#[test]
fn fm_fields_correct() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251800 22013KT 9999 FEW030")
            .unwrap();
    let fm = &t.forecasts[1];
    let w = fm.wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(220));
    assert_eq!(w.speed, 13);
}

#[test]
fn fm_replaces_all_previous_conditions() {
    // Il blocco FM NON eredita le nubi/visibilità dal blocco precedente.
    // Il Vec clouds del blocco FM deve contenere solo ciò che è scritto dopo FM.
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN010 OVC020 FM251800 22013KT 9999 FEW030",
    )
    .unwrap();
    let fm = &t.forecasts[1];
    assert_eq!(
        fm.clouds.len(),
        1,
        "FM deve contenere solo le nubi del suo blocco"
    );
    assert_eq!(
        fm.clouds[0].amount,
        metar_taf_parser::metar::models::cloud::CloudAmount::FEW
    );
}

#[test]
fn fm_with_non_zero_minutes() {
    // FM251830 — ore 18:30Z
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251830 22013KT 9999 FEW030")
            .unwrap();
    let fm = &t.forecasts[1];
    assert_eq!(fm.from, Some((25, 18, 30)));
}

#[test]
fn fm_multiple_groups() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 \
         FM251500 22010KT 9999 FEW030 \
         FM260000 27015KT CAVOK",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 3);
    assert_eq!(t.forecasts[1].from, Some((25, 15, 0)));
    assert_eq!(t.forecasts[2].from, Some((26, 0, 0)));
}

#[test]
fn fm_invalid_time_ignored() {
    // FM con orario invalido (minuti 99) non deve aprire un nuovo blocco
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251899 22013KT").unwrap();
    // FM invalido: rimane un solo blocco, ma il vento viene parsato nel base
    assert_eq!(t.forecasts.len(), 1);
}

// ===========================================================================
// BECMG — Becoming (cambiamento graduale)
// ===========================================================================

#[test]
fn becmg_manual_example_wind_change() {
    // Esempio dal manuale: BECMG 2521/2523 22013KT
    // Dalle 21:00Z del 25° al 23:00Z del 25°, vento 220°/13 kt
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2521/2523 22013KT")
        .unwrap();
    assert_eq!(t.forecasts.len(), 2);

    let becmg = &t.forecasts[1];
    assert_eq!(becmg.kind, TafForecastKind::BECMG);

    let p = becmg.period.as_ref().unwrap();
    assert_eq!(p.from, (25, 21, 0));
    assert_eq!(p.to, (25, 23, 0));

    let w = becmg.wind.as_ref().unwrap();
    assert_eq!(w.direction, Some(220));
    assert_eq!(w.speed, 13);
}

#[test]
fn becmg_period_within_4_hours() {
    // La specifica dice: il periodo di BECMG non supera le 4 ore
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2521/2523 22013KT")
        .unwrap();
    let p = t.forecasts[1].period.as_ref().unwrap();
    // da 21 a 23 = 2 ore <= 4 ore
    let duration_h = (p.to.1 as i16) - (p.from.1 as i16);
    assert!(duration_h <= 4, "durata BECMG deve essere <= 4 ore");
}

#[test]
fn becmg_unmodified_elements_are_absent() {
    // Un elemento non descritto in BECMG non deve apparire nel blocco
    // (il consumatore mantiene il valore precedente).
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2521/2523 22013KT")
        .unwrap();
    let becmg = &t.forecasts[1];
    // La visibilità non è descritta nel BECMG → deve essere None
    assert!(
        becmg.visibility.is_none(),
        "visibilità non modificata deve essere assente nel blocco BECMG"
    );
    // Le nubi non sono descritte nel BECMG → devono essere vuote
    assert!(
        becmg.clouds.is_empty(),
        "nubi non modificate devono essere assenti nel blocco BECMG"
    );
}

#[test]
fn becmg_crosses_midnight() {
    // BECMG 2523/2601 — da 23:00Z del 25° alle 01:00Z del 26°
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2523/2601 22013KT")
        .unwrap();
    let p = t.forecasts[1].period.as_ref().unwrap();
    assert_eq!(p.from, (25, 23, 0));
    assert_eq!(p.to, (26, 1, 0));
}

#[test]
fn becmg_multiple_groups() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN020 \
         BECMG 2514/2516 9999 SCT030 \
         BECMG 2521/2523 22013KT",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 3);
    assert_eq!(t.forecasts[1].kind, TafForecastKind::BECMG);
    assert_eq!(t.forecasts[2].kind, TafForecastKind::BECMG);
}

// ===========================================================================
// TEMPO — Temporary (fluttuazioni temporanee)
// ===========================================================================

#[test]
fn tempo_manual_example_tsra() {
    // Esempio dal manuale: TEMPO 2600/2603 TSRA
    // Tra le 00:00Z del 26° e le 03:00Z del 26° temporali con pioggia
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2600/2603 TSRA SCT020CB")
            .unwrap();
    assert_eq!(t.forecasts.len(), 2);

    let tempo = &t.forecasts[1];
    assert_eq!(tempo.kind, TafForecastKind::TEMPO);

    let p = tempo.period.as_ref().unwrap();
    assert_eq!(p.from, (26, 0, 0));
    assert_eq!(p.to, (26, 3, 0));

    let w = &tempo.weather[0];
    assert!(
        w.descriptors
            .contains(&metar_taf_parser::metar::models::weather::WeatherDescriptor::Thunderstorm)
    );
    assert!(w.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn tempo_period_correct() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2514/2516 -RA").unwrap();
    let p = t.forecasts[1].period.as_ref().unwrap();
    assert_eq!(p.from, (25, 14, 0));
    assert_eq!(p.to, (25, 16, 0));
}

#[test]
fn tempo_crosses_midnight() {
    // TEMPO 2523/2601 — tra 23:00Z del 25° e 01:00Z del 26°
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2523/2601 +RASN").unwrap();
    let p = t.forecasts[1].period.as_ref().unwrap();
    assert_eq!(p.from, (25, 23, 0));
    assert_eq!(p.to, (26, 1, 0));
}

#[test]
fn tempo_multiple_groups() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 \
         TEMPO 2514/2516 -RA \
         TEMPO 2600/2603 TSRA SCT020CB",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 3);
    assert!(
        t.forecasts
            .iter()
            .all(|f| f.kind == TafForecastKind::Base || f.kind == TafForecastKind::TEMPO)
    );
}

#[test]
fn tempo_after_fm() {
    // TEMPO dopo FM — deve essere relativo al blocco FM
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 \
         FM251500 22010KT 9999 FEW030 \
         TEMPO 2518/2520 TSRA SCT020CB",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 3);
    assert_eq!(t.forecasts[2].kind, TafForecastKind::TEMPO);
}

// ===========================================================================
// Combinazioni e casi completi
// ===========================================================================

#[test]
fn complete_taf_all_change_groups() {
    // TAF completo con base + FM + BECMG + TEMPO
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 \
         18010KT 9999 SCT020 \
         FM251500 22010KT 9999 FEW030 \
         BECMG 2521/2523 22013KT \
         TEMPO 2600/2603 TSRA SCT020CB",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 4);
    assert_eq!(t.forecasts[0].kind, TafForecastKind::Base);
    assert_eq!(t.forecasts[1].kind, TafForecastKind::FM);
    assert_eq!(t.forecasts[2].kind, TafForecastKind::BECMG);
    assert_eq!(t.forecasts[3].kind, TafForecastKind::TEMPO);
}

#[test]
fn change_groups_preserve_independent_content() {
    // Ogni blocco deve contenere solo i propri elementi
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 BKN020 \
         BECMG 2521/2523 22013KT",
    )
    .unwrap();
    // base: vento + vis + nubi
    let base = &t.forecasts[0];
    assert!(base.wind.is_some());
    assert!(base.visibility.is_some());
    assert!(!base.clouds.is_empty());

    // becmg: solo vento
    let becmg = &t.forecasts[1];
    assert!(becmg.wind.is_some());
    assert!(becmg.visibility.is_none());
    assert!(becmg.clouds.is_empty());
}

// ===========================================================================
// Describe
// ===========================================================================

#[test]
fn describe_fm_kind_label() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 FM251800 22013KT 9999 FEW030")
            .unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.forecasts.len(), 2);
    // Il blocco FM ha un proprio campo "kind"
    assert!(
        desc.forecasts[1].kind.contains("From") || desc.forecasts[1].kind.contains("from"),
        "expected FM kind label in: {}",
        desc.forecasts[1].kind
    );
}

#[test]
fn describe_becmg_kind_label() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2521/2523 22013KT")
        .unwrap();
    let desc = describe_taf(&t, Language::En);
    let kind = &desc.forecasts[1].kind;
    assert!(
        kind.contains("Becom") || kind.contains("becom"),
        "expected BECMG kind label in: {kind}"
    );
}

#[test]
fn describe_tempo_kind_label() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2514/2516 -RA").unwrap();
    let desc = describe_taf(&t, Language::En);
    let kind = &desc.forecasts[1].kind;
    assert!(
        kind.contains("Temp") || kind.contains("temp"),
        "expected TEMPO kind label in: {kind}"
    );
}

#[test]
fn describe_becmg_period() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 BECMG 2521/2523 22013KT")
        .unwrap();
    let desc = describe_taf(&t, Language::En);
    let period = desc.forecasts[1].period.as_ref().unwrap();
    assert!(period.contains("25"), "expected day 25 in period: {period}");
}

#[test]
fn describe_tempo_period() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TEMPO 2600/2603 TSRA").unwrap();
    let desc = describe_taf(&t, Language::En);
    let period = desc.forecasts[1].period.as_ref().unwrap();
    assert!(period.contains("26"), "expected day 26 in period: {period}");
}
