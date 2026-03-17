//! Military flight-condition color codes.
//!
//! Military airfields often report current conditions and trend forecasts
//! using standardised color codes instead of (or in addition to) explicit
//! visibility and ceiling values.
use serde::Serialize;

/// The six military flight-condition color codes, ordered from best (BLU)
/// to worst (RED) conditions.
///
/// Each code summarises a combined visibility and ceiling threshold:
///
/// | Code | Visibility       | Ceiling          |
/// |------|-----------------|------------------|
/// | BLU  | > 5 mi          | > 2 500 ft        |
/// | WHT  | 3⅛ – 5 mi       | 1 500 – 2 500 ft  |
/// | GRN  | 2¼ – 3⅛ mi      | 700 – 1 500 ft    |
/// | YLO  | 1⅛ – 2¼ mi      | 300 – 700 ft      |
/// | AMB  | ½ – 1⅛ mi       | 200 – 300 ft      |
/// | RED  | < ½ mi          | < 200 ft          |
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MilitaryColorCode {
    /// Visibility > 5 mi and ceiling > 2 500 ft.
    Blu,
    /// Visibility 3⅛–5 mi and ceiling 1 500–2 500 ft.
    Wht,
    /// Visibility 2¼–3⅛ mi and ceiling 700–1 500 ft.
    Grn,
    /// Visibility 1⅛–2¼ mi and ceiling 300–700 ft.
    Ylo,
    /// Visibility ½–1⅛ mi and ceiling 200–300 ft.
    Amb,
    /// Visibility < ½ mi or ceiling < 200 ft.
    Red,
}

/// A military flight-condition color code, optionally prefixed with `BLACK`.
///
/// When `black` is `true` the airfield is closed even though the meteorological
/// conditions correspond to the given `code` (e.g. `BLACKGRN` means the field
/// is closed but conditions are otherwise at GRN level).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MilitaryColor {
    /// The flight-condition category.
    pub code: MilitaryColorCode,
    /// `true` when the `BLACK` prefix is present, indicating a closed field.
    pub black: bool,
}

