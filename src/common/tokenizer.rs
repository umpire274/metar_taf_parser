//! Module `tokenizer`.
//!
//! Contains types and parsing logic implemented for this crate.
#[derive(Debug)]
/// Defines the Tokenizer domain model used by the parser.
pub struct Tokenizer {
    tokens: Vec<String>,
    index: usize,
}

impl Tokenizer {
    /// Creates a new `new` value with normalized defaults.
    pub fn new(input: &str) -> Self {
        let tokens = input
            .split_whitespace()
            .map(|s| s.trim_end_matches('='))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Self { tokens, index: 0 }
    }

    /// Helper function used by `peek` parsing logic.
    pub fn peek(&self) -> Option<&str> {
        self.tokens.get(self.index).map(|s| s.as_str())
    }
}

impl Iterator for Tokenizer {
    /// Type alias used by Item-related logic.
    type Item = String;

    /// Helper function used by `next` parsing logic.
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.index)?.clone();
        self.index += 1;
        Some(token)
    }
}
