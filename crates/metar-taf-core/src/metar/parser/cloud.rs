use crate::metar::models::cloud::{CloudAmount, CloudLayer, CloudType};

pub fn parse_cloud(token: &str) -> Option<CloudLayer> {
    if token.len() < 3 {
        return None;
    }

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

    // Vertical visibility: VV003 or VV///
    if token.starts_with("VV") && token.len() == 5 {
        let altitude_ft = if &token[2..] == "///" {
            None
        } else {
            let altitude_hundreds: u16 = token[2..].parse().ok()?;
            Some(altitude_hundreds * 100)
        };

        return Some(CloudLayer {
            amount: CloudAmount::VV,
            altitude_ft,
            cloud_type: None,
        });
    }

    // Cloud layers: FEW030, SCT050CB, BKN100TCU, OVC///
    if token.len() < 6 {
        return None;
    }

    let (amount_str, rest) = token.split_at(3);
    let amount = parse_cloud_amount(amount_str)?;

    if rest == "///" {
        return Some(CloudLayer {
            amount,
            altitude_ft: None,
            cloud_type: None,
        });
    }

    if rest.len() != 3 && rest.len() != 5 {
        return None;
    }

    let altitude_hundreds: u16 = rest.get(0..3)?.parse().ok()?;
    let cloud_type = match rest.get(3..) {
        Some("") | None => None,
        Some("CB") => Some(CloudType::CB),
        Some("TCU") => Some(CloudType::TCU),
        _ => return None,
    };

    Some(CloudLayer {
        amount,
        altitude_ft: Some(altitude_hundreds * 100),
        cloud_type,
    })
}

fn parse_cloud_amount(value: &str) -> Option<CloudAmount> {
    match value {
        "FEW" => Some(CloudAmount::FEW),
        "SCT" => Some(CloudAmount::SCT),
        "BKN" => Some(CloudAmount::BKN),
        "OVC" => Some(CloudAmount::OVC),
        _ => None,
    }
}
