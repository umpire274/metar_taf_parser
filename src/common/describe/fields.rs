//! Shared field description helpers used by both METAR and TAF description logic.

use crate::common::describe::locale::Locale;
use crate::metar::models::cloud::{CloudAmount, CloudLayer};
use crate::metar::models::color_code::{MilitaryColor, MilitaryColorCode};
use crate::metar::models::pressure::Pressure;
use crate::metar::models::remark::{AutoStationKind, LightningType, Remark, Remarks};
use crate::metar::models::runway_state::RunwayState;
use crate::metar::models::rvr::{RunwayVisualRange, RvrQualifier, RvrTendency, RvrUnit};
use crate::metar::models::sea_state::{SeaState, WaveHeightKind};
use crate::metar::models::trend::MetarTrendDetail;
use crate::metar::models::visibility::{Visibility, VisibilityQualifier};
use crate::metar::models::weather::Weather;
use crate::metar::models::wind::Wind;
use crate::metar::models::wind_shear::MetarWindShearRunway;
use crate::taf::models::icing::{Icing, IcingIntensity};
use crate::taf::models::turbulence::{Turbulence, TurbulenceIntensity};
use crate::taf::models::wind_shear::TafWindShear;

/// Describes a wind group in natural language.
///
/// Produces output such as `"wind from 180° at 10 kt"`,
/// `"wind from variable direction at 5 kt, gusting 15 kt"`, or
/// `"wind from 200° at 8 kt, variable 180 to 240°"`.
pub fn describe_wind<L: Locale>(wind: &Wind, locale: &L) -> String {
    if wind.indeterminate {
        return "wind direction and speed not available".to_string();
    }

    let dir = match wind.direction {
        None => "variable direction".to_string(),
        Some(d) => format!("{}°", d),
    };
    let unit = locale.wind_unit(&wind.unit);

    let base = match wind.gust {
        None => format!("wind from {} at {} {}", dir, wind.speed, unit),
        Some(g) => format!(
            "wind from {} at {} {}, gusting {} {}",
            dir, wind.speed, unit, g, unit
        ),
    };

    match &wind.variation {
        None => base,
        Some(v) => format!("{}, variable {} to {}°", base, v.min, v.max),
    }
}

/// Describes a visibility group in natural language.
///
/// Handles CAVOK, prevailing visibility in metres (with optional NDV and
/// above/below qualifiers), statute-mile groups, and directional minimums.
pub fn describe_visibility<L: Locale>(vis: &Visibility, locale: &L) -> String {
    match vis {
        Visibility::CAVOK => "CAVOK (ceiling and visibility OK)".to_string(),
        Visibility::Single {
            prevailing,
            qualifier,
            ndv,
        } => {
            let base = match qualifier {
                None => {
                    if *prevailing >= 9999 {
                        "visibility greater than 10 km".to_string()
                    } else {
                        format!("visibility {} m", prevailing)
                    }
                }
                Some(VisibilityQualifier::Above) => {
                    format!("visibility more than {} m", prevailing)
                }
                Some(VisibilityQualifier::Below) => {
                    format!("visibility less than {} m", prevailing)
                }
            };
            if *ndv {
                format!("{} (no directional variation)", base)
            } else {
                base
            }
        }
        Visibility::WithMinimum {
            prevailing,
            minimum,
            direction,
        } => {
            let dir = locale.visibility_direction(direction);
            format!(
                "visibility {} m, minimum {} m to the {}",
                prevailing, minimum, dir
            )
        }
    }
}

/// Describes a cloud layer in natural language.
///
/// Handles special values (NSC, SKC, VV) as well as standard layers with
/// optional altitude and cloud type.
pub fn describe_cloud<L: Locale>(cloud: &CloudLayer, locale: &L) -> String {
    match &cloud.amount {
        CloudAmount::NSC => "no significant clouds".to_string(),
        CloudAmount::SKC => "sky clear".to_string(),
        CloudAmount::NCD => "no clouds detected".to_string(),
        CloudAmount::CLR => "no cloud below 12,000 ft".to_string(),
        CloudAmount::VV => match cloud.altitude_ft {
            None => "vertical visibility not available".to_string(),
            Some(f) => format!("vertical visibility {} ft", f),
        },
        amount => {
            let amount_str = locale.cloud_amount(amount);
            match cloud.altitude_ft {
                None => format!("{} (height not available)", amount_str),
                Some(f) => {
                    let base = format!("{} at {} ft", amount_str, f);
                    match &cloud.cloud_type {
                        None => base,
                        Some(t) => format!("{} ({})", base, locale.cloud_type(t)),
                    }
                }
            }
        }
    }
}

