//! Module `mod`.
//!
//! Contains types and parsing logic implemented for this crate.
pub mod db;
/// Exposes the `model` module.
pub mod model;

pub use db::AirportDb;
pub use model::Airport;
