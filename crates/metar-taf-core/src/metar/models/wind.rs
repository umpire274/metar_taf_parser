use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Wind {
    pub direction: Option<u16>,
    pub speed_kt: u16,
    pub gust_kt: Option<u16>,
}
