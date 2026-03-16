use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TafTemperature {
    pub value: i8,
    pub day: u8,
    pub hour: u8,
}
