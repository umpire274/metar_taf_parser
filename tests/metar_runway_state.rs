use metar_taf_parser::parse_metar;

#[test]
fn parse_runway_state_full_icao() {
    let metar = "METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.runway_state.len(), 1);
    let rs = &parsed.runway_state[0];

    assert_eq!(rs.runway_designator, "19");
    assert_eq!(rs.deposit_type, Some(4)); // Dry snow
    assert_eq!(rs.coverage, Some(5)); // 26–50%
    assert_eq!(rs.thickness.as_deref(), Some("02")); // 2 mm
    assert_eq!(rs.braking_action.as_deref(), Some("35")); // friction / braking
}

#[test]
fn parse_runway_state_with_missing_fields() {
    let metar = "METAR XXXX 181200Z 18005KT 9999 R01/39//37";
    let parsed = parse_metar(metar).unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "01");
    assert_eq!(rs.deposit_type, Some(3));
    assert_eq!(rs.coverage, Some(9));
    assert_eq!(rs.thickness.as_deref(), None);
    assert_eq!(rs.braking_action.as_deref(), Some("37"));
}

#[test]
fn parse_runway_state_preserve_codes() {
    let metar = "METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    let rs = &parsed.runway_state[0];

    assert_eq!(rs.runway_designator, "19");
    assert_eq!(rs.deposit_type, Some(4));
    assert_eq!(rs.coverage, Some(5));

    // IMPORTANT: preserved as string
    assert_eq!(rs.thickness.as_deref(), Some("02"));
    assert_eq!(rs.braking_action.as_deref(), Some("35"));
}

#[test]
fn reject_non_numeric_runway_designator() {
    let metar = "METAR XXXX 181200Z 18005KT 9999 RAB/390037";
    let parsed = parse_metar(metar).unwrap();

    assert!(parsed.runway_state.is_empty());
}

#[test]
fn reject_invalid_runway_state_data_characters() {
    let metar = "METAR XXXX 181200Z 18005KT 9999 R01/39A/37";
    let parsed = parse_metar(metar).unwrap();

    assert!(parsed.runway_state.is_empty());
}

#[test]
fn parse_runway_state_snoclo() {
    // R/SNOCLO signals the entire airfield is closed due to snow or ice.
    let parsed = parse_metar("METAR EGLL 181200Z 18005KT 9999 R/SNOCLO").unwrap();

    assert_eq!(parsed.runway_state.len(), 1);
    let rs = &parsed.runway_state[0];
    assert!(rs.snoclo);
    assert_eq!(rs.runway_designator, "");
    assert_eq!(rs.deposit_type, None);
    assert_eq!(rs.coverage, None);
    assert_eq!(rs.thickness, None);
    assert_eq!(rs.braking_action, None);
}

#[test]
fn parse_runway_state_with_left_suffix() {
    let parsed = parse_metar("METAR EGLL 181200Z 18005KT 9999 R23L/450235").unwrap();

    assert_eq!(parsed.runway_state.len(), 1);
    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "23L");
    assert_eq!(rs.deposit_type, Some(4));
    assert_eq!(rs.coverage, Some(5));
    assert_eq!(rs.thickness.as_deref(), Some("02"));
    assert_eq!(rs.braking_action.as_deref(), Some("35"));
}

#[test]
fn parse_runway_state_with_right_suffix() {
    let parsed = parse_metar("METAR EGLL 181200Z 18005KT 9999 R06R/210091").unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "06R");
    assert_eq!(rs.deposit_type, Some(2));
    assert_eq!(rs.coverage, Some(1));
    assert_eq!(rs.braking_action.as_deref(), Some("91"));
}

#[test]
fn parse_runway_state_with_centre_suffix() {
    let parsed = parse_metar("METAR EGLL 181200Z 18005KT 9999 R18C/010035").unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "18C");
    assert_eq!(rs.deposit_type, Some(0)); // clear and dry
    assert_eq!(rs.coverage, Some(1));
}

#[test]
fn parse_runway_state_all_data_missing() {
    // All six data positions are '/', so every field must be None.
    // Token: R + 09 + / (separator) + ////// (6 data slots) = "R09///////"
    let parsed = parse_metar("METAR EGLL 181200Z 18005KT 9999 R09///////").unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "09");
    assert_eq!(rs.deposit_type, None);
    assert_eq!(rs.coverage, None);
    assert_eq!(rs.thickness, None);
    assert_eq!(rs.braking_action, None);
}

#[test]
fn parse_runway_state_braking_action_missing() {
    let parsed = parse_metar("METAR XXXX 181200Z 18005KT 9999 R09/3950//").unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.deposit_type, Some(3));
    assert_eq!(rs.coverage, Some(9));
    assert_eq!(rs.thickness.as_deref(), Some("50"));
    assert_eq!(rs.braking_action, None);
}

#[test]
fn parse_runway_state_thickness_missing() {
    let parsed = parse_metar("METAR XXXX 181200Z 18005KT 9999 R09/39//35").unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.deposit_type, Some(3));
    assert_eq!(rs.coverage, Some(9));
    assert_eq!(rs.thickness, None);
    assert_eq!(rs.braking_action.as_deref(), Some("35"));
}

#[test]
fn parse_multiple_runway_states() {
    let metar = "METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 R01/010091 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.runway_state.len(), 2);
    assert_eq!(parsed.runway_state[0].runway_designator, "19");
    assert_eq!(parsed.runway_state[1].runway_designator, "01");
    assert_eq!(parsed.runway_state[1].deposit_type, Some(0)); // clear and dry
    assert_eq!(parsed.runway_state[1].braking_action.as_deref(), Some("91"));
}

#[test]
fn reject_designator_with_invalid_suffix() {
    // 'X' is not a valid suffix — only L, R, C are accepted.
    let parsed = parse_metar("METAR XXXX 181200Z 18005KT 9999 R23X/450235").unwrap();

    assert!(parsed.runway_state.is_empty());
}
