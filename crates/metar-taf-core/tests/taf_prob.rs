use metar_taf_core::parse_taf;
use metar_taf_core::taf::models::forecast::TafForecastKind;

#[test]
fn taf_with_prob30_tempo() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
PROB30 TEMPO 1220/1224 3000 TSRA BKN010";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 2);

    let prob = &taf.forecasts[1];
    assert_eq!(prob.kind, TafForecastKind::PROB);
    assert_eq!(prob.probability, Some(30));

    let period = prob.period.as_ref().expect("period missing");
    assert_eq!(period.from, (12, 20, 0));
    assert_eq!(period.to, (12, 24, 0));

    let vis = prob.visibility.as_ref().expect("visibility missing");
    match vis {
        metar_taf_core::metar::models::visibility::Visibility::Single { prevailing } => {
            assert_eq!(*prevailing, 3000);
        }
        _ => panic!("unexpected visibility"),
    }
}
