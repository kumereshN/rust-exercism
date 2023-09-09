use std::char;
pub fn encode(key: &str, s: &str) -> Option<String> {
    Some(key
        .chars()
        .zip(s
            .chars())
        .map(|(c1, c2)| {
            let start = if c1.is_ascii_uppercase() { b'A' } else { b'a' };
            // First convert to char's index on the 'a'..='z' range
            let c1 = c1 as u8 - start;
            let c2 = c2 as u8 - start;
            let encoded_unicode_char_value = (c1 + c2).rem_euclid(26) + start;
            char::from_u32(encoded_unicode_char_value as u32).unwrap()
        })
        .collect::<String>()
    )
    // unimplemented!("Use {key} to encode {s} using shift cipher")
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    Some(key
        .chars()
        .zip(s
            .chars())
        .map(|(c1, c2)| {
            let start = if c1.is_ascii_uppercase() { b'A' } else { b'a' };
            // First convert to char's index on the 'a'..='z' range
            let c1 = c1 as u8 - start;
            let c2 = c2 as u8 - start;
            let decoded_unicode_char_value = (c1.abs_diff(c2)) + start;
            char::from_u32(decoded_unicode_char_value as u32).unwrap()
        })
        .collect::<String>()
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)"
    )
}
