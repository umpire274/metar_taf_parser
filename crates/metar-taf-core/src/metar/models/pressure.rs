#[derive(Debug)]
pub enum Pressure {
    QnhHpa(u16),
    AltimeterInHg(f32),
}
