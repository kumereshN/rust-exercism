use std::collections::HashMap;

fn map_chars(text: &str, a_map: HashMap<char, char>) -> Vec<char> {
    text
        .to_ascii_lowercase()
        .chars()
        .filter_map(|c| {
            match (c.is_alphabetic(), c.is_numeric()) {
                (true, false) => { Some(*a_map.get(&c).unwrap()) },
                (false, true) => { Some(c) },
                _ => None
            }
        })
        .collect::<Vec<char>>()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let alphabets = ('a'..='z').collect::<String>();
    let alphabets_reverse = alphabets.chars().rev().collect::<String>();

    let alphabet_map: HashMap<_, _> = alphabets
        .chars()
        .zip(alphabets_reverse.chars())
        .collect();

    map_chars(plain, alphabet_map)
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let alphabets = ('a'..='z').collect::<String>();
    let alphabets_reverse = alphabets.chars().rev().collect::<String>();

    let alphabet_map: HashMap<_, _> = alphabets_reverse
        .chars()
        .zip(alphabets.chars())
        .collect();

    map_chars(cipher, alphabet_map)
        .iter()
        .collect::<String>()
}
