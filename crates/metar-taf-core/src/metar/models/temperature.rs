use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Temperature {
    pub air: i8,
    pub dew_point: i8,
}
