use crate::metar::models::cloud::{CloudAmount, CloudLayer, CloudType};

pub fn parse_cloud(token: &str) -> Option<CloudLayer> {
    // Protezione minima
    if token.len() < 3 {
        return None;
    }

    // NSC / SKC
    match token {
        "NSC" => {
            return Some(CloudLayer {
                amount: CloudAmount::NSC,
                altitude_ft: None,
                cloud_type: None,
            });
        }
        "SKC" => {
            return Some(CloudLayer {
                amount: CloudAmount::SKC,
                altitude_ft: None,
                cloud_type: None,
            });
        }
        _ => {}
    }

    // VV003
    if token.starts_with("VV") && token.len() == 5 {
        let altitude: u16 = token[2..].parse().ok()?;
        return Some(CloudLayer {
            amount: CloudAmount::VV,
            altitude_ft: Some(altitude * 100),
            cloud_type: None,
        });
    }

    // FEW030, SCT050, BKN100, OVC///
    if token.len() < 6 {
        return None;
    }

    let (amount_str, rest) = token.split_at(3);

    let amount = match amount_str {
        "FEW" => CloudAmount::FEW,
        "SCT" => CloudAmount::SCT,
        "BKN" => CloudAmount::BKN,
        "OVC" => CloudAmount::OVC,
        _ => return None,
    };

    // OVC///
    if rest == "///" {
        return Some(CloudLayer {
            amount,
            altitude_ft: None,
            cloud_type: None,
        });
    }

    // Altitudine + tipo opzionale
    let altitude: u16 = rest.get(0..3)?.parse().ok()?;
    let cloud_type = match rest.get(3..) {
        Some("CB") => Some(CloudType::CB),
        Some("TCU") => Some(CloudType::TCU),
        _ => None,
    };

    Some(CloudLayer {
        amount,
        altitude_ft: Some(altitude * 100),
        cloud_type,
    })
}
