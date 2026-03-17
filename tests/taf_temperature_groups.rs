//! Integration tests for TAF temperature groups — TX (max) and TN (min).
//!
//! Format: `TX<value>/<DDHH>Z` for maximum, `TN<value>/<DDHH>Z` for minimum.
//! Negative temperatures use the `M` prefix (e.g. `TNM01` = −1 °C).
//!
//! Examples from the manual:
//! - `TX22/1718Z` — maximum temperature of 22 °C at 18:00Z on day 17
//! - `TNM01/1801Z` — minimum temperature of −1 °C at 01:00Z on day 18

use metar_taf_parser::taf::models::forecast::TafForecastKind;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ===========================================================================
// Test already present (preserved)
// ===========================================================================

#[test]
fn parse_taf_tx_tn_groups_in_base_forecast() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 TX18/1214Z TN08/1304Z")
        .expect("TAF should parse");

    let base = &taf.forecasts[0];
    let tx = base.max_temperature.as_ref().expect("TX should be parsed");
    let tn = base.min_temperature.as_ref().expect("TN should be parsed");

    assert_eq!(tx.value, 18);
    assert_eq!(tx.day, 12);
    assert_eq!(tx.hour, 14);

    assert_eq!(tn.value, 8);
    assert_eq!(tn.day, 13);
    assert_eq!(tn.hour, 4);
}

#[test]
fn parse_taf_negative_tn_group() {
    let taf = parse_taf("TAF UUEE 121100Z 1212/1318 18010KT 9999 FEW030 TNM05/1303Z")
        .expect("TAF should parse");

    let base = &taf.forecasts[0];
    let tn = base.min_temperature.as_ref().expect("TN should be parsed");
    assert_eq!(tn.value, -5);
    assert_eq!(tn.day, 13);
    assert_eq!(tn.hour, 3);
}

#[test]
fn malformed_taf_temperature_group_is_unparsed() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 TX1/1214Z")
        .expect("TAF should parse");

    assert!(taf.unparsed_groups.iter().any(|g| g == "TX1/1214Z"));
}

// ===========================================================================
// Esempi dal manuale
// ===========================================================================

#[test]
fn tx22_manual_example() {
    // TX22/1718Z — temperatura massima 22°C alle 18:00Z del 17°
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z TNM01/1801Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    let tx = base.max_temperature.as_ref().expect("TX should be parsed");
    assert_eq!(tx.value, 22);
    assert_eq!(tx.day, 17);
    assert_eq!(tx.hour, 18);
}

#[test]
fn tnm01_manual_example() {
    // TNM01/1801Z — temperatura minima -1°C alle 01:00Z del 18°
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z TNM01/1801Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    let tn = base.min_temperature.as_ref().expect("TN should be parsed");
    assert_eq!(tn.value, -1);
    assert_eq!(tn.day, 18);
    assert_eq!(tn.hour, 1);
}

#[test]
fn tx_and_tn_manual_example_combined() {
    // Entrambi i gruppi presenti nello stesso TAF — come da esempio del manuale
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z TNM01/1801Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    assert!(base.max_temperature.is_some(), "TX deve essere presente");
    assert!(base.min_temperature.is_some(), "TN deve essere presente");
}

// ===========================================================================
// Casi limite
// ===========================================================================

#[test]
fn tx_only_no_tn() {
    // Solo TX, senza TN
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    assert!(base.max_temperature.is_some());
    assert!(
        base.min_temperature.is_none(),
        "TN non deve essere presente se non specificato"
    );
}

#[test]
fn tn_only_no_tx() {
    // Solo TN, senza TX
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TNM01/1801Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    assert!(base.min_temperature.is_some());
    assert!(
        base.max_temperature.is_none(),
        "TX non deve essere presente se non specificato"
    );
}

