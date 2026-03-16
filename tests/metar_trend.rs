use metar_taf_parser::metar::models::trend::{MetarTrend, MetarTrendTimeKind};
use metar_taf_parser::parse_metar;

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