/// Describes a present weather group in natural language.
///
/// Assembles intensity, descriptors and phenomena into a single phrase,
/// e.g. `"heavy showers of rain"` or `"light freezing drizzle"`.
pub fn describe_weather<L: Locale>(weather: &Weather, locale: &L) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(intensity) = &weather.intensity {
        parts.push(locale.weather_intensity(intensity).to_string());
    }
    for desc in &weather.descriptors {
        parts.push(locale.weather_descriptor(desc).to_string());
    }
    for phen in &weather.phenomena {
        parts.push(locale.weather_phenomenon(phen));
    }

    parts.join(" ")
}

/// Describes a pressure group in natural language.
///
/// Supports both QNH in hPa and altimeter in inHg.
pub fn describe_pressure(pressure: &Pressure) -> String {
    match pressure {
        Pressure::QnhHpa(v) => format!("QNH {} hPa", v),
        Pressure::AltimeterInHg(v) => format!("altimeter {:.2} inHg", v),
    }
}

/// Describes a military flight-condition color code in natural language.
///
/// When `black` is `true`, the description is prefixed with `"closed field"`.
///
/// # Examples
///
/// `GRN` → `"Green (GRN)"`
/// `BLACKAMB` → `"closed field – Amber (AMB)"`
pub fn describe_military_color(color: &MilitaryColor) -> String {
    let label = match color.code {
        MilitaryColorCode::Blu => "Blue (BLU)",
        MilitaryColorCode::Wht => "White (WHT)",
        MilitaryColorCode::Grn => "Green (GRN)",
        MilitaryColorCode::Ylo => "Yellow (YLO)",
        MilitaryColorCode::Amb => "Amber (AMB)",
        MilitaryColorCode::Red => "Red (RED)",
    };
    if color.black {
        format!("closed field – {}", label)
    } else {
        label.to_string()
    }
}

/// Describes a METAR trend detail (TEMPO, BECMG, NOSIG) in natural language.
pub fn describe_trend_detail<L: Locale>(detail: &MetarTrendDetail, locale: &L) -> String {
    let kind = locale.metar_trend(&detail.kind);
    let mut parts = vec![kind.to_string()];

    for t in &detail.times {
        let kind_str = locale.metar_trend_time_kind(&t.kind);
        parts.push(format!("{} {:02}:{:02}Z", kind_str, t.hour, t.minute));
    }

    if let Some(wind) = &detail.wind {
        parts.push(describe_wind(wind, locale));
    }
    if let Some(vis) = &detail.visibility {
        parts.push(describe_visibility(vis, locale));
    }
    for w in &detail.weather {
        parts.push(describe_weather(w, locale));
    }
    for c in &detail.clouds {
        parts.push(describe_cloud(c, locale));
    }

    if let Some(color) = &detail.color_code {
        parts.push(describe_military_color(color));
    }

    parts.join(", ")
}

/// Describes a Runway Visual Range group in natural language.
///
/// Example outputs:
/// - `"RVR runway 23: 500 m"`
/// - `"RVR runway 23: more than 1500 m, up to 2000 m (increasing)"`
pub fn describe_rvr(rvr: &RunwayVisualRange) -> String {
    let unit = match rvr.unit {
        RvrUnit::Meters => "m",
        RvrUnit::Feet => "ft",
    };

    let fmt_value = |v: &crate::metar::models::rvr::RvrValue| -> String {
        match &v.qualifier {
            None => format!("{} {}", v.value, unit),
            Some(RvrQualifier::Above) => format!("more than {} {}", v.value, unit),
            Some(RvrQualifier::Below) => format!("less than {} {}", v.value, unit),
        }
    };

    let range = match &rvr.max {
        None => fmt_value(&rvr.min),
        Some(max) => format!("{}, up to {}", fmt_value(&rvr.min), fmt_value(max)),
    };

    let tendency = match &rvr.tendency {
        None => String::new(),
        Some(RvrTendency::Upward) => " (increasing)".to_string(),
        Some(RvrTendency::Downward) => " (decreasing)".to_string(),
        Some(RvrTendency::NoChange) => " (no change)".to_string(),
    };

    format!(
        "RVR runway {}: {}{}",
        rvr.runway_designator, range, tendency
    )
}

/// Describes a runway state group in natural language.
///
/// Produces output such as
/// `"runway 19: dry snow, coverage 26–50%, thickness 2 mm, braking action 35"`.
/// For `R/SNOCLO` returns `"airfield closed due to snow/ice"`.
///
/// ICAO codes are translated to human-readable labels where defined; raw values
/// are shown for reserved or unknown codes.
pub fn describe_runway_state(rs: &RunwayState) -> String {
    if rs.snoclo {
        return "airfield closed due to snow/ice (SNOCLO)".to_string();
    }

    let deposit = rs
        .deposit_type
        .map(deposit_type_label)
        .unwrap_or("not reported");
    let coverage = rs.coverage.map(coverage_label).unwrap_or("not reported");

    let mut parts = vec![
        format!("runway {}", rs.runway_designator),
        format!("deposit: {}", deposit),
        format!("coverage: {}", coverage),
    ];

    match rs.thickness.as_deref() {
        None => parts.push("thickness: not reported".to_string()),
        Some(v) => parts.push(format!("thickness: {}", thickness_label(v))),
    }

    match rs.braking_action.as_deref() {
        None => parts.push("braking: not reported".to_string()),
        Some(v) => parts.push(format!("braking: {}", braking_label(v))),
    }

    parts.join(", ")
}

