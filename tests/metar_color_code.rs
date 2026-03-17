use metar_taf_parser::metar::models::color_code::{MilitaryColor, MilitaryColorCode};
use metar_taf_parser::metar::models::trend::MetarTrend;
use metar_taf_parser::{Language, describe_metar, parse_metar};

// ---------------------------------------------------------------------------
// Parse: plain color codes in the METAR body
// ---------------------------------------------------------------------------

#[test]
fn parse_single_color_code_grn() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN").unwrap();

    assert_eq!(
        m.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Grn,
            black: false
        })
    );
    assert!(m.color_code_forecast.is_none());
}

#[test]
fn parse_all_plain_color_codes() {
    for (token, expected) in [
        ("BLU", MilitaryColorCode::Blu),
        ("WHT", MilitaryColorCode::Wht),
        ("GRN", MilitaryColorCode::Grn),
        ("YLO", MilitaryColorCode::Ylo),
        ("AMB", MilitaryColorCode::Amb),
        ("RED", MilitaryColorCode::Red),
    ] {
        let input = format!(
            "METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 {}",
            token
        );
        let m = parse_metar(&input).unwrap();
        assert_eq!(
            m.color_code.as_ref().map(|c| &c.code),
            Some(&expected),
            "failed for token {}",
            token
        );
        assert!(!m.color_code.unwrap().black);
    }
}

// ---------------------------------------------------------------------------
// Parse: BLACK prefix (closed field)
// ---------------------------------------------------------------------------

#[test]
fn parse_black_color_code() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 BLACKGRN").unwrap();

    let color = m.color_code.unwrap();
    assert_eq!(color.code, MilitaryColorCode::Grn);
    assert!(color.black, "BLACK prefix must set black = true");
}

#[test]
fn parse_black_amb_color_code() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 1200 OVC005 15/10 Q1013 BLACKAMB").unwrap();

    let color = m.color_code.unwrap();
    assert_eq!(color.code, MilitaryColorCode::Amb);
    assert!(color.black);
}

// ---------------------------------------------------------------------------
// Parse: implicit forecast (second bare color code)
// ---------------------------------------------------------------------------

#[test]
fn parse_two_color_codes_current_and_forecast() {
    // GRN = current, BLU = implicit BECMG forecast
    let m = parse_metar("METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 GRN BLU").unwrap();

    assert_eq!(
        m.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Grn,
            black: false
        })
    );
    assert_eq!(
        m.color_code_forecast,
        Some(MilitaryColor {
            code: MilitaryColorCode::Blu,
            black: false
        })
    );
}

#[test]
fn parse_current_and_forecast_with_tempo() {
    // GRN current, AMB implicit forecast, TEMPO RED
    let m = parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN AMB TEMPO RED")
        .unwrap();

    assert_eq!(
        m.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Grn,
            black: false
        })
    );
    assert_eq!(
        m.color_code_forecast,
        Some(MilitaryColor {
            code: MilitaryColorCode::Amb,
            black: false
        })
    );
    assert!(matches!(m.trend, Some(MetarTrend::Tempo)));
    let detail = m.trend_detail.unwrap();
    assert_eq!(
        detail.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Red,
            black: false
        })
    );
}

// ---------------------------------------------------------------------------
// Parse: color code inside explicit BECMG / TEMPO trend
// ---------------------------------------------------------------------------

#[test]
fn parse_color_code_in_becmg_trend() {
    let m =
        parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN BECMG BLU").unwrap();

    assert_eq!(
        m.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Grn,
            black: false
        })
    );
    assert!(matches!(m.trend, Some(MetarTrend::Becmg)));
    let detail = m.trend_detail.unwrap();
    assert_eq!(
        detail.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Blu,
            black: false
        })
    );
}

#[test]
fn parse_color_code_in_tempo_trend() {
    let m =
        parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN TEMPO AMB").unwrap();

    let detail = m.trend_detail.unwrap();
    assert_eq!(
        detail.color_code,
        Some(MilitaryColor {
            code: MilitaryColorCode::Amb,
            black: false
        })
    );
}

#[test]
fn parse_black_color_code_in_tempo_trend() {
    // TEMPO BLACKRED: temporarily closed due to RED conditions
    let m = parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN TEMPO BLACKRED")
        .unwrap();

    let detail = m.trend_detail.unwrap();
    let color = detail.color_code.unwrap();
    assert_eq!(color.code, MilitaryColorCode::Red);
    assert!(color.black);
}

// ---------------------------------------------------------------------------
// No color code: ordinary METAR must leave fields None
// ---------------------------------------------------------------------------

#[test]
fn no_color_code_in_ordinary_metar() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(m.color_code.is_none());
    assert!(m.color_code_forecast.is_none());
}

// ---------------------------------------------------------------------------
// Describe
// ---------------------------------------------------------------------------

#[test]
fn describe_color_code_grn() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 GRN").unwrap();
    let desc = describe_metar(&m, Language::En);
    let cc = desc.color_code.unwrap();
    assert!(cc.contains("Green") || cc.contains("GRN"), "{}", cc);
}

#[test]
fn describe_black_color_code() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 BLACKGRN").unwrap();
    let desc = describe_metar(&m, Language::En);
    let cc = desc.color_code.unwrap();
    assert!(cc.to_lowercase().contains("closed"), "{}", cc);
    assert!(cc.contains("GRN") || cc.contains("Green"), "{}", cc);
}

#[test]
fn describe_color_code_forecast() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 GRN BLU").unwrap();
    let desc = describe_metar(&m, Language::En);
    assert!(desc.color_code.is_some());
    let forecast = desc.color_code_forecast.unwrap();
    assert!(
        forecast.contains("BLU") || forecast.contains("Blue"),
        "{}",
        forecast
    );
}

#[test]
fn describe_color_code_in_tempo_trend() {
    let m =
        parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN TEMPO AMB").unwrap();
    let desc = describe_metar(&m, Language::En);
    let trend = desc.trend.unwrap();
    assert!(trend.to_lowercase().contains("temporar"), "{}", trend);
    assert!(
        trend.contains("AMB") || trend.to_lowercase().contains("amber"),
        "{}",
        trend
    );
}

#[test]
fn format_metar_includes_color_code_line() {
    let m = parse_metar("METAR EGVN 120930Z 25010KT 9999 FEW020 15/10 Q1013 GRN BLU").unwrap();
    let text = metar_taf_parser::format_metar(&m, Language::En);
    assert!(text.contains("Color code:"), "{}", text);
    assert!(text.contains("Color fcst:"), "{}", text);
}
