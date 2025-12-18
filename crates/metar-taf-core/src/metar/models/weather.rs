#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeatherIntensity {
    Light,
    Moderate,
    Heavy,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeatherDescriptor {
    Shallow,      // MI
    Partial,      // PR
    Patches,      // BC
    LowDrifting,  // DR
    Blowing,      // BL
    Showers,      // SH
    Thunderstorm, // TS
    Freezing,     // FZ
    Vicinity,     // VC
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeatherPhenomenon {
    Rain,       // RA
    Snow,       // SN
    Drizzle,    // DZ
    Thunder,    // TS (standalone)
    Fog,        // FG
    Mist,       // BR
    Hail,       // GR
    SmallHail,  // GS
    IcePellets, // PL
    SnowGrains, // SG
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Weather {
    pub intensity: Option<WeatherIntensity>,
    pub descriptors: Vec<WeatherDescriptor>,
    pub phenomena: Vec<WeatherPhenomenon>,
}
