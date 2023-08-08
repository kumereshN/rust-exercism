/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let alphabets = ('a'..='z').collect::<String>();
    let encoded_string = plaintext
        .chars()
        .filter_map(|mut c| {
            c = c.to_ascii_lowercase();
            match (c.is_alphabetic(), c.is_numeric()) {
                (true, false) => {
                    let index_char = alphabets.chars().position(|x| x == c).unwrap();
                    let e_no = (((a * index_char as i32) + b) %  26) as usize;
                    alphabets.chars().nth(e_no)
                }
                (false, true) => {
                    Some(c)
                }
                (_, _) => None
            }
        }).collect::<String>();

    Ok(encoded_string)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {ciphertext} with the key ({a}, {b})");
}
