use crate::cli::Commands;
use crate::fetch::{fetch_metar, fetch_taf};
use crate::input::{normalize_icao, prompt_icao};

use metar_taf_core::{parse_metar, parse_taf};

pub fn execute(command: Commands, icao: Option<String>) {
    let icao = match icao {
        Some(code) => normalize_icao(code),
        None => prompt_icao(),
    };

    match command {
        Commands::Get {
            metar,
            taf,
            all,
            json,
            raw,
        } => {
            if metar || all {
                let raw_string = match fetch_metar(&icao) {
                    Ok(raw) => raw,
                    Err(e) => {
                        eprintln!("Failed to fetch METAR for {}: {}", icao, e);
                        std::process::exit(1);
                    }
                };

                // ---- RAW ONLY ----
                if raw {
                    println!("RAW METAR {}:\n{}\n", icao, raw_string);
                    return;
                }

                let parsed = match parse_metar(&raw_string) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Failed to parse METAR for {}: {}", icao, e);
                        std::process::exit(2);
                    }
                };

                // ---- JSON ----
                if json {
                    let out = serde_json::to_string_pretty(&parsed).unwrap_or_else(|e| {
                        eprintln!("Failed to serialize METAR as JSON: {}", e);
                        std::process::exit(3);
                    });
                    println!("{}", out);
                } else {
                    // ---- DEFAULT (debug) ----
                    println!("{:#?}", parsed);
                }
            }

            if taf || all {
                let raw_string = match fetch_taf(&icao) {
                    Ok(raw) => raw,
                    Err(e) => {
                        eprintln!("Failed to fetch TAF for {}: {}", icao, e);
                        std::process::exit(1);
                    }
                };

                // ---- RAW ONLY ----
                if raw {
                    println!("RAW TAF {}:\n{}\n", icao, raw_string);
                    return;
                }

                let parsed = match parse_taf(&raw_string) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Failed to parse TAF for {}: {}", icao, e);
                        std::process::exit(2);
                    }
                };

                // ---- JSON ----
                if json {
                    let out = serde_json::to_string_pretty(&parsed).unwrap_or_else(|e| {
                        eprintln!("Failed to serialize TAF as JSON: {}", e);
                        std::process::exit(3);
                    });
                    println!("{}", out);
                } else {
                    // ---- DEFAULT (debug) ----
                    println!("{:#?}", parsed);
                }
            }
        }
    }
}
