use to_binary::{BinaryString, BinaryError};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<String> {
    // Original return value should be Vec<u8>. Change it later when you're done.
    let concat_binary_string = values
        .iter()
        .map(|&n| {
            BinaryString::from(n as u8).0
        })
        .collect::<Vec<String>>()
        .join("");

    let len_concat_binary_string = concat_binary_string.len();

    if len_concat_binary_string % 7 == 0 {
        let windows = 7;
        let vec_string = concat_binary_string
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(7);
    }

    vec![concat_binary_string]
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
}
