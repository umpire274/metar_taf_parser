use metar_taf_core::parse_taf;
use metar_taf_core::taf::models::forecast::TafForecastKind;

#[test]
fn taf_with_tempo_group() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
TEMPO 1220/1222 4000 -RA BKN015";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 2);

    let tempo = &taf.forecasts[1];
    assert_eq!(tempo.kind, TafForecastKind::TEMPO);

    let period = tempo.period.as_ref().expect("TEMPO period missing");
    assert_eq!(period.from, (12, 20, 0));
    assert_eq!(period.to, (12, 22, 0));

    let vis = tempo.visibility.as_ref().expect("visibility missing");
    // 4000 metri
    match vis {
        metar_taf_core::metar::models::visibility::Visibility::Single { prevailing } => {
            assert_eq!(*prevailing, 4000);
        }
        _ => panic!("unexpected visibility type"),
    }

    assert!(!tempo.clouds.is_empty());
}

#[test]
fn taf_with_invalid_tempo_period_does_not_consume_following_tokens() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
TEMPO 99AA/1222 21015KT SCT030";

    let taf = parse_taf(input).expect("TAF should parse");

    // invalid TEMPO period should not open a new forecast
    assert_eq!(taf.forecasts.len(), 1);

    let base = &taf.forecasts[0];
    let wind = base.wind.as_ref().expect("wind should still be parsed");
    assert_eq!(wind.direction, Some(210));
    assert_eq!(wind.speed, 15);
    assert!(!base.clouds.is_empty());
}
