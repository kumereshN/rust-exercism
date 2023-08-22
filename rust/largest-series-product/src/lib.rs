#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let chs = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?;
    if span == 0 {
        return Ok(1);
    }
    chs.windows(span)
        .map(|v| v.iter().map(|&n| n as u64).product())
        .max()
        .ok_or(Error::SpanTooLong)
}