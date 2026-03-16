use metar_taf_core::parse_taf;

#[test]
fn stores_unknown_taf_groups_in_unparsed_list() {
    let taf =
        parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 ZZZ").expect("TAF should parse");

    assert!(taf.unparsed_groups.iter().any(|g| g == "ZZZ"));
}

#[test]
fn keeps_collecting_unknown_groups_across_change_sections() {
    let taf =
        parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 TEMPO 1220/1222 ABCDE BKN015")
            .expect("TAF should parse");

    assert!(taf.unparsed_groups.iter().any(|g| g == "ABCDE"));
}
