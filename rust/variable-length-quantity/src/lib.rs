

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    // Original return value should be Vec<u8>. Change it later when you're done.
    let mut res = vec![];
    values.iter().rev().for_each(|&num| {
        res.push(num as u8 & 0x7F);
        let mut n = num >> 7;
        while n != 0 {
            res.push((n as u8 & 0x7F) | 0x80);
            n >>= 7;
        }
    });
    res.reverse();
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = Vec::new();
    let mut iter = bytes.iter().peekable();
    let mut number = 0u32;

    while let Some(byte) = iter.next() {
        number = if number.leading_zeros() >= 7 {
            (number << 7) | (byte & 0x7F) as u32
        } else {
            return Err(Error::Overflow)
        };
        if byte & 0x80 == 0 {
            res.push(number);
            number = 0;
            continue;
        } else if iter.peek().is_none() {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(res)
}
