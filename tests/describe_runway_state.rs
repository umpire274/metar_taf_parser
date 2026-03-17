//! Integration tests for the natural-language description of runway state groups.
//!
//! These tests exercise the `describe_runway_state` helper indirectly through
//! `describe_metar`, covering SNOCLO, ICAO deposit/coverage/thickness/braking
//! codes, special numeric codes and the µ friction format.

use metar_taf_parser::{Language, describe_metar, parse_metar};

// ── SNOCLO ────────────────────────────────────────────────────────────────────

#[test]
fn describe_runway_state_snoclo() {
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R/SNOCLO").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.runway_state.len(), 1);
    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("airfield closed"),
        "expected SNOCLO description in: {}",
        rs
    );
    assert!(rs.contains("SNOCLO"), "expected SNOCLO label in: {}", rs);
}

// ── All fields missing (//////) ───────────────────────────────────────────────

#[test]
fn describe_runway_state_all_fields_missing() {
    // Token R09/////// → all six data slots are '/', every field is None.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R09///////").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    // At least deposit and coverage are "not reported".
    assert!(
        rs.contains("not reported"),
        "expected 'not reported' in: {}",
        rs
    );
}

// ── Deposit type ──────────────────────────────────────────────────────────────

#[test]
fn describe_runway_state_deposit_clear_and_dry() {
    // Deposit code 0 → "clear and dry".
    // Token R09/010235 → D=0, C=1, TT=02, BB=35.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R09/010235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("clear and dry"),
        "expected 'clear and dry' in: {}",
        rs
    );
}

// ── Coverage codes ────────────────────────────────────────────────────────────

#[test]
fn describe_runway_state_coverage_10_percent_or_less() {
    // Coverage code 1 → "10% or less".
    // Token R09/010235 → C=1.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R09/010235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("10% or less"),
        "expected '10% or less' in: {}",
        rs
    );
}

#[test]
fn describe_runway_state_coverage_11_25_percent() {
    // Coverage code 2 → "11–25%".
    // Token R09/020235 → C=2.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R09/020235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(rs.contains("11\u{2013}25%"), "expected '11–25%' in: {}", rs);
}

// ── Thickness codes ───────────────────────────────────────────────────────────

#[test]
fn describe_runway_state_thickness_less_than_1mm() {
    // Thickness code "00" → "less than 1 mm".
    // Token R01/450035 → TT=00.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/450035").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("less than 1 mm"),
        "expected <1 mm label in: {}",
        rs
    );
}

#[test]
fn describe_runway_state_thickness_special_10cm() {
    // Thickness code "92" → "10 cm".
    // Token R01/459235 → D=4, C=5, TT=92, BB=35.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/459235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(rs.contains("10 cm"), "expected '10 cm' in: {}", rs);
}

#[test]
fn describe_runway_state_thickness_special_40cm_or_more() {
    // Thickness code "98" → "40 cm or more".
    // Token R01/459835 → TT=98.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/459835").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("40 cm or more"),
        "expected '40 cm or more' in: {}",
        rs
    );
}

#[test]
fn describe_runway_state_thickness_99_closed() {
    // Thickness code "99" → "closed".
    // Token R01/459935 → TT=99.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/459935").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("thickness: closed"),
        "expected 'thickness: closed' in: {}",
        rs
    );
}

// ── Braking action codes ──────────────────────────────────────────────────────

#[test]
fn describe_runway_state_braking_as_friction_coefficient() {
    // Braking code "35" (≤75) → formatted as µ = 0.35.
    // Token R19/450235 → BB=35.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R19/450235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("\u{00b5} = 0.35"),
        "expected µ = 0.35 in: {}",
        rs
    );
}

#[test]
fn describe_runway_state_braking_code_91_poor() {
    // Braking code "91" → "poor".
    // Token R01/450291 → BB=91.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/450291").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(rs.contains("poor"), "expected 'poor' in: {}", rs);
}

#[test]
fn describe_runway_state_braking_code_93_medium() {
    // Braking code "93" → "medium".
    // Token R01/450293 → BB=93.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/450293").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(rs.contains("medium"), "expected 'medium' in: {}", rs);
}

#[test]
fn describe_runway_state_braking_code_95_good() {
    // Braking code "95" → "good".
    // Token R01/450295 → BB=95.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/450295").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(rs.contains("good"), "expected 'good' in: {}", rs);
}

#[test]
fn describe_runway_state_braking_code_99_unreliable() {
    // Braking code "99" → "figures unreliable".
    // Token R01/450299 → BB=99.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R01/450299").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("figures unreliable"),
        "expected 'figures unreliable' in: {}",
        rs
    );
}

// ── Designator with suffix ────────────────────────────────────────────────────

#[test]
fn describe_runway_state_designator_with_suffix() {
    // Runway designator "23L" must appear verbatim in the description.
    let metar = parse_metar("METAR EGLL 181200Z 18005KT 9999 R23L/450235").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rs = &desc.runway_state[0];
    assert!(
        rs.contains("runway 23L"),
        "expected 'runway 23L' in: {}",
        rs
    );
}
