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
