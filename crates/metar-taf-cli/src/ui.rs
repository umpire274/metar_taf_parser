use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Print a string with a typewriter effect
pub fn typewriter_print(text: &str, delay_ms: u64) {
    let mut stdout = io::stdout();

    for ch in text.chars() {
        print!("{}", ch);
        stdout.flush().ok();
        thread::sleep(Duration::from_millis(delay_ms));
    }
}
