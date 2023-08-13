use std::iter::Iterator;
use std::ops::{Div};

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}


fn is_coprime(mut a: i32, mut b: i32) -> bool {
    let mut r = -1;

    while r != 0 {
        r = a % b;
        if r == 0 {
            break
        }
        (a, b) = (b, r);
    }

    b == 1
}

fn calculate_mmi(a: f32, m: f32) -> u32 {
    (m).div(a).round() as u32
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let alphabets = ('a'..='z').collect::<String>();
    let alphabets_len = alphabets.len() as i32;
    match is_coprime(a, alphabets_len) {
        true => {
            Ok(plaintext
                .chars()
                .filter_map(|mut c| {
                    c = c.to_ascii_lowercase();
                    match (c.is_alphabetic(), c.is_numeric()) {
                        (true, false) => {
                            let index_char = alphabets.chars().position(|x| x == c).unwrap() as i32;
                            let encoded_no = (((a * index_char) + b) %  alphabets_len) as usize;
                            alphabets.chars().nth(encoded_no)
                        }
                        (false, true) => {
                            Some(c)
                        }
                        (_, _) => None
                    }
                })
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|s| {
                    format!("{} ", s.iter().collect::<String>())
                })
                .collect::<String>()
                .trim_end()
                .to_string()
            )
        },
        false => {
            Err(AffineCipherError::NotCoprime(a))
        }
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let alphabets = ('a'..='z').collect::<String>();
    let alphabets_len = alphabets.len() as f32;
    let mmi_value = calculate_mmi(a as f32, alphabets_len) as i32;

    Ok(ciphertext
        .chars()
        .filter_map(|mut c| {
            c = c.to_ascii_lowercase();
            match(c.is_alphabetic(), c.is_numeric()) {
                (true, false) => {
                    let y = alphabets.chars().position(|x| x == c).unwrap() as i32;
                    let mut decoded_value = mmi_value * (y - b);
                    match decoded_value < 0 {
                        true => {
                            decoded_value = (decoded_value + alphabets_len as i32) % alphabets_len as i32;
                        },
                        false => {
                            decoded_value %= alphabets_len as i32;
                        }
                    }
                    alphabets.chars().nth(decoded_value as usize)
                },
                (false, true) => {
                    Some(c)
                },
                (_, _) => {
                    None
                }
            }
        })
        .collect::<String>()
    )
}
