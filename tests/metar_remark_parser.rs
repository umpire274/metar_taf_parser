use metar_taf_parser::metar::models::cloud::CloudAmount;
use metar_taf_parser::metar::models::remark::{AutoStationKind, LightningType, Remark};
use metar_taf_parser::parse_metar;
use metar_taf_parser::{Language, describe_metar};

// ---------------------------------------------------------------------------
// AutoStation
// ---------------------------------------------------------------------------

#[test]
fn remark_ao2_parsed() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK AO2").unwrap();
    assert!(
        m.remarks
            .items
            .contains(&Remark::AutoStation(AutoStationKind::AO2))
    );
    assert_eq!(m.remarks.raw, "AO2");
}

#[test]
fn remark_ao1_parsed() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK AO1").unwrap();
    assert!(
        m.remarks
            .items
            .contains(&Remark::AutoStation(AutoStationKind::AO1))
    );
}

// ---------------------------------------------------------------------------
// Sea level pressure
// ---------------------------------------------------------------------------

#[test]
fn remark_slp_low_value() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK SLP132").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::SeaLevelPressure(v) if (v - 1013.2).abs() < 0.01
    ));
}

#[test]
fn remark_slp_high_value() {
    // SLP982 → 900 + 98.2 = 998.2 hPa
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK SLP982").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::SeaLevelPressure(v) if (v - 998.2).abs() < 0.01
    ));
}

// ---------------------------------------------------------------------------
// Peak wind
// ---------------------------------------------------------------------------

#[test]
fn remark_peak_wind_parsed() {
    let m =
        parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK PK WND 18025/1432").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::PeakWind {
            direction: 180,
            speed: 25,
            hour: 14,
            minute: 32
        }
    ));
}

// ---------------------------------------------------------------------------
// Wind shift
// ---------------------------------------------------------------------------

#[test]
fn remark_wind_shift_without_fropa() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK WSHFT 1432").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::WindShift {
            hour: 14,
            minute: 32,
            frontal: false
        }
    ));
}

#[test]
fn remark_wind_shift_with_fropa() {
    let m =
        parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK WSHFT 1432 FROPA").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::WindShift {
            hour: 14,
            minute: 32,
            frontal: true
        }
    ));
}

// ---------------------------------------------------------------------------
// Precipitation
// ---------------------------------------------------------------------------

#[test]
fn remark_precipitation_amount() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK P0015").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::PrecipitationAmount(v) if (v - 0.15).abs() < 0.001
    ));
}

// ---------------------------------------------------------------------------
// Hourly temperature
// ---------------------------------------------------------------------------

#[test]
fn remark_hourly_temperature_positive() {
    // T02560178 → temp=25.6°C, dew=17.8°C
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK T02560178").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::HourlyTemperature { temperature: t, dewpoint: d }
            if (t - 25.6).abs() < 0.01 && (d - 17.8).abs() < 0.01
    ));
}

#[test]
fn remark_hourly_temperature_negative() {
    // T11560178 → temp=-15.6°C, dew=17.8°C
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK T11560178").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::HourlyTemperature { temperature: t, dewpoint: _ }
            if (t - (-15.6)).abs() < 0.01
    ));
}

// ---------------------------------------------------------------------------
// Max/Min temperature
// ---------------------------------------------------------------------------

#[test]
fn remark_max_min_temperature() {
    // 401280056 → max=12.8°C, min=5.6°C
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK 401280056").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::MaxMinTemperature { max: mx, min: mn }
            if (mx - 12.8).abs() < 0.01 && (mn - 5.6).abs() < 0.01
    ));
}

// ---------------------------------------------------------------------------
// Pressure tendency
// ---------------------------------------------------------------------------

#[test]
fn remark_pressure_tendency() {
    // 50132 → code=0, change=13.2 hPa
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK 50132").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::PressureTendency { tendency_code: 0, change_hpa: c }
            if (c - 13.2).abs() < 0.01
    ));
}

// ---------------------------------------------------------------------------
// Maintenance indicator
// ---------------------------------------------------------------------------

#[test]
fn remark_maintenance_indicator() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK $").unwrap();
    assert!(m.remarks.items.contains(&Remark::MaintenanceIndicator));
}

// ---------------------------------------------------------------------------
// Virga
// ---------------------------------------------------------------------------

#[test]
fn remark_virga() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK VIRGA").unwrap();
    assert!(m.remarks.items.contains(&Remark::Virga));
}

// ---------------------------------------------------------------------------
// Pressure rising/falling rapidly
// ---------------------------------------------------------------------------

#[test]
fn remark_presrr() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK PRESRR").unwrap();
    assert!(m.remarks.items.contains(&Remark::PressureRisingRapidly));
}

