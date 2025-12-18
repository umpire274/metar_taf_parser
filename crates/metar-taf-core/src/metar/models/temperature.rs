use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Temperature {
    pub temperature: i8,
    pub dew_point: i8,
}
