//! English locale for METAR/TAF natural language descriptions.

use super::Locale;
use crate::common::report_modifier::ReportModifier;
use crate::metar::models::cloud::{CloudAmount, CloudType};
use crate::metar::models::trend::{MetarTrend, MetarTrendTimeKind};
use crate::metar::models::visibility::VisibilityDirection;
use crate::metar::models::weather::{WeatherDescriptor, WeatherIntensity, WeatherPhenomenon};
use crate::metar::models::wind::WindUnit;
use crate::taf::models::forecast::TafForecastKind;

/// English locale implementation of [`Locale`].
pub struct En;

impl Locale for En {
    fn cloud_amount(&self, amount: &CloudAmount) -> &'static str {
        match amount {
            CloudAmount::FEW => "few clouds",
            CloudAmount::SCT => "scattered clouds",
            CloudAmount::BKN => "broken clouds",
            CloudAmount::OVC => "overcast",
            CloudAmount::NSC => "no significant clouds",
            CloudAmount::SKC => "sky clear",
            CloudAmount::VV => "vertical visibility",
            CloudAmount::NCD => "no clouds detected",
            CloudAmount::CLR => "no cloud below 12,000 ft",
        }
    }

    fn cloud_type(&self, t: &CloudType) -> &'static str {
        match t {
            CloudType::CB => "cumulonimbus",
            CloudType::TCU => "towering cumulus",
        }
    }

    fn wind_unit(&self, unit: &WindUnit) -> &'static str {
        match unit {
            WindUnit::KT => "kt",
            WindUnit::MPS => "m/s",
            WindUnit::MPH => "mph",
        }
    }

    fn weather_intensity(&self, i: &WeatherIntensity) -> &'static str {
        match i {
            WeatherIntensity::Light => "light",
            WeatherIntensity::Moderate => "moderate",
            WeatherIntensity::Heavy => "heavy",
            WeatherIntensity::Recent => "recent",
        }
    }

    fn weather_descriptor(&self, d: &WeatherDescriptor) -> &'static str {
        match d {
            WeatherDescriptor::Shallow => "shallow",
            WeatherDescriptor::Partial => "partial",
            WeatherDescriptor::Patches => "patches of",
            WeatherDescriptor::LowDrifting => "low drifting",
            WeatherDescriptor::Blowing => "blowing",
            WeatherDescriptor::Showers => "showers of",
            WeatherDescriptor::Thunderstorm => "thunderstorm with",
            WeatherDescriptor::Freezing => "freezing",
            WeatherDescriptor::Vicinity => "in the vicinity",
        }
    }

    fn weather_phenomenon(&self, p: &WeatherPhenomenon) -> String {
        match p {
            WeatherPhenomenon::Rain => "rain".to_string(),
            WeatherPhenomenon::Snow => "snow".to_string(),
            WeatherPhenomenon::Drizzle => "drizzle".to_string(),
            WeatherPhenomenon::Thunder => "thunderstorm".to_string(),
            WeatherPhenomenon::Fog => "fog".to_string(),
            WeatherPhenomenon::Mist => "mist".to_string(),
            WeatherPhenomenon::Haze => "haze".to_string(),
            WeatherPhenomenon::Smoke => "smoke".to_string(),
            WeatherPhenomenon::Hail => "hail".to_string(),
            WeatherPhenomenon::SmallHail => "small hail".to_string(),
            WeatherPhenomenon::IcePellets => "ice pellets".to_string(),
            WeatherPhenomenon::IceCrystals => "ice crystals".to_string(),
            WeatherPhenomenon::SnowGrains => "snow grains".to_string(),
            WeatherPhenomenon::SandWhirls => "sand whirls".to_string(),
            WeatherPhenomenon::Squalls => "squalls".to_string(),
            WeatherPhenomenon::FunnelCloud => "funnel cloud".to_string(),
            WeatherPhenomenon::Sand => "sand".to_string(),
            WeatherPhenomenon::Dust => "widespread dust".to_string(),
            WeatherPhenomenon::DustStorm => "dust storm".to_string(),
            WeatherPhenomenon::SandStorm => "sand storm".to_string(),
            WeatherPhenomenon::Spray => "spray".to_string(),
            WeatherPhenomenon::VolcanicAsh => "volcanic ash".to_string(),
            WeatherPhenomenon::UnknownPrecipitation => "unknown precipitation".to_string(),
            WeatherPhenomenon::NoSignificantWeather => "no significant weather".to_string(),
            WeatherPhenomenon::Unknown(code) => format!("unknown weather ({})", code),
        }
    }

    fn visibility_direction(&self, d: &VisibilityDirection) -> &'static str {
        match d {
            VisibilityDirection::N => "north",
            VisibilityDirection::NE => "north-east",
            VisibilityDirection::E => "east",
            VisibilityDirection::SE => "south-east",
            VisibilityDirection::S => "south",
            VisibilityDirection::SW => "south-west",
            VisibilityDirection::W => "west",
            VisibilityDirection::NW => "north-west",
        }
    }

    fn report_modifier(&self, m: &ReportModifier) -> Option<&'static str> {
        match m {
            ReportModifier::Normal => None,
            ReportModifier::Auto => Some("automated report"),
            ReportModifier::Correction => Some("corrected report"),
            ReportModifier::Amendment => Some("amended forecast"),
            ReportModifier::Nil => Some("no data available"),
        }
    }

    fn forecast_kind(&self, k: &TafForecastKind) -> &'static str {
        match k {
            TafForecastKind::Base => "Base forecast",
            TafForecastKind::FM => "From",
            TafForecastKind::BECMG => "Becoming",
            TafForecastKind::TEMPO => "Temporary",
            TafForecastKind::PROB => "Probability",
        }
    }

    fn metar_trend(&self, t: &MetarTrend) -> &'static str {
        match t {
            MetarTrend::Nosig => "no significant change",
            MetarTrend::Becmg => "becoming",
            MetarTrend::Tempo => "temporarily",
        }
    }

    fn metar_trend_time_kind(&self, k: &MetarTrendTimeKind) -> &'static str {
        match k {
            MetarTrendTimeKind::From => "from",
            MetarTrendTimeKind::Until => "until",
            MetarTrendTimeKind::At => "at",
        }
    }
}
