//! Module `lib`.
//!
//! Contains types and parsing logic implemented for this crate.
pub mod airports;
/// Exposes the `common` module.
pub mod common;
/// Exposes the `metar` module.
pub mod metar;
/// Exposes the `taf` module.
pub mod taf;

pub use common::describe::{
    ForecastDescription, Language, MetarDescription, TafDescription, describe_metar, describe_taf,
    format_metar, format_taf,
};
pub use common::parse::{ParseError, ParsedReport, parse, parse_strict};
pub use metar::parser::metar::{parse_metar, parse_metar_strict};
pub use taf::parser::taf::{parse_taf, parse_taf_strict};
