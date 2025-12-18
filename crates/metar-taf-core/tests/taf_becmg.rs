use metar_taf_core::parse_taf;
use metar_taf_core::taf::models::forecast::TafForecastKind;

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
