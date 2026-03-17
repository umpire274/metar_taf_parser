use metar_taf_parser::{Language, describe_metar, parse_metar};

#[test]
fn parse_positive_temperature() {
    let metar = "METAR LIRF 181200Z 18005KT 9999 SCT020 15/10 Q1015";
    let p = parse_metar(metar).unwrap();
    let t = p.temperature.expect("temperature missing");
    assert_eq!(t.temperature, 15);
    assert_eq!(t.dew_point, 10);
}

#[test]
fn parse_negative_temperature() {
    let metar = "METAR UOOO 181400Z 08002MPS CAVOK M25/M28 Q1014";
    let p = parse_metar(metar).unwrap();
    let t = p.temperature.expect("temperature missing");
    assert_eq!(t.temperature, -25);
    assert_eq!(t.dew_point, -28);
}

#[test]
fn reject_malformed_temperature_token() {
    let metar = "METAR LIRF 181200Z 18005KT 9999 SCT020 5/10 Q1015";
    let p = parse_metar(metar).unwrap();

    assert!(p.temperature.is_none());
}

#[test]
fn reject_out_of_range_temperature_token() {
    let metar = "METAR LIRF 181200Z 18005KT 9999 SCT020 99/10 Q1015";
    let p = parse_metar(metar).unwrap();

    assert!(p.temperature.is_none());
}

#[test]
fn parse_manual_example_temperature() {
    // 02/M01 — manual example: temperature 2°C, dew point -1°C
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 02/M01 Q1015").unwrap();
    let t = m.temperature.unwrap();
    assert_eq!(t.temperature, 2);
    assert_eq!(t.dew_point, -1);
}

#[test]
fn parse_zero_temperature_and_dew_point() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 00/00 Q1015").unwrap();
    let t = m.temperature.unwrap();
    assert_eq!(t.temperature, 0);
    assert_eq!(t.dew_point, 0);
}

#[test]
fn parse_temperature_equals_dew_point() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/18 Q1015").unwrap();
    let t = m.temperature.unwrap();
    assert_eq!(t.temperature, 18);
    assert_eq!(t.dew_point, 18);
}

#[test]
fn parse_negative_m00_dew_point() {
    // M00 is a valid encoding — negative zero, stored as 0
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 10/M00 Q1015").unwrap();
    let t = m.temperature.unwrap();
    assert_eq!(t.temperature, 10);
    assert_eq!(t.dew_point, 0);
}

#[test]
fn describe_temperature_positive() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let temp = desc.temperature.unwrap();
    assert!(temp.contains("18°C"), "{}", temp);
    assert!(temp.contains("12°C"), "{}", temp);
}

#[test]
fn describe_temperature_negative() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 02/M01 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let temp = desc.temperature.unwrap();
    assert!(temp.contains("2°C"), "{}", temp);
    assert!(temp.contains("-1°C"), "{}", temp);
}
