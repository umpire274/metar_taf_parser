//! Parser for METAR wind shear runway groups (`WS R23`, `WS ALL RWY`).
//!
//! The `WS` token is consumed by the caller; this module reads the
//! subsequent token(s) from the tokenizer to identify the affected runway.
use crate::common::tokenizer::Tokenizer;
use crate::metar::models::wind_shear::MetarWindShearRunway;

/// Reads the runway qualifier following a `WS` token from the tokenizer.
///
/// Must be called **after** the `WS` token has already been consumed.
///
/// | Remaining tokens      | Result                                    |
/// |-----------------------|-------------------------------------------|
/// | `ALL RWY …`           | [`MetarWindShearRunway::AllRunways`]       |
/// | `R23 …`, `R06R …`     | [`MetarWindShearRunway::Runway`]`("23")`  |
/// | anything else         | `None` (no tokens consumed)               |
///
/// When `None` is returned the tokenizer position is unchanged.
pub fn parse_metar_wind_shear_runway(tokenizer: &mut Tokenizer) -> Option<MetarWindShearRunway> {
    let next = tokenizer.peek()?;

    if next == "ALL" {
        tokenizer.next(); // consume ALL
        // Consume the optional "RWY" token if present
        if tokenizer.peek().map(|t| t == "RWY").unwrap_or(false) {
            tokenizer.next();
        }
        return Some(MetarWindShearRunway::AllRunways);
    }

    // Runway designator starts with 'R' followed by at least two digits
    if next.starts_with('R') && next.len() >= 3 && next[1..3].chars().all(|c| c.is_ascii_digit()) {
        let token = tokenizer.next()?;
        // Strip leading 'R' to get the bare designator (e.g. "23", "06R", "23L")
        return Some(MetarWindShearRunway::Runway(token[1..].to_string()));
    }

    None
}

