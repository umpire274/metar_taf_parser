//! Module `visibility`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

/// Qualifier for visibility values reported as more or less than a threshold.
///
/// Used in statute-mile visibility groups prefixed with `P` (above) or `M` (below).
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum VisibilityQualifier {
    /// Visibility is more than the reported value (`P` prefix, e.g. `P6SM`).
    Above,
    /// Visibility is less than the reported value (`M` prefix, e.g. `M1/4SM`).
    Below,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for Visibility.
pub enum Visibility {
    CAVOK,
    Single {
        /// Prevailing visibility in metres.
        prevailing: u16,
        /// `P`/`M` qualifier present in statute-mile groups.
        qualifier: Option<VisibilityQualifier>,
        /// `true` when the `NDV` (No Directional Variation) suffix was present.
        ndv: bool,
    },
    WithMinimum {
        prevailing: u16,
        minimum: u16,
        direction: VisibilityDirection,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for VisibilityDirection.
pub enum VisibilityDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
