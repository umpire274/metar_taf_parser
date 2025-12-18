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
        Commands::Get { metar, taf, all } => {
            if metar || all {
                match fetch_metar(&icao) {
                    Ok(raw) => {
                        println!("RAW METAR {}:\n{}\n", icao, raw);

                        match parse_metar(&raw) {
                            Ok(parsed) => {
                                println!("PARSED METAR {}:\n{:#?}\n", icao, parsed);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse METAR for {}: {}", icao, e);
                                std::process::exit(2);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch METAR for {}: {}", icao, e);
                        std::process::exit(1);
                    }
                }
            }

            if taf || all {
                match fetch_taf(&icao) {
                    Ok(raw) => {
                        println!("RAW TAF {}:\n{}\n", icao, raw);

                        match parse_taf(&raw) {
                            Ok(parsed) => {
                                println!("PARSED TAF {}:\n{:#?}\n", icao, parsed);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse TAF for {}: {}", icao, e);
                                std::process::exit(2);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch TAF for {}: {}", icao, e);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
