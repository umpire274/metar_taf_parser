use crate::taf::errors::TafError;
use crate::taf::models::time::{TafTime, TafValidity};

pub fn parse_taf_time(token: &str) -> Result<TafTime, TafError> {
    if token.len() != 7 || !token.ends_with('Z') || !token[..6].chars().all(|c| c.is_ascii_digit())
    {
        return Err(TafError::InvalidFormat);
    }

    let day: u8 = token[0..2].parse().map_err(|_| TafError::InvalidFormat)?;
    let hour: u8 = token[2..4].parse().map_err(|_| TafError::InvalidFormat)?;
    let minute: u8 = token[4..6].parse().map_err(|_| TafError::InvalidFormat)?;

    if !(1..=31).contains(&day) || hour > 23 || minute > 59 {
        return Err(TafError::InvalidFormat);
    }

    Ok(TafTime { day, hour, minute })
}

pub fn parse_validity(token: &str) -> Result<TafValidity, TafError> {
    let (from, to) = token.split_once('/').ok_or(TafError::InvalidFormat)?;

    if from.len() != 4
        || to.len() != 4
        || !from.chars().all(|c| c.is_ascii_digit())
        || !to.chars().all(|c| c.is_ascii_digit())
    {
        return Err(TafError::InvalidFormat);
    }

    let from_day: u8 = from[0..2].parse().map_err(|_| TafError::InvalidFormat)?;
    let from_hour: u8 = from[2..4].parse().map_err(|_| TafError::InvalidFormat)?;
    let to_day: u8 = to[0..2].parse().map_err(|_| TafError::InvalidFormat)?;
    let to_hour: u8 = to[2..4].parse().map_err(|_| TafError::InvalidFormat)?;

    if !(1..=31).contains(&from_day)
        || !(1..=31).contains(&to_day)
        || from_hour > 23
        || to_hour > 24
    {
        return Err(TafError::InvalidFormat);
    }

    Ok(TafValidity {
        from_day,
        from_hour,
        to_day,
        to_hour,
    })
}
