//! Integration tests for TAF Gruppo 8 — gruppi probabilità (PROB30, PROB40).
//!
//! - `PROB30 DDHH/DDHH` — c'è una probabilità del 30% che le condizioni descritte
//!   si verifichino nel periodo indicato.
//! - `PROB40 DDHH/DDHH` — c'è una probabilità del 40% che le condizioni descritte
//!   si verifichino nel periodo indicato.
//! - `PROB40 TEMPO DDHH/DDHH` — probabilità del 40% associata a variazioni temporanee.
//!
//! Solo probabilità del 30% e 40% sono usate nei TAF; quando la probabilità è ≥50%
//! si ricorre a FM, BECMG o TEMPO, quindi PROB50 e superiori non sono validi.

use metar_taf_parser::metar::models::weather::{WeatherDescriptor, WeatherPhenomenon};
use metar_taf_parser::taf::models::forecast::TafForecastKind;
use metar_taf_parser::{Language, describe_taf, parse_taf};

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
        metar_taf_parser::metar::models::visibility::Visibility::Single { prevailing, .. } => {
            assert_eq!(*prevailing, 3000);
        }
        _ => panic!("unexpected visibility"),
    }

    assert_eq!(prob.weather.len(), 1);
    let weather = &prob.weather[0];
    assert!(
        weather
            .descriptors
            .iter()
            .any(|d| matches!(d, WeatherDescriptor::Thunderstorm))
    );
    assert!(
        weather
            .phenomena
            .iter()
            .any(|p| matches!(p, WeatherPhenomenon::Rain))
    );
}

#[test]
fn taf_with_prob40_without_tempo() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
PROB40 1220/1223 4000 SHRA BKN020";

    let taf = parse_taf(input).expect("TAF should parse");

    assert_eq!(taf.forecasts.len(), 2);
    let prob = &taf.forecasts[1];
    assert_eq!(prob.kind, TafForecastKind::PROB);
    assert_eq!(prob.probability, Some(40));

    let period = prob.period.as_ref().expect("period missing");
    assert_eq!(period.from, (12, 20, 0));
    assert_eq!(period.to, (12, 23, 0));

    assert_eq!(prob.weather.len(), 1);
    assert!(
        prob.weather[0]
            .descriptors
            .iter()
            .any(|d| matches!(d, WeatherDescriptor::Showers))
    );
    assert!(
        prob.weather[0]
            .phenomena
            .iter()
            .any(|p| matches!(p, WeatherPhenomenon::Rain))
    );
}

#[test]
fn taf_prob_with_invalid_period_does_not_consume_following_tokens() {
    let input = "\
TAF LIRF 121100Z 1212/1318
18010KT 9999 FEW030
PROB30 99AA/1223 20012KT SCT040";

    let taf = parse_taf(input).expect("TAF should parse");

    // Invalid PROB period should not open a new forecast; following wind/clouds stay in base
    assert_eq!(taf.forecasts.len(), 1);

    let base = &taf.forecasts[0];
    let wind = base.wind.as_ref().expect("wind should still be parsed");
    assert_eq!(wind.direction, Some(200));
    assert_eq!(wind.speed, 12);
    assert!(!base.clouds.is_empty());
}

#[test]
fn taf_prob_parses_nsw_weather_payload() {
    let input = "TAF LIRF 121100Z 1212/1318 18010KT 9999 FEW030 PROB40 1220/1223 NSW";

    let taf = parse_taf(input).expect("TAF should parse");
    let prob = &taf.forecasts[1];

    assert!(
        prob.weather
            .iter()
            .flat_map(|w| w.phenomena.iter())
            .any(|p| matches!(p, WeatherPhenomenon::NoSignificantWeather))
    );
}

// ===========================================================================
// Esempio dal manuale
// ===========================================================================

#[test]
fn prob40_manual_example_period() {
    // Esempio: PROB40 2510/2513 — probabilità 40% tra 10:00Z e 13:00Z del 25°
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2510/2513 3000 TSRA BKN010CB",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 2);

    let prob = &t.forecasts[1];
    assert_eq!(prob.kind, TafForecastKind::PROB);
    assert_eq!(prob.probability, Some(40));

    let p = prob.period.as_ref().unwrap();
    assert_eq!(p.from, (25, 10, 0));
    assert_eq!(p.to, (25, 13, 0));
}

