use crate::metar::models::Metar;
use crate::metar::models::visibility::{Visibility, VisibilityDirection};

pub fn parse_visibility(token: &str, metar: &Metar) -> Option<Visibility> {
    // CAVOK
    if token == "CAVOK" {
        return Some(Visibility::CAVOK);
    }

    // Prevalente (5000)
    if token.len() == 4 && token.chars().all(|c| c.is_ascii_digit()) {
        let prevailing: u16 = token.parse().ok()?;
        return Some(Visibility::Single { prevailing });
    }

    // Minima direzionale (2000SW)
    if token.len() >= 5 {
        let (dist_part, dir_part) = token.split_at(token.len() - 2);

        let minimum: u16 = dist_part.parse().ok()?;
        let direction = parse_visibility_direction(dir_part)?;

        // Deve esistere una visibilità prevalente precedente
        if let Some(Visibility::Single { prevailing }) = metar.visibility {
            return Some(Visibility::WithMinimum {
                prevailing,
                minimum,
                direction,
            });
        }
    }

    None
}

fn parse_visibility_direction(s: &str) -> Option<VisibilityDirection> {
    match s {
        "N" => Some(VisibilityDirection::N),
        "NE" => Some(VisibilityDirection::NE),
        "E" => Some(VisibilityDirection::E),
        "SE" => Some(VisibilityDirection::SE),
        "S" => Some(VisibilityDirection::S),
        "SW" => Some(VisibilityDirection::SW),
        "W" => Some(VisibilityDirection::W),
        "NW" => Some(VisibilityDirection::NW),
        _ => None,
    }
}
