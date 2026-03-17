//! Module `rmk`.
//!
//! Contains types and parsing logic implemented for this crate.
//!
//! Parser for the METAR RMK section.
//!
//! Converts raw remark tokens into strongly-typed [`Remark`] variants.
//! Tokens that do not match any known pattern are preserved in [`Remarks::unparsed`].

use crate::metar::models::cloud::CloudAmount;
use crate::metar::models::remark::{AutoStationKind, LightningType, Remark, Remarks};

/// Parses a slice of remark tokens into a structured [`Remarks`] value.
///
/// Multi-token groups (e.g. `PK WND dddff/HHmm`, `WSHFT HHmm [FROPA]`) are
/// consumed as a unit. Unrecognised tokens are placed in [`Remarks::unparsed`].
/// The original text is preserved verbatim in [`Remarks::raw`].
pub fn parse_rmk(tokens: &[String]) -> Remarks {
    if tokens.is_empty() {
        return Remarks::default();
    }

    let raw = tokens.join(" ");
    let mut items = Vec::new();
    let mut unparsed = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        // PK WND dddff/HHmm  (3 tokens)
        if tokens[i] == "PK"
            && i + 2 < tokens.len()
            && tokens[i + 1] == "WND"
            && let Some(pk) = parse_peak_wind(&tokens[i + 2])
        {
            items.push(pk);
            i += 3;
            continue;
        }

        // WSHFT HHmm [FROPA]
        if tokens[i] == "WSHFT"
            && i + 1 < tokens.len()
            && let Some((hour, minute)) = parse_hhmm(&tokens[i + 1])
        {
            let frontal = i + 2 < tokens.len() && tokens[i + 2] == "FROPA";
            items.push(Remark::WindShift {
                hour,
                minute,
                frontal,
            });
            i += if frontal { 3 } else { 2 };
            continue;
        }

        // WIND <sensor_id> <wind_group>  (3 tokens)
        if tokens[i] == "WIND"
            && i + 2 < tokens.len()
            && let Some((direction, speed, gust)) = parse_wind_token(&tokens[i + 2])
        {
            items.push(Remark::WindAtSensor {
                sensor_id: tokens[i + 1].clone(),
                direction,
                speed,
                gust,
            });
            i += 3;
            continue;
        }

        if let Some(r) = parse_single_token(&tokens[i]) {
            items.push(r);
            i += 1;
            continue;
        }

        unparsed.push(tokens[i].clone());
        i += 1;
    }

    Remarks {
        items,
        unparsed,
        raw,
    }
}

// ---------------------------------------------------------------------------
// Single-token parsers
// ---------------------------------------------------------------------------

fn parse_single_token(token: &str) -> Option<Remark> {
    if let Some(r) = parse_slp(token) {
        return Some(r);
    }
    if let Some(r) = parse_precipitation(token) {
        return Some(r);
    }
    if let Some(r) = parse_hourly_temperature(token) {
        return Some(r);
    }
    if let Some(r) = parse_max_min_temperature(token) {
        return Some(r);
    }
    if let Some(r) = parse_pressure_tendency(token) {
        return Some(r);
    }
    if let Some(r) = parse_cloud_augmentation(token) {
        return Some(r);
    }

    match token {
        "AO1" => return Some(Remark::AutoStation(AutoStationKind::AO1)),
        "AO2" => return Some(Remark::AutoStation(AutoStationKind::AO2)),
        "$" => return Some(Remark::MaintenanceIndicator),
        "VIRGA" => return Some(Remark::Virga),
        "PRESRR" => return Some(Remark::PressureRisingRapidly),
        "PRESFR" => return Some(Remark::PressureFallingRapidly),
        "RVRNO" | "PWINO" | "TSNO" | "VISNO" | "CHINO" => {
            return Some(Remark::SensorStatus(token.to_string()));
        }
        _ => {}
    }

    if token.starts_with("LTG") {
        return Some(parse_lightning(token));
    }

    None
}

// ---------------------------------------------------------------------------
// Multi-token helpers
// ---------------------------------------------------------------------------

