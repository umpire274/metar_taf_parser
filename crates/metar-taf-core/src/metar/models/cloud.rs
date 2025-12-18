#[derive(Debug)]
pub struct CloudLayer {
    pub amount: CloudAmount,
    pub altitude_ft: Option<u16>, // feet
    pub cloud_type: Option<CloudType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloudAmount {
    FEW,
    SCT,
    BKN,
    OVC,
    NSC,
    SKC,
    VV,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloudType {
    CB,
    TCU,
}
