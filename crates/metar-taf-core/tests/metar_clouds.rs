use metar_taf_core::metar::models::cloud::{CloudAmount, CloudType};
use metar_taf_core::parse_metar;

#[test]
fn metar_multiple_cloud_layers() {
    let input = "LIRF 121250Z 18010KT 9999 FEW030 SCT050CB BKN100 OVC/// 18/12 Q1015";

    let metar = parse_metar(input).expect("METAR should parse");

    assert_eq!(metar.clouds.len(), 4);

    assert!(matches!(metar.clouds[0].amount, CloudAmount::FEW));
    assert_eq!(metar.clouds[0].altitude_ft, Some(3000));

    assert!(matches!(metar.clouds[1].amount, CloudAmount::SCT));
    assert_eq!(metar.clouds[1].cloud_type, Some(CloudType::CB));

    assert!(matches!(metar.clouds[3].amount, CloudAmount::OVC));
    assert_eq!(metar.clouds[3].altitude_ft, None);
}
