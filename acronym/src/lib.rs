pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|s| !s.is_empty())
        .flat_map(|s| {
            if s.chars().all(char::is_uppercase) || s.chars().all(char::is_lowercase) {
                s.chars().take(1).map(|c| c.to_ascii_uppercase()).collect::<Vec<_>>()
            } else {
                s.chars().filter(|c| c.is_uppercase()).collect::<Vec<_>>()
            }
        })
        .collect()
}
