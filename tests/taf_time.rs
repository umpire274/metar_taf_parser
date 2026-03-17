//! Integration tests for TAF Gruppo 2 — data-orario di emissione e periodo di validità.
//!
//! **Emissione** `DDHHmmZ`: giorno (01–31), ora UTC (00–23), minuti (00–59), suffisso Z.
//! **Validità** `DDHH/DDHH`: giorno/ora di inizio e fine; l'ora finale può essere 24
//! (mezzanotte a cavallo del giorno successivo).

use metar_taf_parser::{parse_taf, parse_taf_strict};

// ---------------------------------------------------------------------------
// Issued-at (DDHHmmZ) — valori corretti
// ---------------------------------------------------------------------------

#[test]
fn issued_at_fields_correct() {
    // 251100Z → day=25, hour=11, minute=00
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let ia = t.issued_at.unwrap();
    assert_eq!(ia.day, 25);
    assert_eq!(ia.hour, 11);
    assert_eq!(ia.minute, 0);
}

#[test]
fn issued_at_non_zero_minutes() {
    // 181133Z → minute=33 deve essere accettato
    let t = parse_taf("TAF LIRF 181133Z 1812/1918 18010KT 9999 SCT020").unwrap();
    let ia = t.issued_at.unwrap();
    assert_eq!(ia.day, 18);
    assert_eq!(ia.hour, 11);
    assert_eq!(ia.minute, 33);
}

#[test]
fn issued_at_midnight() {
    // 120000Z → ora 00:00 del giorno 12
    let t = parse_taf("TAF LIRF 120000Z 1200/1306 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.issued_at.unwrap().hour, 0);
}

#[test]
fn issued_at_end_of_day() {
    // 122359Z → 23:59 — l'ora massima valida per l'emissione
    let t = parse_taf("TAF LIRF 122359Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let ia = t.issued_at.unwrap();
    assert_eq!(ia.hour, 23);
    assert_eq!(ia.minute, 59);
}

#[test]
fn issued_at_last_day_of_month() {
    // day=31 deve essere accettato
    let t = parse_taf("TAF LIRF 311100Z 3112/0106 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.issued_at.unwrap().day, 31);
}

// ---------------------------------------------------------------------------
// Issued-at — valori invalidi
// ---------------------------------------------------------------------------

#[test]
fn issued_at_rejects_day_zero() {
    assert!(parse_taf("TAF LIRF 001100Z 1212/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn issued_at_rejects_hour_24() {
    // L'ora di emissione non può essere 24 (al contrario della fine della validità)
    assert!(parse_taf("TAF LIRF 122400Z 1212/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn issued_at_rejects_minute_60() {
    assert!(parse_taf("TAF LIRF 122460Z 1212/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn issued_at_rejects_missing_z_suffix() {
    assert!(parse_taf("TAF LIRF 121100 1212/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn issued_at_rejects_wrong_length() {
    // 6 caratteri invece di 7
    assert!(parse_taf("TAF LIRF 12110Z 1212/1318 18010KT 9999 SCT020").is_err());
}

// ---------------------------------------------------------------------------
// Validity (DDHH/DDHH) — valori corretti
// ---------------------------------------------------------------------------

#[test]
fn validity_same_day_start_end() {
    // 1212/1318 → inizia il 12 alle 12, finisce il 13 alle 18
    let t = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let v = t.validity.unwrap();
    assert_eq!(v.from_day, 12);
    assert_eq!(v.from_hour, 12);
    assert_eq!(v.to_day, 13);
    assert_eq!(v.to_hour, 18);
}

#[test]
fn validity_cross_day() {
    // 2512/2618 → esempio dal manuale: 25/12Z a 26/18Z
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let v = t.validity.unwrap();
    assert_eq!(v.from_day, 25);
    assert_eq!(v.from_hour, 12);
    assert_eq!(v.to_day, 26);
    assert_eq!(v.to_hour, 18);
}

#[test]
fn validity_to_hour_24_accepted() {
    // Ora finale 24 = mezzanotte che chiude il giorno
    let t = parse_taf("TAF LIRF 121100Z 1212/1324 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.validity.unwrap().to_hour, 24);
}

#[test]
fn validity_crosses_midnight() {
    // 2518/2600 → inizia 25/18Z, finisce 26/00Z
    let t = parse_taf("TAF LIRF 251700Z 2518/2600 18010KT 9999 SCT020").unwrap();
    let v = t.validity.unwrap();
    assert_eq!(v.from_day, 25);
    assert_eq!(v.from_hour, 18);
    assert_eq!(v.to_day, 26);
    assert_eq!(v.to_hour, 0);
}

#[test]
fn validity_crosses_month_boundary() {
    // 3118/0106 → fine mese: inizia 31/18Z, finisce 01/06Z del mese successivo
    let t = parse_taf("TAF LIRF 311700Z 3118/0106 18010KT 9999 SCT020").unwrap();
    let v = t.validity.unwrap();
    assert_eq!(v.from_day, 31);
    assert_eq!(v.to_day, 1);
    assert_eq!(v.to_hour, 6);
}

#[test]
fn validity_from_hour_zero() {
    // Inizio a mezzanotte: from_hour=00
    let t = parse_taf("TAF LIRF 122300Z 1300/1406 18010KT 9999 SCT020").unwrap();
    assert_eq!(t.validity.unwrap().from_hour, 0);
}

// ---------------------------------------------------------------------------
// Validity — valori invalidi
// ---------------------------------------------------------------------------

#[test]
fn validity_rejects_from_day_zero() {
    assert!(parse_taf("TAF LIRF 121100Z 0012/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_to_day_zero() {
    assert!(parse_taf("TAF LIRF 121100Z 1212/0018 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_from_hour_24() {
    // L'ora di inizio validità non può essere 24
    assert!(parse_taf("TAF LIRF 121100Z 1224/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_to_hour_25() {
    assert!(parse_taf("TAF LIRF 121100Z 1212/1325 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_from_hour_30() {
    assert!(parse_taf("TAF LIRF 121100Z 1230/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_wrong_format_missing_slash() {
    assert!(parse_taf("TAF LIRF 121100Z 12121318 18010KT 9999 SCT020").is_err());
}

#[test]
fn validity_rejects_short_from_part() {
    assert!(parse_taf("TAF LIRF 121100Z 122/1318 18010KT 9999 SCT020").is_err());
}

// ---------------------------------------------------------------------------
// Describe — formato dell'output
// ---------------------------------------------------------------------------

#[test]
fn describe_issued_at_format() {
    use metar_taf_parser::{Language, describe_taf};
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.issued_at.as_deref(), Some("Day 25 at 11:00Z"));
}

#[test]
fn describe_validity_format() {
    use metar_taf_parser::{Language, describe_taf};
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&t, Language::En);
    assert_eq!(desc.validity.as_deref(), Some("25/12Z to 26/18Z"));
}

// ---------------------------------------------------------------------------
// Strict mode — time groups are validated the same way
// ---------------------------------------------------------------------------

#[test]
fn strict_accepts_valid_time_groups() {
    let result = parse_taf_strict("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020");
    assert!(result.is_ok());
}

#[test]
fn strict_rejects_invalid_issued_at() {
    assert!(parse_taf_strict("TAF LIRF 122400Z 1212/1318 18010KT 9999 SCT020").is_err());
}

#[test]
fn strict_rejects_invalid_validity() {
    assert!(parse_taf_strict("TAF LIRF 121100Z 1224/1318 18010KT 9999 SCT020").is_err());
}

