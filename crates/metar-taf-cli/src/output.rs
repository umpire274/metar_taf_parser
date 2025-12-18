use metar_taf_core::airports::Airport;
use metar_taf_core::metar::models::Metar;
use metar_taf_core::taf::models::Taf;
use serde::Serialize;

#[derive(Serialize)]
pub struct MetarOutput<'a> {
    pub airport: &'a Airport,
    pub metar: &'a Metar,
}

#[derive(Serialize)]
pub struct TafOutput<'a> {
    pub airport: &'a Airport,
    pub taf: &'a Taf,
}

#[derive(Serialize)]
pub struct AllOutput<'a> {
    pub airport: &'a Airport,
    pub metar: Option<&'a Metar>,
    pub taf: Option<&'a Taf>,
}
