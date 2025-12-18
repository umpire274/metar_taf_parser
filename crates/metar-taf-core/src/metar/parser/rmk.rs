pub fn parse_rmk(tokens: &[String]) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }

    Some(tokens.join(" "))
}
