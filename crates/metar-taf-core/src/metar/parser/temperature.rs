use crate::metar::models::temperature::Temperature;

fn parse_single_temp(s: &str) -> Option<i8> {
    if let Some(rest) = s.strip_prefix('M') {
        let v: i8 = rest.parse().ok()?;
        Some(-v)
    } else {
        let v: i8 = s.parse().ok()?;
        Some(v)
    }
}

pub fn parse_temperature(token: &str) -> Option<Temperature> {
    let (temp_str, dew_str) = token.split_once('/')?;

    let temperature = parse_single_temp(temp_str)?;
    let dew_point = parse_single_temp(dew_str)?;

    Some(Temperature {
        temperature,
        dew_point,
    })
}
