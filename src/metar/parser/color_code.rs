//! Parser for military flight-condition color code tokens.
use crate::metar::models::color_code::{MilitaryColor, MilitaryColorCode};

/// Parses a military color code token into a [`MilitaryColor`].
///
/// Recognised plain tokens: `BLU`, `WHT`, `GRN`, `YLO`, `AMB`, `RED`.
/// The same tokens prefixed with `BLACK` (e.g. `BLACKGRN`) set
/// [`MilitaryColor::black`] to `true`, indicating a closed airfield.
///
/// # Returns
///
/// Returns `Some(MilitaryColor)` when the token is a valid color code,
/// otherwise `None`.
///
/// # Examples
///
/// ```
/// use metar_taf_parser::metar::parser::color_code::parse_color_code;
/// use metar_taf_parser::metar::models::color_code::{MilitaryColor, MilitaryColorCode};
///
/// let c = parse_color_code("GRN").unwrap();
/// assert_eq!(c.code, MilitaryColorCode::Grn);
/// assert!(!c.black);
///
/// let c = parse_color_code("BLACKAMB").unwrap();
/// assert_eq!(c.code, MilitaryColorCode::Amb);
/// assert!(c.black);
///
/// assert!(parse_color_code("PROB30").is_none());
/// ```
pub fn parse_color_code(token: &str) -> Option<MilitaryColor> {
    let (black, code_str) = match token.strip_prefix("BLACK") {
        Some(rest) => (true, rest),
        None => (false, token),
    };

    let code = match code_str {
        "BLU" => MilitaryColorCode::Blu,
        "WHT" => MilitaryColorCode::Wht,
        "GRN" => MilitaryColorCode::Grn,
        "YLO" => MilitaryColorCode::Ylo,
        "AMB" => MilitaryColorCode::Amb,
        "RED" => MilitaryColorCode::Red,
        _ => return None,
    };

    Some(MilitaryColor { code, black })
}