/// Parses `dddff/HHmm` into a `PeakWind` remark.
fn parse_peak_wind(token: &str) -> Option<Remark> {
    let (wind_part, time_part) = token.split_once('/')?;

    // Direction (3 digits) + speed (2+ digits)
    if wind_part.len() < 5 || !wind_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let direction: u16 = wind_part[..3].parse().ok()?;
    let speed: u16 = wind_part[3..].parse().ok()?;

    if direction > 360 {
        return None;
    }

    let (hour, minute) = parse_hhmm(time_part)?;

    Some(Remark::PeakWind {
        direction,
        speed,
        hour,
        minute,
    })
}

/// Parses a 4-digit `HHmm` time string.
fn parse_hhmm(token: &str) -> Option<(u8, u8)> {
    if token.len() != 4 || !token.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let hour: u8 = token[..2].parse().ok()?;
    let minute: u8 = token[2..].parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }
    Some((hour, minute))
}

// ---------------------------------------------------------------------------
// Single-token helpers
// ---------------------------------------------------------------------------

/// Parses `SLPxxx` into a [`Remark::SeaLevelPressure`].
///
/// Values below 500 are assumed to be in the 1000–1049 hPa range;
/// values 500 and above are assumed to be in the 950–999 hPa range.
fn parse_slp(token: &str) -> Option<Remark> {
    let body = token.strip_prefix("SLP")?;
    if body.len() != 3 || !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let raw: u32 = body.parse().ok()?;
    let base = if raw < 500 { 1000.0_f32 } else { 900.0_f32 };
    Some(Remark::SeaLevelPressure(base + (raw as f32) / 10.0))
}

/// Parses `Pxxxx` into a [`Remark::PrecipitationAmount`] (hundredths of an inch).
fn parse_precipitation(token: &str) -> Option<Remark> {
    let body = token.strip_prefix('P')?;
    if body.len() != 4 || !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let raw: u32 = body.parse().ok()?;
    Some(Remark::PrecipitationAmount((raw as f32) / 100.0))
}

/// Parses `Tsssstdddd` into a [`Remark::HourlyTemperature`].
///
/// Each nibble: sign digit (0=positive, 1=negative) followed by 3 digits in tenths °C.
fn parse_hourly_temperature(token: &str) -> Option<Remark> {
    let body = token.strip_prefix('T')?;
    if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let temp_sign = &body[0..1];
    let temp_digits: f32 = body[1..4].parse().ok()?;
    let dew_sign = &body[4..5];
    let dew_digits: f32 = body[5..8].parse().ok()?;

    let temperature = if temp_sign == "1" {
        -temp_digits / 10.0
    } else {
        temp_digits / 10.0
    };
    let dewpoint = if dew_sign == "1" {
        -dew_digits / 10.0
    } else {
        dew_digits / 10.0
    };

    Some(Remark::HourlyTemperature {
        temperature,
        dewpoint,
    })
}

