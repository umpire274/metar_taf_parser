use metar_taf_core::parse_metar;

#[test]
fn stores_unknown_metar_groups_in_unparsed_list() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 ZZZ 18/12 Q1015")
        .expect("METAR should parse");

    assert!(metar.unparsed_groups.iter().any(|g| g == "ZZZ"));
}
