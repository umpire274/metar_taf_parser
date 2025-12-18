use metar_taf_core::parse_metar;

#[test]
fn metar_basic_parsing() {
    let input = "LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015";

    let metar = parse_metar(input).expect("METAR should parse");

    assert_eq!(metar.station, "LIRF");

    let time = metar.time.expect("time missing");
    assert_eq!(time.day, 12);
    assert_eq!(time.hour, 12);
    assert_eq!(time.minute, 50);

    let wind = metar.wind.expect("wind missing");
    assert_eq!(wind.direction, Some(180));
    assert_eq!(wind.speed, 10);
    assert_eq!(wind.gust, None);

    let temp = metar.temperature.expect("temperature missing");
    assert_eq!(temp.temperature, 18);
    assert_eq!(temp.dew_point, 12);

    let pressure = metar.pressure.expect("pressure missing");
    match pressure {
        metar_taf_core::metar::models::pressure::Pressure::QnhHpa(v) => {
            assert_eq!(v, 1015);
        }
        _ => panic!("unexpected pressure type"),
    }
}
