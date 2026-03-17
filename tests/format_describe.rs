use metar_taf_parser::{Language, format_metar, format_taf, parse_metar, parse_taf};

// ---------------------------------------------------------------------------
// format_metar tests
// ---------------------------------------------------------------------------

#[test]
fn format_metar_contains_header_and_all_present_fields() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG").unwrap();
    let text = format_metar(&metar, Language::En);

    assert!(text.contains("METAR LIRF"), "expected header: {}", text);
    assert!(text.contains("Time:"), "expected time label: {}", text);
    assert!(text.contains("Wind:"), "expected wind label: {}", text);
    assert!(
        text.contains("Visibility:"),
        "expected visibility label: {}",
        text
    );
    assert!(
        text.contains("Weather:"),
        "expected weather label: {}",
        text
    );
    assert!(text.contains("Clouds:"), "expected clouds label: {}", text);
    assert!(
        text.contains("Temperature:"),
        "expected temperature label: {}",
        text
    );
    assert!(
        text.contains("Pressure:"),
        "expected pressure label: {}",
        text
    );
    assert!(text.contains("Trend:"), "expected trend label: {}", text);
}

#[test]
fn format_metar_omits_absent_fields() {
    // Minimal METAR with no weather, no trend, no remarks
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let text = format_metar(&metar, Language::En);

    assert!(
        !text.contains("Weather:"),
        "unexpected weather label: {}",
        text
    );
    assert!(!text.contains("Trend:"), "unexpected trend label: {}", text);
    assert!(
        !text.contains("Remarks:"),
        "unexpected remarks label: {}",
        text
    );
    assert!(
        !text.contains("Status:"),
        "unexpected status label: {}",
        text
    );
}

#[test]
fn format_metar_includes_remarks_when_present() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 RMK AO2").unwrap();
    let text = format_metar(&metar, Language::En);

    assert!(
        text.contains("Remarks:"),
        "expected remarks label: {}",
        text
    );
    assert!(text.contains("AO2"), "expected rmk text: {}", text);
}

#[test]
fn format_metar_includes_status_for_auto() {
    let metar = parse_metar("METAR LIRF 121250Z AUTO 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let text = format_metar(&metar, Language::En);

    assert!(text.contains("Status:"), "expected status label: {}", text);
    assert!(text.contains("automated"), "expected AUTO text: {}", text);
}

#[test]
fn format_metar_multiple_weather_groups() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 -RA BCFG 18/12 Q1015").unwrap();
    let text = format_metar(&metar, Language::En);

    // Both weather groups must appear on separate Weather: lines
    let weather_lines: Vec<&str> = text
        .lines()
        .filter(|l| l.trim_start().starts_with("Weather:"))
        .collect();
    assert_eq!(weather_lines.len(), 2, "expected 2 weather lines: {}", text);
}

#[test]
fn format_metar_multiple_cloud_layers() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW015 SCT030 BKN080 18/12 Q1015").unwrap();
    let text = format_metar(&metar, Language::En);

    let cloud_lines: Vec<&str> = text
        .lines()
        .filter(|l| l.trim_start().starts_with("Clouds:"))
        .collect();
    assert_eq!(cloud_lines.len(), 3, "expected 3 cloud lines: {}", text);
}

#[test]
fn format_metar_cavok_no_cloud_lines() {
    let metar = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    let text = format_metar(&metar, Language::En);

    assert!(
        text.contains("Visibility:"),
        "expected visibility: {}",
        text
    );
    assert!(text.contains("CAVOK"), "expected CAVOK text: {}", text);
    // CAVOK clears clouds in the parser, so no cloud lines expected
    assert!(
        !text.contains("Clouds:"),
        "unexpected cloud lines: {}",
        text
    );
}

// ---------------------------------------------------------------------------
// format_taf tests
// ---------------------------------------------------------------------------

#[test]
fn format_taf_contains_header_and_base_forecast() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(text.contains("TAF LIRF"), "expected header: {}", text);
    assert!(text.contains("Issued:"), "expected issued label: {}", text);
    assert!(
        text.contains("Validity:"),
        "expected validity label: {}",
        text
    );
    assert!(
        text.contains("[Base forecast]"),
        "expected base block: {}",
        text
    );
    assert!(text.contains("Wind:"), "expected wind label: {}", text);
    assert!(text.contains("Clouds:"), "expected clouds label: {}", text);
}

#[test]
fn format_taf_shows_tempo_block() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TEMPO 1218/1222 4000 -RA")
        .unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(
        text.contains("[Temporary]"),
        "expected TEMPO block: {}",
        text
    );
    assert!(text.contains("12/18Z"), "expected period start: {}", text);
    assert!(
        text.contains("Weather:"),
        "expected weather label: {}",
        text
    );
    assert!(text.contains("rain"), "expected -RA: {}", text);
}

#[test]
fn format_taf_shows_becmg_block() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 BECMG 1215/1217 24015KT")
        .unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(
        text.contains("[Becoming]"),
        "expected BECMG block: {}",
        text
    );
    assert!(text.contains("240°"), "expected direction: {}", text);
}

#[test]
fn format_taf_shows_fm_block() {
    let taf =
        parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 FM121800 24020KT 9999 BKN040")
            .unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(text.contains("[From]"), "expected FM block: {}", text);
    assert!(text.contains("18:00Z"), "expected FM time: {}", text);
}

#[test]
fn format_taf_shows_temperatures() {
    let taf =
        parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TX18/1214Z TN08/1304Z").unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(
        text.contains("Max temp:"),
        "expected max temp label: {}",
        text
    );
    assert!(
        text.contains("Min temp:"),
        "expected min temp label: {}",
        text
    );
}

#[test]
fn format_taf_shows_wind_shear() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 WS020/25040KT").unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(
        text.contains("Wind shear:"),
        "expected wind shear label: {}",
        text
    );
    assert!(text.contains("2000 ft"), "expected height: {}", text);
}

#[test]
fn format_taf_amended_shows_status() {
    let taf = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(text.contains("Status:"), "expected status label: {}", text);
    assert!(text.contains("amended"), "expected AMD text: {}", text);
}

#[test]
fn format_taf_prob_block_shows_probability() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 PROB30 TEMPO 1214/1216 2000 TSRA",
    )
    .unwrap();
    let text = format_taf(&taf, Language::En);

    assert!(text.contains("(30%)"), "expected probability: {}", text);
}

// ---------------------------------------------------------------------------
// Display trait tests (via format! / to_string)
// ---------------------------------------------------------------------------

#[test]
fn metar_description_display_via_to_string() {
    use metar_taf_parser::{Language, describe_metar};

    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);
    let text = desc.to_string();

    // Display output must match format_metar output exactly
    let expected = format_metar(&metar, Language::En);
    assert_eq!(text, expected);
}

#[test]
fn taf_description_display_via_to_string() {
    use metar_taf_parser::{Language, describe_taf};

    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&taf, Language::En);
    let text = desc.to_string();

    let expected = format_taf(&taf, Language::En);
    assert_eq!(text, expected);
}
