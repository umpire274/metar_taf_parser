use metar_taf_parser::{parse_metar, parse_metar_strict};

#[test]
fn tolerant_metar_parsing_keeps_unknown_groups() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 ZZZ 18/12 Q1015")
        .expect("METAR should parse in tolerant mode");

    assert!(metar.unparsed_groups.iter().any(|g| g == "ZZZ"));
}

#[test]
fn strict_metar_parsing_rejects_unknown_groups() {
    let err = parse_metar_strict("LIRF 121250Z 18010KT 9999 FEW030 ZZZ 18/12 Q1015")
        .expect_err("strict METAR should reject unknown groups");

    match err {
        metar_taf_parser::metar::errors::MetarError::UnknownGroup(groups) => {
            assert!(groups.contains("ZZZ"));
        }
        _ => panic!("unexpected error kind"),
    }
}

#[test]
fn strict_metar_parsing_rejects_unknown_trend_payload_tokens() {
    let err = parse_metar_strict(
        "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO AT1330 UNKNOWN",
    )
    .expect_err("strict METAR should reject unknown trend payload tokens");

    match err {
        metar_taf_parser::metar::errors::MetarError::UnknownGroup(groups) => {
            assert!(groups.contains("UNKNOWN"));
        }
        _ => panic!("unexpected error kind"),
    }
}

#[test]
fn strict_metar_parsing_accepts_supported_message() {
    let metar = parse_metar_strict("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015")
        .expect("strict METAR should parse supported message");

    assert!(metar.unparsed_groups.is_empty());
}
