//! Module `visibility`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
/// Enumerates the allowed values for Visibility.
pub enum Visibility {
    CAVOK,
    Single {
        prevailing: u16,
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
