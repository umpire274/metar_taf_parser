//! Remark types for METAR messages.
//!
//! Defines the structured types used to represent parsed METAR remark (RMK) groups.

use crate::metar::models::cloud::CloudAmount;
use serde::Serialize;

/// Indicates the type of automated station.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AutoStationKind {
    /// Station without precipitation discriminator (AO1).
    AO1,
    /// Station with precipitation discriminator (AO2).
    AO2,
}

/// Type of lightning discharge observed.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LightningType {
    /// In-cloud.
    IC,
    /// Cloud-to-cloud.
    CC,
    /// Cloud-to-air.
    CA,
    /// Cloud-to-ground.
    CG,
}

/// A structured parsed remark group from the METAR RMK section.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Remark {
    /// Peak wind observation: direction in degrees, speed in knots, observation time.
    PeakWind {
        direction: u16,
        speed: u16,
        hour: u8,
        minute: u8,
    },
    /// Wind shift at the given time, with optional frontal passage indicator.
    WindShift { hour: u8, minute: u8, frontal: bool },
    /// Sea level pressure in hPa (one decimal), e.g. `SLP132` = 1013.2 hPa.
    SeaLevelPressure(f32),
    /// Hourly precipitation amount in inches (two decimals), e.g. `P0015` = 0.15 in.
    PrecipitationAmount(f32),
    /// Hourly temperature and dew point in tenths of degrees Celsius.
    ///
    /// Encoded as `Tsssstdddd`: first nibble is sign+temp, second is sign+dewpoint.
    HourlyTemperature { temperature: f32, dewpoint: f32 },
    /// 24-hour maximum and minimum temperature in tenths of degrees Celsius.
    ///
    /// Encoded as `4sTTTsTTT`.
    MaxMinTemperature { max: f32, min: f32 },
    /// 3-hour pressure tendency.
    ///
    /// `tendency_code` is ICAO code 0–8; `change_hpa` is the change in hPa (tenths).
    PressureTendency { tendency_code: u8, change_hpa: f32 },
    /// Automated station type.
    AutoStation(AutoStationKind),
    /// Lightning observation with discharge types and optional direction text.
    Lightning {
        types: Vec<LightningType>,
        direction: Option<String>,
    },
    /// Maintenance check indicator (`$`).
    MaintenanceIndicator,
    /// Virga observed.
    Virga,
    /// Pressure rising rapidly (`PRESRR`).
    PressureRisingRapidly,
    /// Pressure falling rapidly (`PRESFR`).
    PressureFallingRapidly,
    /// Sensor status indicator, e.g. `RVRNO`, `PWINO`, `TSNO`, `VISNO`, `CHINO`.
    SensorStatus(String),
    /// Cloud ceiling reported by an automated sensor where the cloud type (CB/TCU)
    /// is not measurable. Appears in RMK as e.g. `OVC014///`, `BKN020///`.
    ///
    /// This is the standard ASOS/AWOS augmented cloud format used at automated stations.
    CloudAugmentation {
        /// Cloud coverage amount.
        amount: CloudAmount,
        /// Base altitude in feet.
        base_ft: u16,
    },
    /// Wind observation from a named secondary sensor or anemometer.
    ///
    /// Appears in RMK as `WIND <sensor_id> <wind_group>`,
    /// e.g. `WIND SKEID VRB01G22KT`.
    WindAtSensor {
        /// Identifier of the secondary sensor or reference station.
        sensor_id: String,
        /// Wind direction in degrees. `None` indicates variable direction (VRB).
        direction: Option<u16>,
        /// Wind speed in knots (or MPS, depending on originating source).
        speed: u16,
        /// Gust speed, if reported.
        gust: Option<u16>,
    },
}

/// The collection of parsed and unparsed remark groups for a METAR message.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Remarks {
    /// Structured parsed remark groups, in order of appearance.
    pub items: Vec<Remark>,
    /// Tokens that could not be parsed into a known remark type.
    pub unparsed: Vec<String>,
    /// The raw RMK section text, exactly as it appeared in the message.
    pub raw: String,
}
