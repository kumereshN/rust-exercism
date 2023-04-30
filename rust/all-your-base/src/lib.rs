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

fn convert_digits_to_base_10(vec_of_numbers: Vec<u32>, to_base: u32) -> Vec<u32>{
    vec_of_numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| {
            n * (to_base.pow(i as u32))
        })
        .sum::<u32>()
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()

}

fn convert_digits_to_target_base(vec_of_numbers: Vec<u32>, to_base: u32) -> Vec<u32> {
    let mut num = vec_of_numbers
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit);

    if num == 0 {
        return vec![0]
    }

    let mut res:Vec<u32> = vec![];

    while num != 0 {
        let reminder = num % to_base;
        res.push(reminder);
        num /= to_base;
    }
    res.reverse();
    res
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0..=1, _) => {
            Err(Error::InvalidInputBase)
        }
        (_, 0..=1) => {
            Err(Error::InvalidOutputBase)
        },
        (_, 10) => {
            if number.iter().any(|&n| n >= from_base) {
                let invalid_num = number
                    .iter()
                    .filter(|&&n| n > 1)
                    .copied()
                    .collect::<Vec<u32>>();

                Err(Error::InvalidDigit(*invalid_num.first().unwrap()))
            } else {
                let base_10_digit = convert_digits_to_base_10(number.to_vec(), from_base);
                Ok(base_10_digit)
            }
        }
        (10, _) => {
            let res = convert_digits_to_target_base(number.to_vec(), to_base);
            Ok(res)
        },
        (_, _) => {
            let base_10_vec = convert_digits_to_base_10(number.to_vec(), from_base);
            let res = convert_digits_to_target_base(base_10_vec, to_base);
            Ok(res)
        }
    }
}
