//! Module `taf`.
//!
//! Contains types and parsing logic implemented for this crate.
use crate::common::report_modifier::ReportModifier;
use crate::common::tokenizer::Tokenizer;
use crate::taf::errors::TafError;
use crate::taf::models::taf::Taf;
use crate::taf::parser::time::{parse_taf_time, parse_validity};

/// Parses input tokens into typed data for `parse_taf`.
pub fn parse_taf(input: &str) -> Result<Taf, TafError> {
    parse_taf_with_mode(input, false)
}

/// Parses input tokens into typed data for `parse_taf_strict`.
pub fn parse_taf_strict(input: &str) -> Result<Taf, TafError> {
    parse_taf_with_mode(input, true)
}

/// Parses input tokens into typed data for `parse_taf_with_mode`.
fn parse_taf_with_mode(input: &str, strict: bool) -> Result<Taf, TafError> {
    let normalized = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut tokenizer = Tokenizer::new(&normalized);

    // Optional leading "TAF"
    let first = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let token = if first == "TAF" {
        tokenizer.next().ok_or(TafError::InvalidFormat)?
    } else {
        first
    };

    // NIL report
    if token == "NIL" {
        return Ok(Taf {
            station: String::new(),
            issued_at: Default::default(),
            validity: Default::default(),
            modifier: ReportModifier::Nil,
            forecasts: Vec::new(),
            unparsed_groups: Vec::new(),
        });
    }

    // Optional AMD/COR — check if the next token is NIL before treating it as station
    let (modifier, station) = match token.as_str() {
        "AMD" => {
            let next = tokenizer.next().ok_or(TafError::InvalidFormat)?;
            if next == "NIL" {
                return Ok(Taf {
                    station: String::new(),
                    issued_at: None,
                    validity: None,
                    modifier: ReportModifier::Nil,
                    forecasts: Vec::new(),
                    unparsed_groups: Vec::new(),
                });
            }
            (ReportModifier::Amendment, next)
        }
        "COR" => {
            let next = tokenizer.next().ok_or(TafError::InvalidFormat)?;
            if next == "NIL" {
                return Ok(Taf {
                    station: String::new(),
                    issued_at: None,
                    validity: None,
                    modifier: ReportModifier::Nil,
                    forecasts: Vec::new(),
                    unparsed_groups: Vec::new(),
                });
            }
            (ReportModifier::Correction, next)
        }
        _ => (ReportModifier::Normal, token),
    };

    // Issue time: DDHHMMZ
    let time_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;

    // In strict mode the station must be a valid 4-letter ICAO identifier.
    if strict && !is_valid_icao_station(&station) {
        return Err(TafError::InvalidFormat);
    }

    let issued_at = parse_taf_time(&time_token)?;

    // Validity: DDHH/DDHH
    let validity_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let validity = parse_validity(&validity_token)?;

    let remaining: Vec<String> = tokenizer.map(|s| s.to_string()).collect();
    let (forecasts, unparsed_groups) = crate::taf::parser::forecast::parse_forecasts(&remaining);

    if strict && !unparsed_groups.is_empty() {
        return Err(TafError::UnsupportedGroup(unparsed_groups.join(" ")));
    }

    Ok(Taf {
        station: station.to_string(),
        issued_at: Some(issued_at),
        validity: Some(validity),
        modifier,
        forecasts,
        unparsed_groups,
    })
}

/// Returns `true` if `s` is a valid ICAO aerodrome identifier.
///
/// ICAO identifiers consist of exactly 4 ASCII uppercase letters (A–Z).
fn is_valid_icao_station(s: &str) -> bool {
    s.len() == 4 && s.bytes().all(|b| b.is_ascii_uppercase())
}

