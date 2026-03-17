//! Integration tests for TAF Gruppo 5 — fenomeni del tempo significativo.
//!
//! Formato: `[intensità][descrittore(i)][fenomeno(i)]`
//! - Intensità: `-` leggero, assente = moderato, `+` forte, `VC` nei dintorni
//! - Descrittori: TS SH FZ BL DR MI BC PR
//! - Precipitazioni: DZ RA SN SG PL IC GR GS
//! - Riduttori della visibilità: BR FG SA DU HZ
//! - Altri fenomeni: SQ FC DS SS
//! - NSW (Nil Significant Weather): termine dei fenomeni nei gruppi evolutivi

use metar_taf_parser::metar::models::weather::{
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
};
use metar_taf_parser::taf::models::forecast::TafForecastKind;
use metar_taf_parser::{Language, describe_taf, parse_taf};

// ---------------------------------------------------------------------------
// Intensità
// ---------------------------------------------------------------------------

#[test]
fn weather_moderate_rain_no_prefix() {
    // RA senza prefisso = intensità moderata (intensity: None)
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 RA BKN010").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert_eq!(w.phenomena, vec![WeatherPhenomenon::Rain]);
    assert!(w.intensity.is_none(), "moderata → intensity assente");
}

#[test]
fn weather_light_rain() {
    // -RA
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 -RA BKN010").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert_eq!(w.intensity, Some(WeatherIntensity::Light));
    assert_eq!(w.phenomena, vec![WeatherPhenomenon::Rain]);
}

#[test]
fn weather_heavy_rain() {
    // +RA
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 2000 +RA OVC010").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert_eq!(w.intensity, Some(WeatherIntensity::Heavy));
    assert_eq!(w.phenomena, vec![WeatherPhenomenon::Rain]);
}

// ---------------------------------------------------------------------------
// Descrittori
// ---------------------------------------------------------------------------

#[test]
fn weather_descriptor_ts_thunderstorm() {
    // TSRA — temporale con pioggia
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 3000 TSRA SCT020CB").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.descriptors.contains(&WeatherDescriptor::Thunderstorm));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Rain));
}

#[test]
fn weather_descriptor_sh_showers() {
    // SHSN — rovesci di neve
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 SHSN BKN015").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.descriptors.contains(&WeatherDescriptor::Showers));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Snow));
}

#[test]
fn weather_descriptor_fz_freezing_fog() {
    // FZFG — nebbia congelante
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 00000KT 0200 FZFG OVC001").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.descriptors.contains(&WeatherDescriptor::Freezing));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Fog));
}

#[test]
fn weather_descriptor_bl_blowing_snow() {
    // BLSN — neve soffiata alta
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 30025KT 2000 BLSN BKN008").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.descriptors.contains(&WeatherDescriptor::Blowing));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Snow));
}

#[test]
fn weather_standalone_ts_is_thunder() {
    // TS senza fenomeno → Thunder isolato
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 TS SCT020CB").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.phenomena.contains(&WeatherPhenomenon::Thunder));
    assert!(w.descriptors.is_empty());
}

// ---------------------------------------------------------------------------
// Tutti i fenomeni della tabella ICAO
// ---------------------------------------------------------------------------

#[test]
fn weather_all_precipitation_codes() {
    let cases = [
        ("DZ", WeatherPhenomenon::Drizzle),
        ("RA", WeatherPhenomenon::Rain),
        ("SN", WeatherPhenomenon::Snow),
        ("SG", WeatherPhenomenon::SnowGrains),
        ("PL", WeatherPhenomenon::IcePellets),
        ("IC", WeatherPhenomenon::IceCrystals),
        ("GR", WeatherPhenomenon::Hail),
        ("GS", WeatherPhenomenon::SmallHail),
    ];
    for (code, expected) in cases {
        let input = format!("TAF LIRF 251100Z 2512/2618 18010KT 5000 {} BKN010", code);
        let t = parse_taf(&input).unwrap();
        let w = &t.forecasts[0].weather[0];
        assert!(
            w.phenomena.contains(&expected),
            "code {code} non ha prodotto il fenomeno atteso"
        );
    }
}

#[test]
fn weather_all_obscuration_codes() {
    let cases = [
        ("BR", WeatherPhenomenon::Mist),
        ("FG", WeatherPhenomenon::Fog),
        ("SA", WeatherPhenomenon::Sand),
        ("DU", WeatherPhenomenon::Dust),
        ("HZ", WeatherPhenomenon::Haze),
    ];
    for (code, expected) in cases {
        let input = format!("TAF LIRF 251100Z 2512/2618 18010KT 5000 {} SCT020", code);
        let t = parse_taf(&input).unwrap();
        let w = &t.forecasts[0].weather[0];
        assert!(
            w.phenomena.contains(&expected),
            "code {code} non ha prodotto il fenomeno atteso"
        );
    }
}

