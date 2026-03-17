use metar_taf_parser::parse_taf;
use metar_taf_parser::taf::models::icing::{Icing, IcingIntensity};
use metar_taf_parser::taf::models::turbulence::{Turbulence, TurbulenceIntensity};

// ---------------------------------------------------------------------------
// Icing
// ---------------------------------------------------------------------------

#[test]
fn taf_icing_light_parsed() {
    // 610304 → 6 + intensity=1(light) + base=030 (3000 ft) + thickness=4 (4000 ft)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 610304").unwrap();

    let icing = &taf.forecasts[0].icing;
    assert_eq!(icing.len(), 1);
    assert_eq!(icing[0].intensity, IcingIntensity::Light);
    assert_eq!(icing[0].base_ft, 3000);
    assert_eq!(icing[0].thickness_ft, 4000);
}

#[test]
fn taf_icing_severe_parsed() {
    // 640502 → 6 + intensity=4(severe) + base=050 (5000 ft) + thickness=2 (2000 ft)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 640502").unwrap();

    let icing = &taf.forecasts[0].icing;
    assert_eq!(icing[0].intensity, IcingIntensity::Severe);
    assert_eq!(icing[0].base_ft, 5000);
    assert_eq!(icing[0].thickness_ft, 2000);
}

#[test]
fn taf_icing_none_intensity() {
    // 600003 → intensity=0 (none)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 600003").unwrap();

    assert_eq!(taf.forecasts[0].icing[0].intensity, IcingIntensity::None);
}

#[test]
fn taf_no_icing_by_default() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();

    assert!(taf.forecasts[0].icing.is_empty());
}

#[test]
fn taf_multiple_icing_layers() {
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 610304 620502").unwrap();

    let icing = &taf.forecasts[0].icing;
    assert_eq!(icing.len(), 2);
    assert_eq!(icing[0].intensity, IcingIntensity::Light);
    assert_eq!(icing[1].intensity, IcingIntensity::ModerateMixedOrRime);
}

// ---------------------------------------------------------------------------
// Turbulence
// ---------------------------------------------------------------------------

#[test]
fn taf_turbulence_light_parsed() {
    // 510304 → 5 + intensity=1(light) + base=030 (3000 ft) + thickness=4 (4000 ft)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 510304").unwrap();

    let turb = &taf.forecasts[0].turbulence;
    assert_eq!(turb.len(), 1);
    assert_eq!(turb[0].intensity, TurbulenceIntensity::Light);
    assert_eq!(turb[0].base_ft, 3000);
    assert_eq!(turb[0].thickness_ft, 4000);
}

#[test]
fn taf_turbulence_moderate_in_cloud() {
    // 520803 → intensity=2 (moderate in-cloud), base=080 (8000 ft), thickness=3 (3000 ft)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 520803").unwrap();

    let turb = &taf.forecasts[0].turbulence;
    assert_eq!(turb[0].intensity, TurbulenceIntensity::ModerateInCloud);
    assert_eq!(turb[0].base_ft, 8000);
    assert_eq!(turb[0].thickness_ft, 3000);
}

#[test]
fn taf_turbulence_extreme() {
    // 560202 → intensity=6(extreme), base=020(2000 ft), thickness=2(2000 ft)
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 560202").unwrap();

    assert_eq!(
        taf.forecasts[0].turbulence[0].intensity,
        TurbulenceIntensity::Extreme
    );
}

#[test]
fn taf_no_turbulence_by_default() {
    let taf = parse_taf("TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020").unwrap();

    assert!(taf.forecasts[0].turbulence.is_empty());
}

#[test]
fn taf_icing_and_turbulence_together() {
    let taf = parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 610304 520803").unwrap();

    assert_eq!(taf.forecasts[0].icing.len(), 1);
    assert_eq!(taf.forecasts[0].turbulence.len(), 1);
}

#[test]
fn taf_icing_in_tempo_block() {
    let taf =
        parse_taf("TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 TEMPO 1218/1222 4000 -RA 610304")
            .unwrap();

    // Base forecast: no icing
    assert!(taf.forecasts[0].icing.is_empty());
    // TEMPO block: has icing
    assert_eq!(taf.forecasts[1].icing.len(), 1);
    assert_eq!(taf.forecasts[1].icing[0].intensity, IcingIntensity::Light);
}

// ---------------------------------------------------------------------------
// Direct model construction
// ---------------------------------------------------------------------------

#[test]
fn icing_model_fields() {
    let icing = Icing {
        intensity: IcingIntensity::Severe,
        base_ft: 10000,
        thickness_ft: 5000,
    };
    assert_eq!(icing.base_ft, 10000);
    assert_eq!(icing.thickness_ft, 5000);
}

#[test]
fn turbulence_model_fields() {
    let turb = Turbulence {
        intensity: TurbulenceIntensity::SevereClearAir,
        base_ft: 15000,
        thickness_ft: 3000,
    };
    assert_eq!(turb.base_ft, 15000);
}
