use metar_taf_core::metar::models::trend::MetarTrend;
use metar_taf_core::parse_metar;

#[test]
fn parse_nosig_trend() {
    let metar = "METAR UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 NOSIG";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Nosig)));
}

#[test]
fn parse_becmg_trend() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 BECMG";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Becmg)));
}

#[test]
fn parse_tempo_trend() {
    let metar = "METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015 TEMPO";
    let parsed = parse_metar(metar).unwrap();

    assert!(matches!(parsed.trend, Some(MetarTrend::Tempo)));
}
