use metar_taf_parser::{parse_taf, parse_taf_strict};

#[test]
fn parse_taf_wind_shear_group() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 WS020/18040KT")
        .expect("TAF should parse");

    let base = &taf.forecasts[0];
    let ws = base
        .wind_shear
        .as_ref()
        .expect("wind shear should be parsed");

    assert_eq!(ws.height_hundreds_ft, 20);
    assert_eq!(ws.direction, 180);
    assert_eq!(ws.speed_kt, 40);
}

#[test]
fn malformed_wind_shear_group_is_unparsed() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 WS02A/18040KT")
        .expect("TAF should parse");

    assert!(taf.unparsed_groups.iter().any(|g| g == "WS02A/18040KT"));
}

#[test]
fn strict_mode_rejects_malformed_wind_shear_group() {
    let err = parse_taf_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 WS02A/18040KT")
        .expect_err("strict TAF should reject malformed WS group");

    match err {
        metar_taf_parser::taf::errors::TafError::UnsupportedGroup(groups) => {
            assert!(groups.contains("WS02A/18040KT"));
        }
        _ => panic!("unexpected error kind"),
    }
}
