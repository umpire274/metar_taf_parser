use crate::metar::models::temperature::Temperature;

fn parse_signed_temp(s: &str) -> Option<i8> {
    if let Some(rest) = s.strip_prefix('M') {
        let v: i8 = rest.parse().ok()?;
        Some(-v)
    } else {
        s.parse().ok()
    }
}

pub fn parse_temperature(token: &str) -> Option<Temperature> {
    let (air, dew) = token.split_once('/')?;

    let air = parse_signed_temp(air)?;
    let dew = parse_signed_temp(dew)?;

    Some(Temperature {
        air,
        dew_point: dew,
    })
}
