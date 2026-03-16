//! Module `rmk`.
//!
//! Contains types and parsing logic implemented for this crate.
pub fn parse_rmk(tokens: &[String]) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }

    Some(tokens.join(" "))
}
