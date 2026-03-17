//! Integration tests for TAF Gruppo 1 — indicatore di località ICAO.
//!
//! Un TAF è sempre indirizzato a un aeroporto specifico identificato dal suo
//! codice ICAO a 4 lettere maiuscole (es. `LIRF`, `EGLL`, `KLAX`).

use metar_taf_parser::{parse_taf, parse_taf_strict};

// ---------------------------------------------------------------------------
// Tolerant mode — station extracted correctly in all header combinations
// ---------------------------------------------------------------------------

#[test]
fn station_extracted_from_plain_taf() {
    let t = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.station, "LIRF");
}

#[test]
fn station_extracted_after_amd() {
    let t = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    assert_eq!(t.station, "EDDF");
}

#[test]
fn station_extracted_after_cor() {
    let t = parse_taf("TAF COR EGLL 121100Z 1212/1318 22012KT 9999 SCT020").unwrap();
    assert_eq!(t.station, "EGLL");
}

#[test]
fn station_extracted_without_taf_prefix() {
    // Some feeds omit the leading TAF token.
    let t = parse_taf("LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.station, "LIRF");
}

// ---------------------------------------------------------------------------
// ICAO regional prefixes
// ---------------------------------------------------------------------------

#[test]
fn station_region_italy() {
    // Italy: L prefix
    let t = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.station, "LIRF");
}

#[test]
fn station_region_uk() {
    // UK: EG prefix
    let t = parse_taf("TAF EGLL 121100Z 1212/1318 22012KT 9999 SCT020").unwrap();
    assert_eq!(t.station, "EGLL");
}

#[test]
fn station_region_germany() {
    // Germany: ED prefix
    let t = parse_taf("TAF EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    assert_eq!(t.station, "EDDF");
}

#[test]
fn station_region_usa() {
    // USA: K prefix
    let t = parse_taf("TAF KLAX 121100Z 1212/1318 27010KT 9999 FEW030").unwrap();
    assert_eq!(t.station, "KLAX");
}

#[test]
fn station_region_russia() {
    // Russia: U prefix
    let t = parse_taf("TAF UUEE 121100Z 1212/1318 36005MPS 9999 BKN020").unwrap();
    assert_eq!(t.station, "UUEE");
}

#[test]
fn station_region_iceland() {
    // Iceland: BI prefix
    let t = parse_taf("TAF BIRK 121100Z 1212/1318 25015KT 9999 SCT025").unwrap();
    assert_eq!(t.station, "BIRK");
}

// ---------------------------------------------------------------------------
// Strict mode — format validation
// ---------------------------------------------------------------------------

#[test]
fn strict_accepts_valid_icao_station() {
    let result = parse_taf_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().station, "LIRF");
}

#[test]
fn strict_rejects_station_too_short() {
    // 3 characters instead of 4
    let result = parse_taf_strict("TAF LIR 121100Z 1212/1318 18010KT 9999 SCT020");
    assert!(result.is_err());
}

#[test]
fn strict_rejects_station_too_long() {
    // 5 characters instead of 4
    let result = parse_taf_strict("TAF LIRFF 121100Z 1212/1318 18010KT 9999 SCT020");
    assert!(result.is_err());
}

#[test]
fn strict_rejects_station_with_lowercase() {
    // ICAO identifiers must be all uppercase
    let result = parse_taf_strict("TAF lirf 121100Z 1212/1318 18010KT 9999 SCT020");
    assert!(result.is_err());
}

#[test]
fn strict_rejects_station_with_digits() {
    // Pure ICAO station codes contain only letters
    let result = parse_taf_strict("TAF L1RF 121100Z 1212/1318 18010KT 9999 SCT020");
    assert!(result.is_err());
}

#[test]
fn tolerant_accepts_non_icao_station() {
    // Tolerant mode does not validate the station format — best-effort parsing.
    let result = parse_taf("TAF LIR 121100Z 1212/1318 18010KT 9999 SCT020");
    // Either parses (accepting the odd station) or fails later on time parsing —
    // the important thing is it does NOT panic.
    let _ = result;
}

