use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(&[' ', ',', '\n'])
        .filter_map(|word| {
            let filtered_word = word.chars().filter(|&c| c == '\'' || c.is_alphanumeric()).collect::<String>();
            match filtered_word.is_empty() {
                true => None,
                false => Some(filtered_word.trim_matches('\'').to_ascii_lowercase())
            }
        })
        .fold(HashMap::new(), |mut acc, word| {
            acc
                .entry(word.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        })
}
