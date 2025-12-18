use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Pressure {
    QnhHpa(u16),
    AltimeterInHg(f32),
}
