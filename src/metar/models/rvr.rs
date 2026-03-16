use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RvrQualifier {
    Above,
    Below,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RvrTendency {
    Upward,
    Downward,
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RvrUnit {
    Meters,
    Feet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RvrValue {
    pub value: u16,
    pub qualifier: Option<RvrQualifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunwayVisualRange {
    pub runway_designator: String,
    pub min: RvrValue,
    pub max: Option<RvrValue>,
    pub tendency: Option<RvrTendency>,
    pub unit: RvrUnit,
}
