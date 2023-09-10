use std::char;
pub fn encode(key: &str, s: &str) -> Option<String> {
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
    // unimplemented!("Use {key} to encode {s} using shift cipher")
}

pub fn decode(key: &str, s: &str) -> Option<String> {
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
    unimplemented!(
        "Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)"
    )
}
