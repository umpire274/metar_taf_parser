use metar_taf_parser::{describe_taf, parse_taf, Language};

#[test]
fn describe_taf_station_and_times() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&taf, Language::En);

    assert_eq!(desc.station, "LIRF");
    assert_eq!(desc.issued_at.unwrap(), "Day 12 at 11:00Z");
    assert_eq!(desc.validity.unwrap(), "12/12Z to 13/18Z");
}

#[test]
fn describe_taf_base_forecast_wind_and_visibility() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&taf, Language::En);

    let base = &desc.forecasts[0];
    assert_eq!(base.kind, "Base forecast");

    let wind = base.wind.as_ref().unwrap();
    assert!(wind.contains("180°"), "expected direction in: {}", wind);
    assert!(wind.contains("10"), "expected speed in: {}", wind);

    let vis = base.visibility.as_ref().unwrap();
    assert!(vis.contains("greater than 10 km"), "expected >10km in: {}", vis);
}

#[test]
fn describe_taf_base_forecast_clouds() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
    let desc = describe_taf(&taf, Language::En);

    let base = &desc.forecasts[0];
    assert_eq!(base.clouds.len(), 1);
    assert!(base.clouds[0].contains("scattered clouds"));
    assert!(base.clouds[0].contains("2000 ft"));
}

#[test]
fn describe_taf_tempo_block() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TEMPO 1218/1222 4000 -RA",
    )
    .unwrap();
    let desc = describe_taf(&taf, Language::En);

    let tempo = desc.forecasts.iter().find(|f| f.kind == "Temporary").unwrap();
    let period = tempo.period.as_ref().unwrap();
    assert!(period.contains("12/18Z"), "expected from in: {}", period);
    assert!(period.contains("12/22Z"), "expected to in: {}", period);

    assert!(!tempo.weather.is_empty());
    let w = &tempo.weather[0];
    assert!(w.contains("light") && w.contains("rain"), "expected -RA in: {}", w);
}

#[test]
fn describe_taf_becmg_block() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 BECMG 1215/1217 24015KT",
    )
    .unwrap();
    let desc = describe_taf(&taf, Language::En);

    let becmg = desc.forecasts.iter().find(|f| f.kind == "Becoming").unwrap();
    let wind = becmg.wind.as_ref().unwrap();
    assert!(wind.contains("240°"), "expected direction in: {}", wind);
}

#[test]
fn describe_taf_fm_block() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 FM121800 24020KT 9999 BKN040",
    )
    .unwrap();
    let desc = describe_taf(&taf, Language::En);

    let fm = desc.forecasts.iter().find(|f| f.kind == "From").unwrap();
    let period = fm.period.as_ref().unwrap();
    assert!(period.contains("12"), "expected day in: {}", period);
    assert!(period.contains("18:00Z"), "expected time in: {}", period);
}

#[test]
fn describe_taf_prob30_tempo_block() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 PROB30 TEMPO 1214/1216 2000 TSRA",
    )
    .unwrap();
    let desc = describe_taf(&taf, Language::En);

    // PROB30 TEMPO is represented as TafForecastKind::PROB with probability 30
    let prob = desc
        .forecasts
        .iter()
        .find(|f| f.kind == "Probability")
        .unwrap();
    let probability = prob.probability.as_ref().unwrap();
    assert_eq!(probability, "30%");
}

#[test]
fn describe_taf_temperatures() {
    let taf = parse_taf(
        "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TX18/1214Z TN08/1304Z",
    )
    .unwrap();
    let desc = describe_taf(&taf, Language::En);

    let base = &desc.forecasts[0];
    let max_temp = base.max_temperature.as_ref().unwrap();
    assert!(max_temp.contains("18°C"), "expected max temp in: {}", max_temp);

    let min_temp = base.min_temperature.as_ref().unwrap();
    assert!(min_temp.contains("8°C"), "expected min temp in: {}", min_temp);
}

#[test]
fn describe_taf_modifier_amd() {
    let taf = parse_taf("TAF AMD EDDF 181333Z 1813/1918 18005KT 9999 SCT025").unwrap();
    let desc = describe_taf(&taf, Language::En);

    let modifier = desc.modifier.unwrap();
    assert!(modifier.contains("amended"), "expected AMD in: {}", modifier);
}

#[test]
fn describe_taf_wind_shear() {
    let taf =
        parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 WS020/25040KT").unwrap();
    let desc = describe_taf(&taf, Language::En);

    let base = &desc.forecasts[0];
    let ws = base.wind_shear.as_ref().unwrap();
    assert!(ws.contains("2000 ft"), "expected height in: {}", ws);
    assert!(ws.contains("250°"), "expected direction in: {}", ws);
    assert!(ws.contains("40 kt"), "expected speed in: {}", ws);
}

#[test]
fn describe_taf_cavok_base() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT CAVOK").unwrap();
    let desc = describe_taf(&taf, Language::En);

    let base = &desc.forecasts[0];
    let vis = base.visibility.as_ref().unwrap();
    assert!(vis.contains("CAVOK"), "expected CAVOK in: {}", vis);
}

