use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MetarTrend {
    Nosig,
}
