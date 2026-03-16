//! Module `cloud`.
//!
//! Contains types and parsing logic implemented for this crate.
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
/// Defines the CloudLayer domain model used by the parser.
pub struct CloudLayer {
    pub amount: CloudAmount,
    pub altitude_ft: Option<u16>, // feet
    pub cloud_type: Option<CloudType>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
/// Enumerates the allowed values for CloudAmount.
pub enum CloudAmount {
    FEW,
    SCT,
    BKN,
    OVC,
    NSC,
    SKC,
    VV,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
/// Enumerates the allowed values for CloudType.
pub enum CloudType {
    CB,
    TCU,
}
