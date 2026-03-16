use std::collections::HashMap;

use super::model::Airport;

pub struct AirportDb {
    by_icao: HashMap<String, Airport>,
}

impl AirportDb {
    pub fn load() -> Self {
        let data = include_str!("../../resources/airports.dat");

        let mut by_icao = HashMap::new();

        for line in data.lines() {
            if let Some(airport) = parse_airport_line(line) {
                by_icao.insert(airport.icao.clone(), airport);
            }
        }

        Self { by_icao }
    }

    pub fn lookup(&self, icao: &str) -> Option<&Airport> {
        self.by_icao.get(&icao.to_uppercase())
    }
}

/// Parse a single OurAirports CSV line
fn parse_airport_line(line: &str) -> Option<Airport> {
    let fields = split_csv_line(line);

    // Sanity check (OurAirports has many columns)
    if fields.len() < 9 {
        return None;
    }

    let icao = fields.get(5)?.trim();
    if icao.len() != 4 {
        return None;
    }

    let name = fields.get(1)?.trim().to_string();
    let country = fields.get(3)?.trim().to_string();

    let latitude: f64 = fields.get(6)?.trim().parse().ok()?;
    let longitude: f64 = fields.get(7)?.trim().parse().ok()?;

    let elevation_ft = fields.get(8).and_then(|v| v.trim().parse::<i32>().ok());

    Some(Airport {
        icao: icao.to_uppercase(),
        name,
        country,
        latitude,
        longitude,
        elevation_ft,
    })
}

/// Minimal CSV splitter supporting quoted fields
fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim_matches('"').to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        fields.push(current.trim_matches('"').to_string());
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_known_airport() {
        let db = AirportDb::load();

        let airport = db.lookup("LIRF").expect("LIRF should exist");

        assert_eq!(airport.icao, "LIRF");
        assert_eq!(airport.country, "Italy");
    }
}
