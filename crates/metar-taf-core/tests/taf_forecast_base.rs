use metar_taf_core::metar::models::cloud::CloudAmount;
use metar_taf_core::metar::models::visibility::Visibility;
use metar_taf_core::parse_taf;

#[test]
fn taf_base_forecast_parsing() {
    let input = "TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 SCT080";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 1);

    let fc = &taf.forecasts[0];

    let wind = fc.wind.as_ref().expect("wind missing");
    assert_eq!(wind.direction, Some(180));
    assert_eq!(wind.speed_kt, 10);

    let vis = fc.visibility.as_ref().expect("visibility missing");
    assert!(matches!(vis, Visibility::Single { prevailing: 9999 }));

    assert_eq!(fc.clouds.len(), 2);
    assert!(matches!(fc.clouds[0].amount, CloudAmount::FEW));
    assert!(matches!(fc.clouds[1].amount, CloudAmount::SCT));
}
