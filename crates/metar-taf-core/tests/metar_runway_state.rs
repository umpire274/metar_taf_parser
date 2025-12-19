use metar_taf_core::parse_metar;

#[test]
fn parse_runway_state_full_icao() {
    let metar = "METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert_eq!(parsed.runway_state.len(), 1);
    let rs = &parsed.runway_state[0];

    assert_eq!(rs.runway_designator, "19");
    assert_eq!(rs.deposit_type, Some(4)); // Dry snow
    assert_eq!(rs.contamination_extent, Some(5)); // 26–50%
    assert_eq!(rs.deposit_depth.as_deref(), Some("02")); // 2 mm
    assert_eq!(rs.braking_action.as_deref(), Some("35")); // friction / braking
}

#[test]
fn parse_runway_state_with_missing_fields() {
    let metar = "METAR XXXX 181200Z 18005KT 9999 R01/39//37";
    let parsed = parse_metar(metar).unwrap();

    let rs = &parsed.runway_state[0];
    assert_eq!(rs.runway_designator, "01");
    assert_eq!(rs.deposit_type, Some(3));
    assert_eq!(rs.contamination_extent, Some(9));
    assert_eq!(rs.deposit_depth.as_deref(), None);
    assert_eq!(rs.braking_action.as_deref(), Some("37"));
}

#[test]
fn parse_runway_state_preserve_codes() {
    let metar = "METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    let rs = &parsed.runway_state[0];

    assert_eq!(rs.runway_designator, "19");
    assert_eq!(rs.deposit_type, Some(4));
    assert_eq!(rs.contamination_extent, Some(5));

    // IMPORTANT: preserved as string
    assert_eq!(rs.deposit_depth.as_deref(), Some("02"));
    assert_eq!(rs.braking_action.as_deref(), Some("35"));
}
