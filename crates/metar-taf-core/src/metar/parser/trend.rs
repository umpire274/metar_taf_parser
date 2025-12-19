use crate::metar::models::trend::MetarTrend;

pub fn parse_trend(token: &str) -> Option<MetarTrend> {
    match token {
        "NOSIG" => Some(MetarTrend::Nosig),
        _ => None,
    }
}
