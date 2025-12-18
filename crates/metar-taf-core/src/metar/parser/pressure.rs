use crate::metar::models::pressure::Pressure;

pub fn parse_pressure(token: &str) -> Option<Pressure> {
    // Q1015 → hPa
    if let Some(rest) = token.strip_prefix('Q') {
        let hpa: u16 = rest.parse().ok()?;
        return Some(Pressure::QnhHpa(hpa));
    }

    // A2992 → inHg
    if let Some(rest) = token.strip_prefix('A') {
        let raw: u16 = rest.parse().ok()?;
        let inhg = raw as f32 / 100.0;
        return Some(Pressure::AltimeterInHg(inhg));
    }

    None
}