#[test]
fn weather_all_other_phenomena_codes() {
    let cases = [
        ("SQ", WeatherPhenomenon::Squalls),
        ("FC", WeatherPhenomenon::FunnelCloud),
        ("DS", WeatherPhenomenon::DustStorm),
        ("SS", WeatherPhenomenon::SandStorm),
    ];
    for (code, expected) in cases {
        let input = format!("TAF LIRF 251100Z 2512/2618 18010KT 3000 {} BKN010", code);
        let t = parse_taf(&input).unwrap();
        let w = &t.forecasts[0].weather[0];
        assert!(
            w.phenomena.contains(&expected),
            "code {code} non ha prodotto il fenomeno atteso"
        );
    }
}

// ---------------------------------------------------------------------------
// Fenomeni multipli nello stesso token
// ---------------------------------------------------------------------------

#[test]
fn weather_multiple_phenomena_rasn() {
    // RASN — pioggia e neve miste
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 3000 RASN BKN010").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.phenomena.contains(&WeatherPhenomenon::Rain));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Snow));
}

#[test]
fn weather_multiple_phenomena_with_intensity() {
    // -RASN
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 4000 -RASN BKN010").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert_eq!(w.intensity, Some(WeatherIntensity::Light));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Rain));
    assert!(w.phenomena.contains(&WeatherPhenomenon::Snow));
}

// ---------------------------------------------------------------------------
// NSW — Nil Significant Weather
// ---------------------------------------------------------------------------

#[test]
fn weather_nsw_no_significant_weather() {
    // NSW come token autonomo in un blocco base
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 NSW SCT040").unwrap();
    let w = &t.forecasts[0].weather[0];
    assert!(w.phenomena.contains(&WeatherPhenomenon::NoSignificantWeather));
    assert!(w.intensity.is_none());
    assert!(w.descriptors.is_empty());
}

#[test]
fn weather_nsw_in_becmg() {
    // NSW in BECMG indica termine dei fenomeni
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 5000 -RA BKN010 BECMG 2516/2518 9999 NSW SCT020",
    )
    .unwrap();
    let becmg = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::BECMG))
        .unwrap();
    assert_eq!(becmg.weather.len(), 1);
    assert!(becmg.weather[0]
        .phenomena
        .contains(&WeatherPhenomenon::NoSignificantWeather));
}

#[test]
fn weather_nsw_in_tempo() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 5000 -RA BKN010 TEMPO 2514/2516 9999 NSW",
    )
    .unwrap();
    let tempo = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::TEMPO))
        .unwrap();
    assert!(tempo.weather[0]
        .phenomena
        .contains(&WeatherPhenomenon::NoSignificantWeather));
}

#[test]
fn weather_nsw_in_fm() {
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 5000 -RA BKN010 FM251800 18005KT 9999 NSW SCT040",
    )
    .unwrap();
    let fm = t
        .forecasts
        .iter()
        .find(|f| matches!(f.kind, TafForecastKind::FM))
        .unwrap();
    assert!(fm.weather[0]
        .phenomena
        .contains(&WeatherPhenomenon::NoSignificantWeather));
}

// ---------------------------------------------------------------------------
// Assenza di fenomeni
// ---------------------------------------------------------------------------

#[test]
fn weather_absent_when_not_reported() {
    // Un blocco senza fenomeni deve avere il Vec vuoto
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020").unwrap();
    assert!(t.forecasts[0].weather.is_empty());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_weather_light_rain() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 5000 -RA BKN010").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wx = &desc.forecasts[0].weather[0];
    assert!(wx.contains("light"), "expected 'light' in: {wx}");
    assert!(wx.contains("rain"), "expected 'rain' in: {wx}");
}

#[test]
fn describe_weather_heavy_rain() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 2000 +RA OVC010").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wx = &desc.forecasts[0].weather[0];
    assert!(wx.contains("heavy"), "expected 'heavy' in: {wx}");
}

#[test]
fn describe_weather_tsra() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 3000 TSRA SCT020CB").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wx = &desc.forecasts[0].weather[0];
    assert!(wx.contains("thunderstorm"), "expected 'thunderstorm' in: {wx}");
    assert!(wx.contains("rain"), "expected 'rain' in: {wx}");
}

#[test]
fn describe_weather_nsw() {
    let t = parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 NSW SCT040").unwrap();
    let desc = describe_taf(&t, Language::En);
    let wx = &desc.forecasts[0].weather[0];
    assert!(
        wx.contains("significant") || wx.contains("NSW"),
        "expected NSW description in: {wx}"
    );
}

