//! Unified entry-point for METAR and TAF parsing.
//!
//! [`parse`] inspects the leading token of the input string and dispatches to
//! the correct specialised parser, returning a [`ParsedReport`] that wraps
//! either a [`Metar`] or a [`Taf`].
//!
//! [`parse_strict`] behaves the same but requires an explicit `METAR`,
//! `SPECI`, or `TAF` prefix and delegates to the strict parser variants, which
//! reject any unrecognised group.

use crate::metar::errors::MetarError;
use crate::metar::models::Metar;
use crate::metar::parser::metar::{parse_metar, parse_metar_strict};
use crate::taf::errors::TafError;
use crate::taf::models::taf::Taf;
use crate::taf::parser::taf::{parse_taf, parse_taf_strict};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Result of a successful [`parse`] or [`parse_strict`] call.
///
/// Wraps either a fully parsed [`Metar`] or [`Taf`] report. Use pattern
/// matching to access the inner value:
///
/// ```rust
/// use metar_taf_parser::{parse, ParsedReport};
///
/// let report = parse("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
/// match report {
///     ParsedReport::Metar(m) => println!("station: {}", m.station),
///     ParsedReport::Taf(t)   => println!("station: {}", t.station),
/// }
/// ```
///
/// The `Metar` variant is heap-allocated (`Box<Metar>`) to keep the enum size
/// small: `Metar` is significantly larger than `Taf` on the stack.
#[derive(Debug)]
pub enum ParsedReport {
    /// A parsed METAR or SPECI observation.
    Metar(Box<Metar>),
    /// A parsed TAF forecast.
    Taf(Taf),
}

/// Error returned by [`parse`] and [`parse_strict`].
///
/// Wraps the underlying specialised error (`MetarError` or `TafError`) or
/// signals that the report type could not be determined from the input prefix.
#[derive(Debug, Error)]
pub enum ParseError {
    /// The input was dispatched to the METAR parser, which rejected it.
    #[error("METAR parse error: {0}")]
    Metar(#[from] MetarError),

    /// The input was dispatched to the TAF parser, which rejected it.
    #[error("TAF parse error: {0}")]
    Taf(#[from] TafError),

    /// The leading token is not `METAR`, `SPECI`, or `TAF`.
    ///
    /// Only returned by [`parse_strict`]; [`parse`] defaults to the METAR
    /// parser when no recognised prefix is present.
    #[error("unknown report type: expected a leading METAR, SPECI, or TAF token")]
    UnknownReportType,
}

// ---------------------------------------------------------------------------
// Public functions
// ---------------------------------------------------------------------------

/// Parses a METAR, SPECI, or TAF string, automatically selecting the correct
/// decoder based on the leading token.
///
/// | Leading token          | Parser used     |
/// |------------------------|-----------------|
/// | `TAF`                  | [`parse_taf`]   |
/// | `METAR`, `SPECI`, none | [`parse_metar`] |
///
/// This function is tolerant: unrecognised groups are collected in
/// `unparsed_groups` rather than causing an error. Use [`parse_strict`] for
/// strict validation.
///
/// # Arguments
///
/// * `input` - Raw METAR or TAF string.
///
/// # Errors
///
/// Returns [`ParseError::Metar`] or [`ParseError::Taf`] if the selected
/// parser rejects the input (e.g. empty string or missing mandatory fields).
///
/// # Example
///
/// ```rust
/// use metar_taf_parser::{parse, ParsedReport};
///
/// let metar = parse("METAR LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
/// assert!(matches!(metar, ParsedReport::Metar(_)));
///
/// let taf = parse("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();
/// assert!(matches!(taf, ParsedReport::Taf(_)));
/// ```
pub fn parse(input: &str) -> Result<ParsedReport, ParseError> {
    let first = input.split_whitespace().next().unwrap_or("");
    if first == "TAF" {
        Ok(ParsedReport::Taf(parse_taf(input)?))
    } else {
        Ok(ParsedReport::Metar(Box::new(parse_metar(input)?)))
    }
}

/// Strict variant of [`parse`]: requires an explicit `METAR`, `SPECI`, or
/// `TAF` leading token and rejects any unrecognised group.
///
/// | Leading token   | Parser used           |
/// |-----------------|-----------------------|
/// | `METAR`/`SPECI` | [`parse_metar_strict`] |
/// | `TAF`           | [`parse_taf_strict`]   |
/// | anything else   | `Err(UnknownReportType)` |
///
/// # Errors
///
/// * [`ParseError::UnknownReportType`] — no recognised prefix found.
/// * [`ParseError::Metar`] — METAR strict parser rejected the input.
/// * [`ParseError::Taf`]   — TAF strict parser rejected the input.
///
/// # Example
///
/// ```rust
/// use metar_taf_parser::{parse_strict, ParseError};
///
/// assert!(parse_strict("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").is_err());
/// ```
pub fn parse_strict(input: &str) -> Result<ParsedReport, ParseError> {
    let first = input.split_whitespace().next().unwrap_or("");
    match first {
        "METAR" | "SPECI" => Ok(ParsedReport::Metar(Box::new(parse_metar_strict(input)?))),
        "TAF" => Ok(ParsedReport::Taf(parse_taf_strict(input)?)),
        _ => Err(ParseError::UnknownReportType),
    }
}