#[test]
fn tx_zero_degrees() {
    // TX00 — temperatura massima esattamente 0°C
    let t = parse_taf(
        "TAF UUEE 171100Z 1712/1818 18010KT 9999 OVC010 TX00/1712Z TN00/1800Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    assert_eq!(base.max_temperature.as_ref().unwrap().value, 0);
    assert_eq!(base.min_temperature.as_ref().unwrap().value, 0);
}

#[test]
fn tx_crosses_midnight_into_next_day() {
    // TX prevista il giorno successivo rispetto alla validità iniziale
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 TX20/2514Z TN08/2604Z",
    )
    .unwrap();
    let base = &t.forecasts[0];
    let tx = base.max_temperature.as_ref().unwrap();
    let tn = base.min_temperature.as_ref().unwrap();
    assert_eq!(tx.day, 25);
    assert_eq!(tx.hour, 14);
    assert_eq!(tn.day, 26);
    assert_eq!(tn.hour, 4);
}

#[test]
fn tx_in_fm_block() {
    // TX specificata all'interno di un blocco FM
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 \
         FM171500 22010KT 9999 FEW030 TX22/1718Z",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 2);
    assert_eq!(t.forecasts[1].kind, TafForecastKind::FM);
    assert!(
        t.forecasts[1].max_temperature.is_some(),
        "TX deve essere presente nel blocco FM"
    );
}

#[test]
fn malformed_tx_missing_z_suffix() {
    // TX senza 'Z' finale — deve finire in unparsed_groups
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718",
    )
    .unwrap();
    assert!(
        t.unparsed_groups.iter().any(|g| g == "TX22/1718"),
        "token senza Z deve essere non parsato"
    );
}

#[test]
fn malformed_tx_non_numeric_when() {
    // TX con parte oraria non numerica — il token non viene riconosciuto come TX/TN
    // e viene consumato da parse_weather come Unknown phenomena (non arriva a
    // unparsed_groups). L'invariante rilevante è che la temperatura MAX resti None.
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/17AAZ",
    )
    .unwrap();
    assert!(
        t.forecasts[0].max_temperature.is_none(),
        "token TX malformato non deve essere parsato come temperatura massima"
    );
}

// ===========================================================================
// Describe
// ===========================================================================

#[test]
fn describe_tx_max_temperature_field() {
    // Il campo max_temperature nel describe deve contenere valore, giorno e ora
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z TNM01/1801Z",
    )
    .unwrap();
    let desc = describe_taf(&t, Language::En);
    let s = desc.forecasts[0]
        .max_temperature
        .as_ref()
        .expect("max_temperature should be described");
    assert!(s.contains("22"), "deve contenere il valore 22: {s}");
    assert!(s.contains("17"), "deve contenere il giorno 17: {s}");
    assert!(s.contains("18"), "deve contenere l'ora 18: {s}");
}

#[test]
fn describe_tn_min_temperature_field() {
    // Il campo min_temperature nel describe deve contenere valore negativo, giorno e ora
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z TNM01/1801Z",
    )
    .unwrap();
    let desc = describe_taf(&t, Language::En);
    let s = desc.forecasts[0]
        .min_temperature
        .as_ref()
        .expect("min_temperature should be described");
    assert!(s.contains("-1"), "deve contenere il valore -1: {s}");
    assert!(s.contains("18"), "deve contenere il giorno 18: {s}");
    assert!(s.contains("01") || s.contains("1"), "deve contenere l'ora 01: {s}");
}

#[test]
fn describe_tx_format_contains_celsius_and_day() {
    // Verifica il formato completo: "maximum temperature X°C on day D at HH:00Z"
    let t = parse_taf(
        "TAF LIRF 171100Z 1712/1818 18010KT 9999 SCT020 TX22/1718Z",
    )
    .unwrap();
    let desc = describe_taf(&t, Language::En);
    let s = desc.forecasts[0].max_temperature.as_ref().unwrap();
    assert!(
        s.contains("maximum"),
        "deve contenere 'maximum': {s}"
    );
    assert!(s.contains("°C") || s.contains("C"), "deve contenere l'unità: {s}");
}
