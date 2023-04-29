#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.

fn convert_digits_to_base_10(slice_of_numbers: &[u32], to_base: u32) -> u32{
    slice_of_numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| {
            n * (to_base.pow(i as u32))
        })
        .sum::<u32>()
}

fn convert_digits_to_target_base(slice_of_numbers: &[u32], to_base: u32) -> u32 {
    10
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    match (from_base, to_base) {
        (_, 10) => {
            let base_10_digit = convert_digits_to_base_10(number, from_base)
                .to_string()
                .chars()
                .filter_map(|c| {c.to_digit(10)})
                .collect::<Vec<u32>>();
            Ok(base_10_digit)
        },
        (10, _) => {
            let res = convert_digits_to_target_base(number, to_base);
            Ok(vec![res])
        },
        (_, _) => {
            let base_10_vec = convert_digits_to_base_10(number, from_base);
            let res = convert_digits_to_base_10(&[base_10_vec], to_base);
            Ok(vec![res])
        }
    }
}