#[test]
fn remark_presfr() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK PRESFR").unwrap();
    assert!(m.remarks.items.contains(&Remark::PressureFallingRapidly));
}

// ---------------------------------------------------------------------------
// Sensor status
// ---------------------------------------------------------------------------

#[test]
fn remark_rvrno() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK RVRNO").unwrap();
    assert!(
        m.remarks
            .items
            .contains(&Remark::SensorStatus("RVRNO".to_string()))
    );
}

// ---------------------------------------------------------------------------
// Lightning
// ---------------------------------------------------------------------------

#[test]
fn remark_lightning_cg() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK LTGCG").unwrap();
    match &m.remarks.items[0] {
        Remark::Lightning { types, direction } => {
            assert!(types.contains(&LightningType::CG));
            assert!(direction.is_none());
        }
        other => panic!("expected Lightning, got {:?}", other),
    }
}

#[test]
fn remark_lightning_multiple_types() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK LTGICCC").unwrap();
    match &m.remarks.items[0] {
        Remark::Lightning { types, .. } => {
            assert!(types.contains(&LightningType::IC));
            assert!(types.contains(&LightningType::CC));
        }
        other => panic!("expected Lightning, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Unknown tokens fall into unparsed
// ---------------------------------------------------------------------------

#[test]
fn remark_unknown_token_goes_to_unparsed() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK QFE746/0995").unwrap();
    assert!(m.remarks.items.is_empty());
    assert!(m.remarks.unparsed.contains(&"QFE746/0995".to_string()));
    assert_eq!(m.remarks.raw, "QFE746/0995");
}

// ---------------------------------------------------------------------------
// Multiple remarks in one section
// ---------------------------------------------------------------------------

#[test]
fn remark_multiple_parsed() {
    let m = parse_metar("KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK AO2 SLP132 T02560178")
        .unwrap();

    assert_eq!(m.remarks.items.len(), 3);
    assert!(
        m.remarks
            .items
            .contains(&Remark::AutoStation(AutoStationKind::AO2))
    );
    assert!(matches!(m.remarks.items[1], Remark::SeaLevelPressure(_)));
    assert!(matches!(
        m.remarks.items[2],
        Remark::HourlyTemperature { .. }
    ));
}

// ---------------------------------------------------------------------------
// Empty RMK section
// ---------------------------------------------------------------------------

#[test]
fn no_rmk_section_gives_empty_remarks() {
    let m = parse_metar("LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015").unwrap();
    assert!(m.remarks.raw.is_empty());
    assert!(m.remarks.items.is_empty());
    assert!(m.remarks.unparsed.is_empty());
}

// ---------------------------------------------------------------------------
// CloudAugmentation — OVC014/// e varianti
// ---------------------------------------------------------------------------

#[test]
fn remark_ovc_cloud_augmentation_parsed() {
    // RMK OVC014/// — overcast a 1400 ft, tipo non determinabile
    let m = parse_metar("ENGM 121250Z 18010KT 9999 OVC020 18/12 Q1015 RMK OVC014///").unwrap();
    assert!(
        matches!(
            m.remarks.items[0],
            Remark::CloudAugmentation {
                amount: CloudAmount::OVC,
                base_ft: 1400,
            }
        ),
        "unexpected remark: {:?}",
        m.remarks.items
    );
}

#[test]
fn remark_bkn_cloud_augmentation_parsed() {
    let m = parse_metar("ENGM 121250Z 18010KT 9999 BKN020 18/12 Q1015 RMK BKN020///").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::CloudAugmentation {
            amount: CloudAmount::BKN,
            base_ft: 2000,
        }
    ));
}

#[test]
fn remark_few_cloud_augmentation_parsed() {
    let m = parse_metar("ENGM 121250Z 18010KT 9999 FEW030 18/12 Q1015 RMK FEW030///").unwrap();
    assert!(matches!(
        m.remarks.items[0],
        Remark::CloudAugmentation {
            amount: CloudAmount::FEW,
            base_ft: 3000,
        }
    ));
}

#[test]
fn remark_cloud_augmentation_not_confused_with_normal_cloud() {
    // Nel corpo del METAR OVC014CB è un cloud normale; dopo RMK OVC014/// è augmentation
    let m = parse_metar("ENGM 121250Z 18010KT 9999 OVC014CB 18/12 Q1015 RMK OVC014///").unwrap();
    // Il cloud nel corpo rimane intatto
    assert_eq!(m.clouds[0].amount, CloudAmount::OVC);
    // Nel RMK viene riconosciuta la CloudAugmentation
    assert!(matches!(
        m.remarks.items[0],
        Remark::CloudAugmentation { base_ft: 1400, .. }
    ));
}