fn deposit_type_label(code: u8) -> &'static str {
    match code {
        0 => "clear and dry",
        1 => "damp",
        2 => "wet or water patches",
        3 => "rime or frost covered",
        4 => "dry snow",
        5 => "wet snow",
        6 => "slush",
        7 => "ice",
        8 => "compacted or rolled snow",
        9 => "frozen ruts or ridges",
        _ => "unknown deposit",
    }
}

fn coverage_label(code: u8) -> &'static str {
    match code {
        1 => "10% or less",
        2 => "11–25%",
        5 => "26–50%",
        9 => "51–100%",
        _ => "unknown coverage",
    }
}

fn thickness_label(code: &str) -> String {
    match code {
        "00" => "less than 1 mm".to_string(),
        "99" => "closed".to_string(),
        v => {
            if let Ok(n) = v.parse::<u8>() {
                match n {
                    92 => "10 cm".to_string(),
                    93 => "15 cm".to_string(),
                    94 => "20 cm".to_string(),
                    95 => "25 cm".to_string(),
                    96 => "30 cm".to_string(),
                    97 => "35 cm".to_string(),
                    98 => "40 cm or more".to_string(),
                    mm => format!("{} mm", mm),
                }
            } else {
                v.to_string()
            }
        }
    }
}

fn braking_label(code: &str) -> String {
    match code {
        "91" => "poor".to_string(),
        "92" => "medium/poor".to_string(),
        "93" => "medium".to_string(),
        "94" => "medium/good".to_string(),
        "95" => "good".to_string(),
        "99" => "figures unreliable".to_string(),
        v => {
            // Values 28–75 are friction coefficients (/100)
            if let Ok(n) = v.parse::<u8>()
                && n <= 75
            {
                return format!("µ = {:.2}", f32::from(n) / 100.0);
            }
            v.to_string()
        }
    }
}

/// Describes a TAF wind shear group in natural language.
pub fn describe_wind_shear(ws: &TafWindShear) -> String {
    format!(
        "wind shear at {} ft: {}° at {} kt",
        ws.height_hundreds_ft * 100,
        ws.direction,
        ws.speed_kt,
    )
}

/// Describes a TAF icing layer in natural language.
///
/// Example output: `"light icing from 4000 ft, depth 3000 ft"`.
pub fn describe_icing(icing: &Icing) -> String {
    let intensity = match &icing.intensity {
        IcingIntensity::None => "no icing",
        IcingIntensity::Light => "light icing",
        IcingIntensity::ModerateMixedOrRime => "moderate icing (mixed/rime)",
        IcingIntensity::ModerateGlaze => "moderate icing (glaze)",
        IcingIntensity::Severe => "severe icing",
        IcingIntensity::Unknown(_) => "icing",
    };
    format!(
        "{} from {} ft, depth {} ft",
        intensity, icing.base_ft, icing.thickness_ft
    )
}

/// Describes a TAF turbulence layer in natural language.
///
/// Example output: `"moderate turbulence (in-cloud) from 8000 ft, depth 2000 ft"`.
pub fn describe_turbulence(turb: &Turbulence) -> String {
    let intensity = match &turb.intensity {
        TurbulenceIntensity::None => "no turbulence",
        TurbulenceIntensity::Light => "light turbulence",
        TurbulenceIntensity::ModerateInCloud => "moderate turbulence (in-cloud)",
        TurbulenceIntensity::ModerateClearAir => "moderate turbulence (clear air)",
        TurbulenceIntensity::SevereInCloud => "severe turbulence (in-cloud)",
        TurbulenceIntensity::SevereClearAir => "severe turbulence (clear air)",
        TurbulenceIntensity::Extreme => "extreme turbulence",
        TurbulenceIntensity::Unknown(_) => "turbulence",
    };
    format!(
        "{} from {} ft, depth {} ft",
        intensity, turb.base_ft, turb.thickness_ft
    )
}

