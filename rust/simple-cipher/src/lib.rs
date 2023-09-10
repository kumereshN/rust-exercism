use std::char;
use rand::prelude::*;

fn is_valid_key(key: &str) -> bool {
    key.chars().all(|c| c.is_ascii_lowercase())
}
pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) || key.is_empty() {
        return None
    }

    Some(key
        .chars().cycle()
        .zip(s
            .chars())
        .map(|(c1, c2)| {
            let start = b'a';
            // First convert to char's index on the 'a'..='z' range
            let c1 = c1 as u8 - start;
            let c2 = c2 as u8 - start;
            let encoded_unicode_char_value = (c1 + c2).rem_euclid(26) + start;
            char::from_u32(encoded_unicode_char_value as u32).unwrap()
        })
        .collect::<String>()
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) || key.is_empty() {
        return None
    }

    Some(key
        .chars().cycle()
        .zip(s
            .chars())
        .map(|(c1, c2)| {
            let start = b'a';
            // First convert to char's index on the 'a'..='z' range
            let c1 = (c1 as u8 - start) as i8;
            let c2 = (c2 as u8 - start) as i8;
            let decoded_unicode_char_value = (c2-c1).rem_euclid(26) as u8 + start;
            char::from_u32(decoded_unicode_char_value as u32).unwrap()
        })
        .collect::<String>()
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key = (0..=100)
        .map(|_| {
            rng.gen_range(b'a'..=b'z') as char
        })
        .collect::<String>();

    let encoded_string = encode(key.as_str(), s).unwrap();
    (key, encoded_string)
}