#[test]
fn remark_cloud_augmentation_not_parsed_without_triple_slash() {
    // OVC014CB NON è una CloudAugmentation: finisce in unparsed
    let m = parse_metar("ENGM 121250Z 18010KT 9999 FEW030 18/12 Q1015 RMK OVC014CB").unwrap();
    assert!(m.remarks.items.is_empty());
    assert!(m.remarks.unparsed.contains(&"OVC014CB".to_string()));
}

// ---------------------------------------------------------------------------
// WindAtSensor — WIND <id> <wind_group>
// ---------------------------------------------------------------------------

#[test]
fn remark_wind_at_sensor_vrb_parsed() {
    // Esempio dal manuale: WIND SKEID VRB01G22KT
    let m = parse_metar(
        "ENBR 121250Z 18010KT 9999 OVC020 10/08 Q1013 RMK OVC014/// WIND SKEID VRB01G22KT",
    )
    .unwrap();

    // Il primo remark è CloudAugmentation
    assert!(matches!(
        m.remarks.items[0],
        Remark::CloudAugmentation { base_ft: 1400, .. }
    ));

    // Il secondo remark è WindAtSensor
    match &m.remarks.items[1] {
        Remark::WindAtSensor {
            sensor_id,
            direction,
            speed,
            gust,
        } => {
            assert_eq!(sensor_id, "SKEID");
            assert_eq!(*direction, None, "VRB deve dare direction=None");
            assert_eq!(*speed, 1);
            assert_eq!(*gust, Some(22));
        }
        other => panic!("atteso WindAtSensor, trovato {:?}", other),
    }
}

#[test]
fn remark_wind_at_sensor_fixed_direction() {
    // WIND LOCREF 18010KT — vento fisso a 180° 10 kt
    let m = parse_metar("ENBR 121250Z 18010KT 9999 FEW030 10/08 Q1013 RMK WIND LOCREF 18010KT")
        .unwrap();
    match &m.remarks.items[0] {
        Remark::WindAtSensor {
            sensor_id,
            direction,
            speed,
            gust,
        } => {
            assert_eq!(sensor_id, "LOCREF");
            assert_eq!(*direction, Some(180));
            assert_eq!(*speed, 10);
            assert_eq!(*gust, None);
        }
        other => panic!("atteso WindAtSensor, trovato {:?}", other),
    }
}

#[test]
fn remark_wind_at_sensor_with_gust_fixed_direction() {
    // WIND ANEMREF 24015G25KT
    let m = parse_metar("ENBR 121250Z 18010KT 9999 FEW030 10/08 Q1013 RMK WIND ANEMREF 24015G25KT")
        .unwrap();
    match &m.remarks.items[0] {
        Remark::WindAtSensor {
            direction,
            speed,
            gust,
            ..
        } => {
            assert_eq!(*direction, Some(240));
            assert_eq!(*speed, 15);
            assert_eq!(*gust, Some(25));
        }
        other => panic!("atteso WindAtSensor, trovato {:?}", other),
    }
}

#[test]
fn remark_wind_at_sensor_invalid_wind_falls_to_unparsed() {
    // Se il token vento è malformato, WIND non viene riconosciuto e i token finiscono in unparsed
    let m =
        parse_metar("ENBR 121250Z 18010KT 9999 FEW030 10/08 Q1013 RMK WIND SKEID INVALID").unwrap();
    assert!(m.remarks.items.is_empty());
    assert!(m.remarks.unparsed.contains(&"WIND".to_string()));
}

// ---------------------------------------------------------------------------
// Describe — CloudAugmentation e WindAtSensor
// ---------------------------------------------------------------------------

#[test]
fn describe_cloud_augmentation() {
    let m = parse_metar("ENGM 121250Z 18010KT 9999 OVC020 18/12 Q1015 RMK OVC014///").unwrap();
    let desc = describe_metar(&m, Language::En);
    let rmk = desc.remarks.as_ref().expect("remarks should be described");
    assert!(rmk.contains("overcast"), "deve contenere 'overcast': {rmk}");
    assert!(rmk.contains("1400"), "deve contenere '1400': {rmk}");
    assert!(
        rmk.contains("undeterminable"),
        "deve contenere 'undeterminable': {rmk}"
    );
}

#[test]
fn describe_wind_at_sensor() {
    let m = parse_metar("ENBR 121250Z 18010KT 9999 OVC020 10/08 Q1013 RMK WIND SKEID VRB01G22KT")
        .unwrap();
    let desc = describe_metar(&m, Language::En);
    let rmk = desc.remarks.as_ref().expect("remarks should be described");
    assert!(rmk.contains("SKEID"), "deve contenere 'SKEID': {rmk}");
    assert!(rmk.contains("variable"), "deve contenere 'variable': {rmk}");
    assert!(rmk.contains("22"), "deve contenere la raffica '22': {rmk}");
}
