use metar_taf_parser::parse_taf;
use metar_taf_parser::taf::models::forecast::TafForecastKind;

#[test]
fn taf_with_becmg_group() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
BECMG 1220/1222 20012KT SCT040";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 2);

    let becmg = &taf.forecasts[1];

    assert_eq!(becmg.kind, TafForecastKind::BECMG);

    let period = becmg.period.as_ref().expect("BECMG period missing");
    assert_eq!(period.from, (12, 20, 0));
    assert_eq!(period.to, (12, 22, 0));

    let wind = becmg.wind.as_ref().unwrap();
    assert_eq!(wind.direction, Some(200));
    assert_eq!(wind.speed, 12);
}

#[test]
fn taf_with_invalid_becmg_period_does_not_consume_following_tokens() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
BECMG 12A0/1222 20012KT SCT040";

    let taf = parse_taf(input).expect("TAF should parse");

    // invalid BECMG period should not open a new forecast
    assert_eq!(taf.forecasts.len(), 1);

    let base = &taf.forecasts[0];
    let wind = base.wind.as_ref().expect("wind should still be parsed");
    assert_eq!(wind.direction, Some(200));
    assert_eq!(wind.speed, 12);
    assert!(!base.clouds.is_empty());
}
