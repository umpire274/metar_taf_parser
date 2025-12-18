use crate::cli::Commands;
use crate::fetch::{fetch_metar, fetch_taf};
use crate::input::{normalize_icao, prompt_icao};
use crate::output::{AllOutput, MetarOutput, TafOutput};

use metar_taf_core::airports::AirportDb;
use metar_taf_core::{parse_metar, parse_taf};

pub fn execute(command: Commands, icao: Option<String>) {
    match command {
        Commands::Get {
            metar,
            taf,
            all,
            json,
            raw,
        } => {
            // -------------------------
            // ICAO resolution
            // -------------------------
            let icao = match icao {
                Some(code) => normalize_icao(code),
                None => prompt_icao(),
            };

            let airport_db = AirportDb::load();

            let airport = match airport_db.lookup(&icao) {
                Some(airport) => airport,
                None => {
                    eprintln!("Unknown ICAO airport code: {}", icao);
                    std::process::exit(1);
                }
            };

            // -------------------------
            // RAW mode (no parsing)
            // -------------------------
            if raw {
                if metar || all {
                    let raw_metar = fetch_metar(&icao).unwrap_or_else(|e| {
                        eprintln!("Failed to fetch METAR for {}: {}", icao, e);
                        std::process::exit(1);
                    });
                    println!("{}", raw_metar);
                }

                if taf || all {
                    let raw_taf = fetch_taf(&icao).unwrap_or_else(|e| {
                        eprintln!("Failed to fetch TAF for {}: {}", icao, e);
                        std::process::exit(1);
                    });
                    println!("{}", raw_taf);
                }

                return;
            }

            // -------------------------
            // Fetch + parse
            // -------------------------
            let metar_parsed = if metar || all {
                let raw = fetch_metar(&icao).unwrap_or_else(|e| {
                    eprintln!("Failed to fetch METAR for {}: {}", icao, e);
                    std::process::exit(1);
                });

                Some(parse_metar(&raw).unwrap_or_else(|e| {
                    eprintln!("Failed to parse METAR for {}: {}", icao, e);
                    std::process::exit(2);
                }))
            } else {
                None
            };

            let taf_parsed = if taf || all {
                let raw = fetch_taf(&icao).unwrap_or_else(|e| {
                    eprintln!("Failed to fetch TAF for {}: {}", icao, e);
                    std::process::exit(1);
                });

                Some(parse_taf(&raw).unwrap_or_else(|e| {
                    eprintln!("Failed to parse TAF for {}: {}", icao, e);
                    std::process::exit(2);
                }))
            } else {
                None
            };

            // -------------------------
            // JSON output (arricchito)
            // -------------------------
            if json {
                if all {
                    let out = AllOutput {
                        airport,
                        metar: metar_parsed.as_ref(),
                        taf: taf_parsed.as_ref(),
                    };

                    println!(
                        "{}",
                        serde_json::to_string_pretty(&out).expect("JSON serialization failed")
                    );
                } else if let Some(metar) = metar_parsed.as_ref() {
                    let out = MetarOutput { airport, metar };

                    println!(
                        "{}",
                        serde_json::to_string_pretty(&out).expect("JSON serialization failed")
                    );
                } else if let Some(taf) = taf_parsed.as_ref() {
                    let out = TafOutput { airport, taf };

                    println!(
                        "{}",
                        serde_json::to_string_pretty(&out).expect("JSON serialization failed")
                    );
                }

                return;
            }

            // -------------------------
            // Default output (debug)
            // -------------------------
            if let Some(metar) = metar_parsed {
                println!("{:#?}", metar);
            }

            if let Some(taf) = taf_parsed {
                println!("{:#?}", taf);
            }
        }
    }
}
