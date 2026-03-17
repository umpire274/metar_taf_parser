use metar_taf_parser::{Language, describe_metar, parse_metar};

#[test]
fn describe_metar_station_and_time() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.station, "LIRF");
    assert_eq!(desc.time.unwrap(), "Day 12 at 12:50Z");
}

#[test]
fn describe_metar_wind_directional() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let wind = desc.wind.unwrap();
    assert!(wind.contains("180°"), "expected direction in: {}", wind);
    assert!(wind.contains("10"), "expected speed in: {}", wind);
    assert!(wind.contains("kt"), "expected unit in: {}", wind);
}

#[test]
fn describe_metar_wind_variable() {
    let metar = parse_metar("LIRF 121250Z VRB03KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let wind = desc.wind.unwrap();
    assert!(
        wind.contains("variable direction"),
        "expected VRB in: {}",
        wind
    );
}

#[test]
fn describe_metar_wind_with_gust() {
    let metar = parse_metar("LIRF 121250Z 18015G25KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let wind = desc.wind.unwrap();
    assert!(wind.contains("gusting"), "expected gust in: {}", wind);
    assert!(wind.contains("25"), "expected gust speed in: {}", wind);
}

#[test]
fn describe_metar_visibility_cavok() {
    let metar = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let vis = desc.visibility.unwrap();
    assert!(vis.contains("CAVOK"), "expected CAVOK in: {}", vis);
}

#[test]
fn describe_metar_visibility_greater_than_10km() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let vis = desc.visibility.unwrap();
    assert!(
        vis.contains("greater than 10 km"),
        "expected >10km in: {}",
        vis
    );
}

#[test]
fn describe_metar_visibility_metres() {
    let metar = parse_metar("LIRF 121250Z 18010KT 4000 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let vis = desc.visibility.unwrap();
    assert!(vis.contains("4000 m"), "expected metres in: {}", vis);
}

#[test]
fn describe_metar_clouds_few() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.clouds.len(), 1);
    let cloud = &desc.clouds[0];
    assert!(
        cloud.contains("few clouds"),
        "expected 'few clouds' in: {}",
        cloud
    );
    assert!(cloud.contains("3000 ft"), "expected altitude in: {}", cloud);
}

#[test]
fn describe_metar_clouds_with_cb() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 SCT025CB 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let cloud = &desc.clouds[0];
    assert!(
        cloud.contains("cumulonimbus"),
        "expected CB type in: {}",
        cloud
    );
}

#[test]
fn describe_metar_clouds_nsc() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 NSC 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.clouds.len(), 1);
    assert!(desc.clouds[0].contains("no significant clouds"));
}

#[test]
fn describe_metar_temperature() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let temp = desc.temperature.unwrap();
    assert!(temp.contains("18°C"), "expected temp in: {}", temp);
    assert!(temp.contains("12°C"), "expected dew point in: {}", temp);
}

#[test]
fn describe_metar_temperature_negative() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 M02/M08 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let temp = desc.temperature.unwrap();
    assert!(temp.contains("-2°C"), "expected negative temp in: {}", temp);
    assert!(
        temp.contains("-8°C"),
        "expected negative dew point in: {}",
        temp
    );
}

#[test]
fn describe_metar_pressure_qnh() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let pres = desc.pressure.unwrap();
    assert!(pres.contains("1015 hPa"), "expected QNH in: {}", pres);
}

#[test]
fn describe_metar_pressure_inhg() {
    let metar = parse_metar("KJFK 121250Z 18010KT 9999 FEW030 18/12 A2992").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let pres = desc.pressure.unwrap();
    assert!(pres.contains("inHg"), "expected inHg in: {}", pres);
}

#[test]
fn describe_metar_weather_light_rain() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.weather.len(), 1);
    let w = &desc.weather[0];
    assert!(w.contains("light"), "expected intensity in: {}", w);
    assert!(w.contains("rain"), "expected phenomenon in: {}", w);
}

#[test]
fn describe_metar_weather_heavy_thunderstorm_rain() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 +TSRA 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let w = &desc.weather[0];
    assert!(w.contains("heavy"), "expected intensity in: {}", w);
    assert!(w.contains("thunderstorm"), "expected descriptor in: {}", w);
    assert!(w.contains("rain"), "expected phenomenon in: {}", w);
}

#[test]
fn describe_metar_modifier_auto() {
    let metar = parse_metar("METAR LIRF 121250Z AUTO 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let modifier = desc.modifier.unwrap();
    assert!(
        modifier.contains("automated"),
        "expected AUTO in: {}",
        modifier
    );
}

#[test]
fn describe_metar_modifier_cor() {
    let metar = parse_metar("METAR COR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let modifier = desc.modifier.unwrap();
    assert!(
        modifier.contains("corrected"),
        "expected COR in: {}",
        modifier
    );
}

#[test]
fn describe_metar_no_modifier_for_normal() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert!(desc.modifier.is_none());
}

#[test]
fn describe_metar_trend_nosig() {
    let metar = parse_metar("LIRF 121250Z 18010KT CAVOK 18/12 Q1015 NOSIG").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let trend = desc.trend.unwrap();
    assert!(
        trend.contains("no significant change"),
        "expected NOSIG in: {}",
        trend
    );
}

#[test]
fn describe_metar_trend_tempo_with_time() {
    let metar =
        parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO AT1400 4000 -RA").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let trend = desc.trend.unwrap();
    assert!(
        trend.contains("temporarily"),
        "expected TEMPO in: {}",
        trend
    );
    assert!(trend.contains("14:00Z"), "expected time in: {}", trend);
}

#[test]
fn describe_metar_remarks_preserved() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 RMK AO2").unwrap();
    let desc = describe_metar(&metar, Language::En);

    let rmk = desc.remarks.unwrap();
    assert!(rmk.contains("AO2"), "expected RMK text in: {}", rmk);
}

#[test]
fn describe_metar_runway_state_included() {
    let metar =
        parse_metar("METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG").unwrap();
    let desc = describe_metar(&metar, Language::En);

    assert_eq!(desc.runway_state.len(), 1);
    let rs = &desc.runway_state[0];
    assert!(rs.contains("runway 19"), "expected designator in: {}", rs);
    assert!(rs.contains("dry snow"), "expected deposit type in: {}", rs);
    assert!(rs.contains("26–50%"), "expected coverage in: {}", rs);
}

#[test]
fn describe_metar_runway_state_in_display() {
    let metar =
        parse_metar("METAR UOOO 191400Z 00000MPS CAVOK M28/M31 Q1020 R19/450235 NOSIG").unwrap();
    let text = metar_taf_parser::format_metar(&metar, Language::En);
    assert!(
        text.contains("Runway:"),
        "expected Runway line in:\n{}",
        text
    );
    assert!(
        text.contains("runway 19"),
        "expected designator in:\n{}",
        text
    );
}

#[test]
fn describe_metar_no_runway_state_when_absent() {
    let metar = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    let desc = describe_metar(&metar, Language::En);
    assert!(desc.runway_state.is_empty());
}
