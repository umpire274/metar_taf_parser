use crate::metar::models::weather::*;

pub fn parse_weather(token: &str) -> Option<Weather> {
    let mut s = token;

    // ---- intensity ----
    let intensity = match s.chars().next()? {
        '-' => {
            s = &s[1..];
            Some(WeatherIntensity::Light)
        }
        '+' => {
            s = &s[1..];
            Some(WeatherIntensity::Heavy)
        }
        _ => None,
    };

    let mut descriptors = Vec::new();
    let mut phenomena = Vec::new();

    // ---- descriptors (2-char codes) ----
    loop {
        let code = s.get(0..2)?;
        let desc = match code {
            "MI" => WeatherDescriptor::Shallow,
            "PR" => WeatherDescriptor::Partial,
            "BC" => WeatherDescriptor::Patches,
            "DR" => WeatherDescriptor::LowDrifting,
            "BL" => WeatherDescriptor::Blowing,
            "SH" => WeatherDescriptor::Showers,
            "TS" => WeatherDescriptor::Thunderstorm,
            "FZ" => WeatherDescriptor::Freezing,
            "VC" => WeatherDescriptor::Vicinity,
            _ => break,
        };
        descriptors.push(desc);
        s = &s[2..];
    }

    // ---- phenomena (2-char codes, repeatable) ----
    while let Some(code) = s.get(0..2) {
        let phen = match code {
            "RA" => WeatherPhenomenon::Rain,
            "SN" => WeatherPhenomenon::Snow,
            "DZ" => WeatherPhenomenon::Drizzle,
            "FG" => WeatherPhenomenon::Fog,
            "BR" => WeatherPhenomenon::Mist,
            "GR" => WeatherPhenomenon::Hail,
            "GS" => WeatherPhenomenon::SmallHail,
            "PL" => WeatherPhenomenon::IcePellets,
            "SG" => WeatherPhenomenon::SnowGrains,
            "TS" => WeatherPhenomenon::Thunder,
            _ => {
                phenomena.push(WeatherPhenomenon::Unknown(code.to_string()));
                break;
            }
        };
        phenomena.push(phen);
        s = &s[2..];
    }

    if descriptors.is_empty() && phenomena.is_empty() {
        None
    } else {
        Some(Weather {
            intensity,
            descriptors,
            phenomena,
        })
    }
}
