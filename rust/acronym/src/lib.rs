pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-' || c == '_')
        .flat_map(|word: &str| {
            word
                .chars()
                .take(1)
                .chain(
                    word
                        .chars()
                        .skip_while(|c: &char| c.is_ascii_uppercase()) //Skip consecutive upper case letters
                        .filter(|c: &char| c.is_ascii_uppercase())
                )
        })
        .map(|c: char| c.to_ascii_uppercase())
        .collect::<String>()
}
