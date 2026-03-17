use metar_taf_parser::metar::models::trend::{MetarTrend, MetarTrendTimeKind};
use metar_taf_parser::{Language, describe_metar, parse_metar};

#[test]
fn parse_nosig_trend() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Nosig)));
    assert!(parsed.trend_detail.is_none());
}

#[test]
fn parse_becmg_trend_marker() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Becmg)));
}

#[test]
fn parse_tempo_trend_marker() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Tempo)));
}

#[test]
fn parse_becmg_trend_payload_details() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM1300 TL1500 22015KT 4000 -RA BKN020";
    let parsed = parse_metar(metar).unwrap();

    let trend = parsed.trend_detail.expect("trend detail expected");
    assert_eq!(trend.kind, MetarTrend::Becmg);
    assert_eq!(trend.times.len(), 2);
    assert_eq!(trend.times[0].kind, MetarTrendTimeKind::From);
    assert_eq!(trend.times[0].hour, 13);
    assert_eq!(trend.times[1].kind, MetarTrendTimeKind::Until);
    assert!(trend.wind.is_some());
    assert!(trend.visibility.is_some());
    assert_eq!(trend.weather.len(), 1);
    assert_eq!(trend.clouds.len(), 1);
    assert!(trend.unparsed_groups.is_empty());
}

#[test]
fn parse_tempo_trend_payload_keeps_unknown_tokens() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO AT1330 3000 NSW UNKNOWN";
    let parsed = parse_metar(metar).unwrap();

    let trend = parsed.trend_detail.expect("trend detail expected");
    assert_eq!(trend.kind, MetarTrend::Tempo);
    assert_eq!(trend.times.len(), 1);
    assert_eq!(trend.times[0].kind, MetarTrendTimeKind::At);
    assert!(trend.unparsed_groups.contains(&"UNKNOWN".to_string()));
}

// ---------------------------------------------------------------------------
// Manual examples
// ---------------------------------------------------------------------------

#[test]
fn parse_becmg_fm_visibility_manual_example() {
    // "FM1200 6000 means that the visibility will be 6 km from 12:00 UTC"
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM1200 6000")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.kind, MetarTrend::Becmg);
    assert_eq!(detail.times.len(), 1);
    assert_eq!(detail.times[0].kind, MetarTrendTimeKind::From);
    assert_eq!(detail.times[0].hour, 12);
    assert_eq!(detail.times[0].minute, 0);

    let vis = detail.visibility.unwrap();
    use metar_taf_parser::metar::models::visibility::Visibility;
    assert!(matches!(
        vis,
        Visibility::Single {
            prevailing: 6000,
            ..
        }
    ));
}

#[test]
fn parse_tempo_cloud_manual_example() {
    // "TEMPO BKN007 means that there is a temporary cloud layer at 700 ft"
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO BKN007").unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.kind, MetarTrend::Tempo);
    assert_eq!(detail.clouds.len(), 1);
    assert_eq!(detail.clouds[0].altitude_ft, Some(700));
}

// ---------------------------------------------------------------------------
// nosig flag
// ---------------------------------------------------------------------------

#[test]
fn nosig_flag_set_for_nosig_trend() {
    let m = parse_metar("METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG").unwrap();
    assert!(m.nosig);
    assert!(matches!(m.trend, Some(MetarTrend::Nosig)));
    assert!(m.trend_detail.is_none());
}

#[test]
fn nosig_flag_false_without_nosig() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(!m.nosig);
    assert!(m.trend.is_none());
}

// ---------------------------------------------------------------------------
// AT time indicator
// ---------------------------------------------------------------------------

#[test]
fn parse_tempo_at_time() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO AT1330 3000")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.times[0].kind, MetarTrendTimeKind::At);
    assert_eq!(detail.times[0].hour, 13);
    assert_eq!(detail.times[0].minute, 30);
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_nosig_trend() {
    let m = parse_metar("METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG").unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(trend.contains("no significant"), "{}", trend);
}

#[test]
fn describe_becmg_with_time_and_visibility() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM1200 6000")
        .unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(
        trend.contains("becoming") || trend.contains("BECMG"),
        "{}",
        trend
    );
    assert!(trend.contains("12:00Z"), "{}", trend);
    assert!(trend.contains("6000"), "{}", trend);
}

#[test]
fn describe_tempo_with_cloud() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO BKN007").unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(trend.contains("temporar"), "{}", trend);
    assert!(trend.contains("700 ft"), "{}", trend);
}

// ---------------------------------------------------------------------------
// PROB30/PROB40 are TAF-only — not a recognised METAR trend keyword
// ---------------------------------------------------------------------------

