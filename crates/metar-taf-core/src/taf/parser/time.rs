use crate::taf::errors::TafError;
use crate::taf::models::time::{TafTime, TafValidity};

pub fn parse_taf_time(token: &str) -> Result<TafTime, TafError> {
    if token.len() != 7 || !token.ends_with('Z') {
        return Err(TafError::InvalidFormat);
    }

    Ok(TafTime {
        day: token[0..2].parse().map_err(|_| TafError::InvalidFormat)?,
        hour: token[2..4].parse().map_err(|_| TafError::InvalidFormat)?,
        minute: token[4..6].parse().map_err(|_| TafError::InvalidFormat)?,
    })
}

pub fn parse_validity(token: &str) -> Result<TafValidity, TafError> {
    let (from, to) = token.split_once('/').ok_or(TafError::InvalidFormat)?;

    Ok(TafValidity {
        from_day: from[0..2].parse().map_err(|_| TafError::InvalidFormat)?,
        from_hour: from[2..4].parse().map_err(|_| TafError::InvalidFormat)?,
        to_day: to[0..2].parse().map_err(|_| TafError::InvalidFormat)?,
        to_hour: to[2..4].parse().map_err(|_| TafError::InvalidFormat)?,
    })
}
