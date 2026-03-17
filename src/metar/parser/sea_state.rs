//! Parser for sea state tokens (`W[TT]/[S|H][HH]`).
use crate::metar::models::sea_state::{SeaState, WaveHeightKind};

/// Parses a sea state token into a [`SeaState`] value.
///
/// Accepts tokens of the form `W[TT]/[S|H][HH]` where:
///
/// - `TT`  = two-digit water temperature, optionally prefixed with `M` for
///   negative values (e.g. `M2` → −2 °C), or `//` for unavailable.
/// - `S`   = WMO state-of-sea code that follows (0–9).
/// - `H`   = significant wave height in decimetres that follows.
/// - `HH`  = one or two digit wave value, or `/` for unavailable.
///
/// # Examples
///
/// ```
/// use metar_taf_parser::metar::parser::sea_state::parse_sea_state;
/// use metar_taf_parser::metar::models::sea_state::{SeaState, WaveHeightKind};
///
/// let s = parse_sea_state("W12/S8").unwrap();
/// assert_eq!(s.water_temperature, Some(12));
/// assert_eq!(s.wave_kind, WaveHeightKind::StateCode);
/// assert_eq!(s.wave_value, Some(8));
///
/// let s = parse_sea_state("WM2/S3").unwrap();
/// assert_eq!(s.water_temperature, Some(-2));
///
/// let s = parse_sea_state("W//S/").unwrap();
/// assert!(s.water_temperature.is_none());
/// assert!(s.wave_value.is_none());
///
/// assert!(parse_sea_state("WIND").is_none());
/// ```
pub fn parse_sea_state(token: &str) -> Option<SeaState> {
    let rest = token.strip_prefix('W')?;

    // The temperature separator is context-dependent:
    // - `W//…` starts with `//`; the wave part begins after the third character (`/`)
    // - all other formats have a `/` after the temperature digits
    let (temp_part, wave_part) = if rest.starts_with("//") {
        // "//S/" → get(2..) = "S/" (the two slashes are at indices 0 and 1)
        let after = rest.get(2..)?;
        ("//", after)
    } else {
        let slash = rest.find('/')?;
        (&rest[..slash], &rest[slash + 1..])
    };

    let water_temperature = parse_temperature(temp_part)?;
    let (wave_kind, wave_value) = parse_wave(wave_part)?;

    Some(SeaState {
        water_temperature,
        wave_kind,
        wave_value,
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Parses the temperature portion: `M12`, `M2`, `12`, `2`, or `//`.
fn parse_temperature(part: &str) -> Option<Option<i8>> {
    if part == "//" {
        return Some(None);
    }

    let (negative, digits) = if let Some(v) = part.strip_prefix('M') {
        (true, v)
    } else {
        (false, part)
    };

    // Accept 1 or 2 ASCII digit(s) — some stations omit the leading zero
    if digits.is_empty() || digits.len() > 2 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let value: i8 = digits.parse().ok()?;
    Some(Some(if negative { -value } else { value }))
}

/// Parses the wave portion: `S8`, `H15`, `S/`, etc.
///
/// Returns `(WaveHeightKind, Option<u8>)` or `None` if the format is invalid.
fn parse_wave(part: &str) -> Option<(WaveHeightKind, Option<u8>)> {
    let (kind, value_str) = if let Some(v) = part.strip_prefix('S') {
        (WaveHeightKind::StateCode, v)
    } else if let Some(v) = part.strip_prefix('H') {
        (WaveHeightKind::HeightDm, v)
    } else {
        return None;
    };

    if value_str == "/" || value_str.is_empty() {
        return Some((kind, None));
    }

    if !value_str.chars().all(|c| c.is_ascii_digit()) || value_str.len() > 2 {
        return None;
    }

    let value: u8 = value_str.parse().ok()?;
    Some((kind, Some(value)))
}
