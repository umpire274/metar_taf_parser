use crate::common::tokenizer::Tokenizer;
use crate::taf::errors::TafError;
use crate::taf::models::taf::Taf;

pub fn parse_taf(input: &str) -> Result<Taf, TafError> {
    let normalized = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut tokenizer = Tokenizer::new(&normalized);

    // Mandatory "TAF"
    let first = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    if first != "TAF" {
        return Err(TafError::InvalidFormat);
    }

    // Optional AMD / COR
    let mut station = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    if station == "AMD" || station == "COR" {
        station = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    }

    // Issue time: DDHHMMZ
    let time_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let issued_at = parse_taf_time(&time_token)?;

    // Validity: DDHH/DDHH
    let validity_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let validity = parse_validity(&validity_token)?;

    // Remaining tokens → forecasts
    let remaining: Vec<String> = tokenizer.collect();
    let forecasts = crate::taf::parser::forecast::parse_forecasts(&remaining);

    Ok(Taf {
        station,
        issued_at,
        validity,
        forecasts,
    })
}

fn parse_taf_time(
    token: &str,
) -> Result<crate::taf::models::time::TafTime, crate::taf::errors::TafError> {
    if token.len() != 7 || !token.ends_with('Z') {
        return Err(crate::taf::errors::TafError::InvalidFormat);
    }

    Ok(crate::taf::models::time::TafTime {
        day: token[0..2]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
        hour: token[2..4]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
        minute: token[4..6]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
    })
}

fn parse_validity(
    token: &str,
) -> Result<crate::taf::models::time::TafValidity, crate::taf::errors::TafError> {
    let (from, to) = token
        .split_once('/')
        .ok_or(crate::taf::errors::TafError::InvalidFormat)?;

    Ok(crate::taf::models::time::TafValidity {
        from_day: from[0..2]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
        from_hour: from[2..4]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
        to_day: to[0..2]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
        to_hour: to[2..4]
            .parse()
            .map_err(|_| crate::taf::errors::TafError::InvalidFormat)?,
    })
}
