use std::collections::HashMap;

fn check_and_remove_quotations_in_string(word: String) -> String {
    let vec_char = word.chars().collect::<Vec<char>>();
    let first_char = *vec_char.first().unwrap();
    let last_char = *vec_char.last().unwrap();

    if first_char == '\'' && last_char == '\'' {
        vec_char.iter().filter(|&&c| c.is_alphanumeric()).collect::<String>()
    } else {
        word
    }
}

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(&[' ', ',', '\n'])
        .filter_map(|word| {
            let filtered_word = word.chars().filter(|&c| c == '\'' || c.is_alphanumeric()).collect::<String>();
            match filtered_word.is_empty() {
                true => None,
                false => Some(check_and_remove_quotations_in_string(filtered_word).to_ascii_lowercase())
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
