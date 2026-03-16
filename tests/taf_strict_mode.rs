use metar_taf_parser::{parse_taf, parse_taf_strict};

#[test]
fn tolerant_taf_parsing_keeps_unknown_groups() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 ZZZ")
        .expect("TAF should parse in tolerant mode");

    assert!(taf.unparsed_groups.iter().any(|g| g == "ZZZ"));
}

#[test]
fn strict_taf_parsing_rejects_unknown_groups() {
    let err = parse_taf_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 ZZZ")
        .expect_err("strict TAF should reject unknown groups");

    match err {
        metar_taf_parser::taf::errors::TafError::UnsupportedGroup(groups) => {
            assert!(groups.contains("ZZZ"));
        }
        _ => panic!("unexpected error kind"),
    }
}

#[test]
fn strict_taf_parsing_accepts_fully_supported_message() {
    let taf = parse_taf_strict("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030")
        .expect("strict TAF should parse supported message");

    assert!(taf.unparsed_groups.is_empty());
}
