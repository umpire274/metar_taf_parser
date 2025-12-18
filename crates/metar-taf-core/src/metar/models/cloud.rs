use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct CloudLayer {
    pub amount: CloudAmount,
    pub altitude_ft: Option<u16>, // feet
    pub cloud_type: Option<CloudType>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum CloudAmount {
    FEW,
    SCT,
    BKN,
    OVC,
    NSC,
    SKC,
    VV,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum CloudType {
    CB,
    TCU,
}
