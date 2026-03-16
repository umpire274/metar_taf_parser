use crate::metar::models::trend::MetarTrend;

pub fn parse_trend(token: &str) -> Option<MetarTrend> {
    match token {
        "NOSIG" => Some(MetarTrend::Nosig),
        "BECMG" => Some(MetarTrend::Becmg),
        "TEMPO" => Some(MetarTrend::Tempo),
        _ => None,
    }
}
