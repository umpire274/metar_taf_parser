use crate::metar::models::pressure::Pressure;

pub fn parse_pressure(token: &str) -> Option<Pressure> {
    // Q1015 → hPa (exactly 4 digits)
    if let Some(rest) = token.strip_prefix('Q') {
        if rest.len() != 4 || !rest.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        let hpa: u16 = rest.parse().ok()?;
        return Some(Pressure::QnhHpa(hpa));
    }

    // A2992 → inHg (exactly 4 digits, hundredths)
    if let Some(rest) = token.strip_prefix('A') {
        if rest.len() != 4 || !rest.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        let raw: u16 = rest.parse().ok()?;
        let inhg = raw as f32 / 100.0;
        return Some(Pressure::AltimeterInHg(inhg));
    }

    None
}