/// Describes a sea state group in natural language.
///
/// Example outputs:
/// - `"sea water 12°C, wave state 8 (WMO code)"`
/// - `"sea water -2°C, significant wave height 15 dm"`
/// - `"sea water temperature not available, wave state not available"`
pub fn describe_sea_state(ss: &SeaState) -> String {
    let temp = match ss.water_temperature {
        Some(t) => format!("sea water {}°C", t),
        None => "sea water temperature not available".to_string(),
    };

    let wave = match ss.wave_value {
        None => "wave state not available".to_string(),
        Some(v) => match ss.wave_kind {
            WaveHeightKind::StateCode => format!("wave state {} (WMO code)", v),
            WaveHeightKind::HeightDm => format!("significant wave height {} dm", v),
        },
    };

    format!("{}, {}", temp, wave)
}

/// Describes a METAR wind shear runway group in natural language.
///
/// Example outputs:
/// - `"wind shear on runway 23"`
/// - `"wind shear on all runways"`
pub fn describe_metar_wind_shear_runway(ws: &MetarWindShearRunway) -> String {
    match ws {
        MetarWindShearRunway::Runway(rwy) => format!("wind shear on runway {}", rwy),
        MetarWindShearRunway::AllRunways => "wind shear on all runways".to_string(),
    }
}

/// Produces a human-readable summary of the RMK section.
///
/// Structured items are described individually; unrecognised tokens are appended verbatim.
/// Returns `None` when the remarks section is empty.
pub fn describe_remarks(remarks: &Remarks) -> Option<String> {
    if remarks.raw.is_empty() {
        return None;
    }

    let mut parts: Vec<String> = remarks.items.iter().map(describe_remark).collect();
    parts.extend(remarks.unparsed.iter().cloned());

    Some(parts.join("; "))
}

fn describe_remark(r: &Remark) -> String {
    match r {
        Remark::PeakWind {
            direction,
            speed,
            hour,
            minute,
        } => {
            format!(
                "peak wind {}° at {} kt at {:02}:{:02}Z",
                direction, speed, hour, minute
            )
        }
        Remark::WindShift {
            hour,
            minute,
            frontal,
        } => {
            if *frontal {
                format!("wind shift at {:02}:{:02}Z (frontal passage)", hour, minute)
            } else {
                format!("wind shift at {:02}:{:02}Z", hour, minute)
            }
        }
        Remark::SeaLevelPressure(hpa) => format!("sea level pressure {:.1} hPa", hpa),
        Remark::PrecipitationAmount(inches) => {
            format!("precipitation {:.2} in", inches)
        }
        Remark::HourlyTemperature {
            temperature,
            dewpoint,
        } => {
            format!(
                "temperature {:.1}°C, dew point {:.1}°C",
                temperature, dewpoint
            )
        }
        Remark::MaxMinTemperature { max, min } => {
            format!("24h max {:.1}°C, min {:.1}°C", max, min)
        }
        Remark::PressureTendency {
            tendency_code,
            change_hpa,
        } => {
            format!(
                "pressure tendency code {} ({:.1} hPa)",
                tendency_code, change_hpa
            )
        }
        Remark::AutoStation(kind) => match kind {
            AutoStationKind::AO1 => {
                "automated station without precipitation discriminator (AO1)".to_string()
            }
            AutoStationKind::AO2 => {
                "automated station with precipitation discriminator (AO2)".to_string()
            }
        },
        Remark::Lightning { types, direction } => {
            let type_str: Vec<&str> = types
                .iter()
                .map(|t| match t {
                    LightningType::IC => "IC",
                    LightningType::CC => "CC",
                    LightningType::CA => "CA",
                    LightningType::CG => "CG",
                })
                .collect();
            let base = format!("lightning ({})", type_str.join("/"));
            match direction {
                None => base,
                Some(d) => format!("{} to the {}", base, d),
            }
        }
        Remark::MaintenanceIndicator => "maintenance check indicator".to_string(),
        Remark::Virga => "virga".to_string(),
        Remark::PressureRisingRapidly => "pressure rising rapidly".to_string(),
        Remark::PressureFallingRapidly => "pressure falling rapidly".to_string(),
        Remark::SensorStatus(code) => format!("sensor not available ({})", code),
        Remark::CloudAugmentation { amount, base_ft } => {
            let amount_str = match amount {
                CloudAmount::FEW => "few clouds",
                CloudAmount::SCT => "scattered clouds",
                CloudAmount::BKN => "broken clouds",
                CloudAmount::OVC => "overcast",
                _ => "clouds",
            };
            format!("{} at {} ft (type undeterminable)", amount_str, base_ft)
        }
        Remark::WindAtSensor {
            sensor_id,
            direction,
            speed,
            gust,
        } => {
            let dir_str = match direction {
                None => "variable".to_string(),
                Some(d) => format!("from {}°", d),
            };
            let base = format!("wind at sensor {} {} at {} kt", sensor_id, dir_str, speed);
            match gust {
                None => base,
                Some(g) => format!("{}, gusting {} kt", base, g),
            }
        }
    }
}
