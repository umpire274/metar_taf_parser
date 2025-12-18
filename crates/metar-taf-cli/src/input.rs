use crate::ui::typewriter_print;
use atty::Stream;
use std::io::{self, Write};

/// Prompt interattivo per codice ICAO
pub fn prompt_icao() -> String {
    loop {
        println!();
        if atty::is(Stream::Stdout) {
            typewriter_print("Enter ICAO airport code: ", 80);
        } else {
            print!("Enter ICAO airport code: ");
        }
        io::stdout().flush().ok();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let code = input.trim().to_uppercase();
            if is_valid_icao(&code) {
                println!();

                return code;
            }
        }

        eprintln!("Invalid ICAO code. Please enter a 4-letter ICAO code.");
    }
}

/// Normalizza e valida ICAO da CLI
pub fn normalize_icao(input: String) -> String {
    let code = input.trim().to_uppercase();
    if !is_valid_icao(&code) {
        eprintln!("Invalid ICAO code: {}", code);
        std::process::exit(1);
    }
    code
}

/// Validazione ICAO base (4 lettere ASCII)
pub fn is_valid_icao(code: &str) -> bool {
    code.len() == 4 && code.chars().all(|c| c.is_ascii_alphabetic())
}
