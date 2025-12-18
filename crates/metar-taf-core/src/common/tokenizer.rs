#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<String>,
    index: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let tokens = input.split_whitespace().map(|s| s.to_string()).collect();

        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&str> {
        self.tokens.get(self.index).map(|s| s.as_str())
    }
}

impl Iterator for Tokenizer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.index)?.clone();
        self.index += 1;
        Some(token)
    }
}
