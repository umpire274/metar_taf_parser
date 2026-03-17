use metar_taf_parser::parse_metar;

#[test]
fn parse_valid_metar_time_group() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    let time = metar.time.expect("time should parse");
    assert_eq!(time.day, 12);
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 50);
}

#[test]
fn reject_invalid_metar_time_ranges() {
    let metar = parse_metar("LIRF 322460Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    assert!(metar.time.is_none());
}

#[test]
fn reject_non_numeric_metar_time() {
    let metar = parse_metar("LIRF 12AB50Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();

    assert!(metar.time.is_none());
}

#[test]
fn reject_day_zero() {
    // day 00 is out of the ICAO range 01–31
    let metar = parse_metar("LIRF 001250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(metar.time.is_none());
}

#[test]
fn parse_time_boundary_values() {
    // day 31, hour 23, minute 59 — all at the upper boundary
    let metar = parse_metar("LIRF 312359Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let time = metar.time.expect("time should parse");
    assert_eq!(time.day, 31);
    assert_eq!(time.hour, 23);
    assert_eq!(time.minute, 59);
}

#[test]
fn parse_time_hour_and_minute_zero_padded() {
    // 280925Z → day 28, hour 09, minute 25 (example from the manual)
    let metar = parse_metar("LIRF 280925Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let time = metar.time.expect("time should parse");
    assert_eq!(time.day, 28);
    assert_eq!(time.hour, 9);
    assert_eq!(time.minute, 25);
}

#[test]
fn describe_time_zero_padded_hour_and_minute() {
    use metar_taf_parser::{Language, describe_metar};
    // 280925Z → "Day 28 at 09:25Z"
    let metar = parse_metar("LIRF 280925Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);
    assert_eq!(desc.time.unwrap(), "Day 28 at 09:25Z");
}
