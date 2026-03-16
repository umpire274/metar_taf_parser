use crate::common::report_modifier::ReportModifier;
use crate::common::tokenizer::Tokenizer;
use crate::taf::errors::TafError;
use crate::taf::models::taf::Taf;
use crate::taf::parser::time::{parse_taf_time, parse_validity};

pub fn parse_taf(input: &str) -> Result<Taf, TafError> {
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
        });
    }

    // Optional AMD/COR
    let (modifier, station) = match token.as_str() {
        "AMD" => (
            ReportModifier::Amendment,
            tokenizer.next().ok_or(TafError::InvalidFormat)?,
        ),
        "COR" => (
            ReportModifier::Correction,
            tokenizer.next().ok_or(TafError::InvalidFormat)?,
        ),
        _ => (ReportModifier::Normal, token),
    };

    // Issue time: DDHHMMZ
    let time_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let issued_at = parse_taf_time(&time_token)?;

    // Validity: DDHH/DDHH
    let validity_token = tokenizer.next().ok_or(TafError::InvalidFormat)?;
    let validity = parse_validity(&validity_token)?;

    let remaining: Vec<String> = tokenizer.map(|s| s.to_string()).collect();
    let forecasts = crate::taf::parser::forecast::parse_forecasts(&remaining);

    Ok(Taf {
        station: station.to_string(),
        issued_at: Some(issued_at),
        validity: Some(validity),
        modifier,
        forecasts,
    })
}
