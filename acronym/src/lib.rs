pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|s| {
            if s[0].is_lowercase() && s[1].is_uppercase() {
                [s[0], ' ', s[1]].to_vec()
            } else {
                s.to_vec()
            }
        })
        .collect::<String>()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
        .map(|s| s.get(0..1).unwrap().to_ascii_uppercase())
        .collect()
}
