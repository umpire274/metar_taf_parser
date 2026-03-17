use metar_taf_parser::metar::models::visibility::Visibility;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

#[test]
fn metar_visibility_cavok() {
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    assert!(matches!(m.visibility.unwrap(), Visibility::CAVOK));
}

#[test]
fn cavok_clears_clouds() {
    // Any cloud group before CAVOK in a malformed feed must be removed
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    assert!(m.clouds.is_empty(), "clouds should be empty with CAVOK");
}

#[test]
fn cavok_clears_weather() {
    // Any weather group before CAVOK in a malformed feed must be removed
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    assert!(m.weather.is_empty(), "weather should be empty with CAVOK");
}

#[test]
fn cavok_full_example_from_manual() {
    // METAR EHLE 280925Z 21009G19KT 060V130 CAVOK 02/M01 Q1001
    let m = parse_metar("METAR EHLE 280925Z 21009G19KT 060V130 CAVOK 02/M01 Q1001").unwrap();

    assert_eq!(m.station, "EHLE");
    assert!(matches!(m.visibility.unwrap(), Visibility::CAVOK));
    assert!(m.clouds.is_empty());
    assert!(m.weather.is_empty());

    let wind = m.wind.unwrap();
    assert_eq!(wind.direction, Some(210));
    assert_eq!(wind.speed, 9);
    assert_eq!(wind.gust, Some(19));

    let variation = wind.variation.unwrap();
    assert_eq!(variation.min, 60);
    assert_eq!(variation.max, 130);

    let temp = m.temperature.unwrap();
    assert_eq!(temp.temperature, 2);
    assert_eq!(temp.dew_point, -1);
}

#[test]
fn cavok_temperature_parsing() {
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    let temp = m.temperature.unwrap();
    assert_eq!(temp.temperature, 18);
    assert_eq!(temp.dew_point, 12);
}

#[test]
fn cavok_pressure_parsing() {
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    use metar_taf_parser::metar::models::pressure::Pressure;
    assert!(matches!(m.pressure.unwrap(), Pressure::QnhHpa(1015)));
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_cavok_visibility() {
    let m = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    let desc = describe_metar(&m, Language::En);
    let vis = desc.visibility.unwrap();
    assert!(vis.contains("CAVOK"), "{}", vis);
    assert!(
        desc.clouds.is_empty(),
        "no cloud entries expected with CAVOK"
    );
}

#[test]
fn describe_cavok_full_example() {
    let m = parse_metar("METAR EHLE 280925Z 21009G19KT 060V130 CAVOK 02/M01 Q1001").unwrap();
    let desc = describe_metar(&m, Language::En);

    assert_eq!(desc.station, "EHLE");
    assert!(desc.visibility.as_deref().unwrap_or("").contains("CAVOK"));

    let wind = desc.wind.unwrap();
    assert!(wind.contains("210°"), "{}", wind);
    assert!(wind.contains("gusting"), "{}", wind);
    assert!(wind.contains("variable 60 to 130°"), "{}", wind);
}
