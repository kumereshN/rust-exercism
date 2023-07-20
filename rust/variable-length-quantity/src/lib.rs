use to_binary::{BinaryString, BinaryError};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    // Original return value should be Vec<u8>. Change it later when you're done.
    let concat_binary_string = values
        .iter()
        .map(|&n| {
            BinaryString::from(n as u8).0
        })
        .collect::<Vec<String>>()
        .join("");


    concat_binary_string.as_bytes().to_vec()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {bytes:?} to a list of numbers")
}
