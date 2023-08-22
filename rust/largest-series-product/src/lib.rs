#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let len_of_digits = string_digits.len();
    let has_alphabets = string_digits
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    if len_of_digits < span {
        Err(Error::SpanTooLong)
    } else if !has_alphabets.is_empty() {
        Err(Error::InvalidDigit(has_alphabets.chars().next().unwrap()))
    }
    else {
        let product_of_chars = string_digits
            .chars()
            .collect::<Vec<char>>()
            .windows(span)
            .map(|c| {
                c.iter().map(|&n| n.to_digit(10).unwrap() as u64).product::<u64>()
            })
            .collect::<Vec<u64>>();
        Ok(product_of_chars.into_iter().max().unwrap())
    }
}