// ===========================================================================
// Validazione: solo PROB30 e PROB40 sono ammessi
// ===========================================================================

#[test]
fn prob50_is_not_recognized() {
    // PROB50 non è un valore valido nei TAF: il token non deve aprire un blocco PROB.
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB50 2514/2516 TSRA").unwrap();
    // Nessun blocco PROB deve essere creato; rimane solo il blocco base.
    assert_eq!(
        t.forecasts.len(),
        1,
        "PROB50 non deve creare un blocco PROB"
    );
}

#[test]
fn prob30_is_recognized() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB30 2514/2516 TSRA").unwrap();
    assert_eq!(t.forecasts.len(), 2);
    assert_eq!(t.forecasts[1].probability, Some(30));
}

#[test]
fn prob40_is_recognized() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2514/2516 TSRA").unwrap();
    assert_eq!(t.forecasts.len(), 2);
    assert_eq!(t.forecasts[1].probability, Some(40));
}

// ===========================================================================
// Casi aggiuntivi
// ===========================================================================

#[test]
fn prob30_crosses_midnight() {
    // PROB30 2523/2601 — probabilità che attraversa la mezzanotte
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB30 2523/2601 TSRA").unwrap();
    let p = t.forecasts[1].period.as_ref().unwrap();
    assert_eq!(p.from, (25, 23, 0));
    assert_eq!(p.to, (26, 1, 0));
}

#[test]
fn prob_combined_with_other_change_groups() {
    // TAF con FM + PROB40 + TEMPO nello stesso messaggio
    let t = parse_taf(
        "TAF LIRF 251100Z 2512/2618 \
         18010KT 9999 SCT020 \
         FM251500 22010KT 9999 FEW030 \
         PROB40 2518/2520 3000 TSRA BKN010CB \
         TEMPO 2600/2603 -RA",
    )
    .unwrap();
    assert_eq!(t.forecasts.len(), 4);
    assert_eq!(t.forecasts[0].kind, TafForecastKind::Base);
    assert_eq!(t.forecasts[1].kind, TafForecastKind::FM);
    assert_eq!(t.forecasts[2].kind, TafForecastKind::PROB);
    assert_eq!(t.forecasts[3].kind, TafForecastKind::TEMPO);
    // Verifica che il valore di probabilità sia corretto
    assert_eq!(t.forecasts[2].probability, Some(40));
}

#[test]
fn prob_unmodified_elements_are_absent() {
    // Elementi non descritti nel blocco PROB non devono essere presenti
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2514/2516 TSRA").unwrap();
    let prob = &t.forecasts[1];
    assert!(
        prob.wind.is_none(),
        "vento non modificato deve essere assente nel blocco PROB"
    );
    assert!(
        prob.visibility.is_none(),
        "visibilità non modificata deve essere assente nel blocco PROB"
    );
    assert!(
        prob.clouds.is_empty(),
        "nubi non modificate devono essere assenti nel blocco PROB"
    );
}

// ===========================================================================
// Describe
// ===========================================================================

#[test]
fn describe_prob_kind_label() {
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2514/2516 TSRA").unwrap();
    let desc = describe_taf(&t, Language::En);
    let kind = &desc.forecasts[1].kind;
    assert!(
        kind.contains("Prob") || kind.contains("prob"),
        "expected PROB kind label in: {kind}"
    );
}

#[test]
fn describe_prob_probability_field() {
    // Il campo describe.probability deve contenere la percentuale
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2514/2516 TSRA").unwrap();
    let desc = describe_taf(&t, Language::En);
    let prob_str = desc.forecasts[1]
        .probability
        .as_ref()
        .expect("probability field should be present");
    assert!(
        prob_str.contains("40"),
        "expected '40' in probability string: {prob_str}"
    );
}

#[test]
fn describe_prob_period_field() {
    // Il campo describe.period deve contenere il periodo del blocco PROB
    let t =
        parse_taf("TAF LIRF 251100Z 2512/2618 18010KT 9999 SCT020 PROB40 2510/2513 TSRA").unwrap();
    let desc = describe_taf(&t, Language::En);
    let period = desc.forecasts[1]
        .period
        .as_ref()
        .expect("period should be present");
    assert!(
        period.contains("25"),
        "expected day 25 in period string: {period}"
    );
}
