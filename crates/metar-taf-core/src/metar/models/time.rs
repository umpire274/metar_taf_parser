use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct MetarTime {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}