#[test]
fn prob30_not_a_metar_trend_keyword() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 PROB30 TEMPO 0200")
        .unwrap();
    // PROB30 is never set as the MetarTrend value (it is a TAF-only construct)
    // TEMPO that follows is still a valid METAR trend keyword
    assert!(
        matches!(m.trend, Some(MetarTrend::Tempo)),
        "TEMPO after PROB30 should still be the active trend, got: {:?}",
        m.trend
    );
}

#[test]
fn prob40_not_a_metar_trend_keyword() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 PROB40 TEMPO 0200")
        .unwrap();
    // Same as PROB30: never a MetarTrend value in METAR context
    assert!(
        matches!(m.trend, Some(MetarTrend::Tempo)),
        "TEMPO after PROB40 should still be the active trend, got: {:?}",
        m.trend
    );
}

// ---------------------------------------------------------------------------
// TL (until) time indicator
// ---------------------------------------------------------------------------

#[test]
fn parse_becmg_tl_only_time() {
    // BECMG with TL only (no FM): the change is complete by the TL time
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG TL1800 22015KT")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.kind, MetarTrend::Becmg);
    assert_eq!(detail.times.len(), 1);
    assert_eq!(detail.times[0].kind, MetarTrendTimeKind::Until);
    assert_eq!(detail.times[0].hour, 18);
    assert_eq!(detail.times[0].minute, 0);
    assert!(detail.wind.is_some());
}

// ---------------------------------------------------------------------------
// TEMPO with weather phenomena and wind
// ---------------------------------------------------------------------------

#[test]
fn parse_tempo_with_weather() {
    // TEMPO with a weather phenomenon (-RA)
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO -RA").unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.kind, MetarTrend::Tempo);
    assert_eq!(detail.weather.len(), 1);
    assert!(detail.clouds.is_empty());
}

#[test]
fn parse_tempo_with_wind() {
    // TEMPO carrying a new wind group
    let m =
        parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO 09010KT").unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(detail.kind, MetarTrend::Tempo);
    assert!(detail.wind.is_some());
    let wind = detail.wind.unwrap();
    assert_eq!(wind.direction, Some(90));
    assert_eq!(wind.speed, 10);
}

// ---------------------------------------------------------------------------
// Invalid trend-time tokens are not parsed as times
// ---------------------------------------------------------------------------

#[test]
fn invalid_trend_time_not_parsed_as_time() {
    // FM2499 has minute 99 which is out of range: must not be parsed as a trend time
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM2499 6000")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    assert!(
        detail.times.is_empty(),
        "FM2499 should not parse as a valid trend time, got: {:?}",
        detail.times
    );
    // Visibility after the invalid time token must still be parsed
    assert!(
        detail.visibility.is_some(),
        "visibility 6000 must be parsed even after invalid FM2499"
    );
}

// ---------------------------------------------------------------------------
// Trend parsing stops at RMK
// ---------------------------------------------------------------------------

#[test]
fn trend_stops_at_rmk() {
    // Tokens after RMK must not be consumed by the trend parser
    let m =
        parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM1300 6000 RMK AO2")
            .unwrap();

    let detail = m.trend_detail.unwrap();
    // RMK-related tokens must not appear in the trend's unparsed_groups
    assert!(
        !detail.unparsed_groups.contains(&"RMK".to_string()),
        "RMK must not be in trend unparsed_groups"
    );
    assert!(
        !detail.unparsed_groups.contains(&"AO2".to_string()),
        "AO2 (remark) must not be in trend unparsed_groups"
    );
    // The trend detail should still carry what appeared before RMK
    assert_eq!(detail.times.len(), 1);
    assert!(detail.visibility.is_some());
}

// ---------------------------------------------------------------------------
// raw_tokens is populated
// ---------------------------------------------------------------------------

#[test]
fn trend_raw_tokens_populated() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG FM1300 22015KT")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    assert!(
        !detail.raw_tokens.is_empty(),
        "raw_tokens should not be empty"
    );
    assert!(
        detail.raw_tokens.contains(&"FM1300".to_string()),
        "FM1300 should be in raw_tokens, got: {:?}",
        detail.raw_tokens
    );
}

// ---------------------------------------------------------------------------
// Additional describe coverage
// ---------------------------------------------------------------------------

#[test]
fn describe_becmg_with_wind() {
    let m =
        parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG 22015KT").unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(
        trend.contains("becoming") || trend.contains("BECMG"),
        "{}",
        trend
    );
    assert!(trend.contains("220"), "{}", trend);
    assert!(trend.contains("15"), "{}", trend);
}

#[test]
fn describe_becmg_with_tl_time() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG TL1800 22015KT")
        .unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(trend.contains("18:00Z"), "{}", trend);
}

#[test]
fn describe_tempo_with_weather() {
    let m = parse_metar("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO -RA").unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(trend.contains("temporar"), "{}", trend);
    // "-RA" should produce some mention of rain in the description
    assert!(trend.to_lowercase().contains("rain"), "{}", trend);
}