/// Parses `4sTTTsTTT` (9 chars) into a [`Remark::MaxMinTemperature`].
///
/// Format: `4` + sign (0/1) + 3 digits (tenths °C) for max, same for min.
fn parse_max_min_temperature(token: &str) -> Option<Remark> {
    if token.len() != 9 {
        return None;
    }
    let bytes = token.as_bytes();
    if bytes[0] != b'4' {
        return None;
    }
    if !token[1..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let max_sign = &token[1..2];
    let max_digits: f32 = token[2..5].parse().ok()?;
    let min_sign = &token[5..6];
    let min_digits: f32 = token[6..9].parse().ok()?;

    let max = if max_sign == "1" {
        -max_digits / 10.0
    } else {
        max_digits / 10.0
    };
    let min = if min_sign == "1" {
        -min_digits / 10.0
    } else {
        min_digits / 10.0
    };

    Some(Remark::MaxMinTemperature { max, min })
}

/// Parses `5Attt` (5 chars) into a [`Remark::PressureTendency`].
///
/// Format: `5` + 1 digit tendency code (0–8) + 3 digits change in tenths hPa.
fn parse_pressure_tendency(token: &str) -> Option<Remark> {
    if token.len() != 5 {
        return None;
    }
    let bytes = token.as_bytes();
    if bytes[0] != b'5' {
        return None;
    }
    if !token[1..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let tendency_code: u8 = token[1..2].parse().ok()?;
    let change_raw: u32 = token[2..5].parse().ok()?;
    let change_hpa = (change_raw as f32) / 10.0;

    Some(Remark::PressureTendency {
        tendency_code,
        change_hpa,
    })
}

/// Parses a lightning token starting with `LTG` into a [`Remark::Lightning`].
///
/// Lightning type codes (`IC`, `CC`, `CA`, `CG`) are extracted from the suffix;
/// any remaining text is treated as a direction description.
fn parse_lightning(token: &str) -> Remark {
    let body = token.strip_prefix("LTG").unwrap_or("");
    let mut types = Vec::new();
    let mut remaining = body;

    while remaining.len() >= 2 {
        match &remaining[..2] {
            "IC" => {
                types.push(LightningType::IC);
                remaining = &remaining[2..];
            }
            "CC" => {
                types.push(LightningType::CC);
                remaining = &remaining[2..];
            }
            "CA" => {
                types.push(LightningType::CA);
                remaining = &remaining[2..];
            }
            "CG" => {
                types.push(LightningType::CG);
                remaining = &remaining[2..];
            }
            _ => break,
        }
    }

    let direction = if remaining.is_empty() {
        None
    } else {
        Some(remaining.to_string())
    };

    Remark::Lightning { types, direction }
}

/// Parses an ASOS cloud augmentation token of the form `CCCddd///`.
///
/// This format appears in the RMK section of automated station METARs to indicate
/// a cloud ceiling where the type (CB/TCU) cannot be determined by the sensor.
/// Example: `OVC014///` → overcast at 1400 ft, type undeterminable.
fn parse_cloud_augmentation(token: &str) -> Option<Remark> {
    // Must be exactly 9 chars: 3 (amount) + 3 (altitude hundreds) + 3 ("///")
    if token.len() != 9 || !token.ends_with("///") {
        return None;
    }

    let amount = match &token[0..3] {
        "FEW" => CloudAmount::FEW,
        "SCT" => CloudAmount::SCT,
        "BKN" => CloudAmount::BKN,
        "OVC" => CloudAmount::OVC,
        _ => return None,
    };

    let altitude_str = &token[3..6];
    if !altitude_str.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let altitude_hundreds: u16 = altitude_str.parse().ok()?;

    Some(Remark::CloudAugmentation {
        amount,
        base_ft: altitude_hundreds * 100,
    })
}

/// Parses a wind group token (e.g. `VRB01G22KT`, `18010KT`, `24015G25MPS`) into
/// its directional and speed components, for use within the RMK wind-at-sensor group.
///
/// Returns `(direction, speed, gust)` where `direction` is `None` for variable wind.
fn parse_wind_token(token: &str) -> Option<(Option<u16>, u16, Option<u16>)> {
    let core = token
        .strip_suffix("KT")
        .or_else(|| token.strip_suffix("MPS"))?;

    // Variable wind: VRBssGggKT or VRBssKT
    if let Some(speed_part) = core.strip_prefix("VRB") {
        let (speed, gust) = parse_speed_gust(speed_part)?;
        return Some((None, speed, gust));
    }

    // Fixed direction: dddssGggKT or dddssKT (at least 5 chars: 3 dir + 2 speed)
    if core.len() < 5 || !core[..3].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let direction: u16 = core[..3].parse().ok()?;
    if direction > 360 {
        return None;
    }

    let (speed, gust) = parse_speed_gust(&core[3..])?;
    Some((Some(direction), speed, gust))
}

/// Splits `ssGgg` or `ss` into `(speed, Some(gust))` or `(speed, None)`.
fn parse_speed_gust(s: &str) -> Option<(u16, Option<u16>)> {
    if let Some((spd, gst)) = s.split_once('G') {
        let speed: u16 = spd.parse().ok()?;
        let gust: u16 = gst.parse().ok()?;
        Some((speed, Some(gust)))
    } else {
        let speed: u16 = s.parse().ok()?;
        Some((speed, None))
    }
}

