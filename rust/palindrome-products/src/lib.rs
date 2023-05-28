/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use iss
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.

use itertools::{Itertools, MinMaxResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let reverse_value = value.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        match value == reverse_value {
            true => Some(Palindrome(value)),
            false => None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let minmax = (min..=max)
        .combinations_with_replacement(2)
        .filter_map(|x| {
            let product = x[0] * x[1];
            Palindrome::new(product)
        })
        .minmax();

    match minmax {
        MinMaxResult::MinMax(a,b) => Some((a,b)),
        MinMaxResult::OneElement(a) => Some((a,a)),
        MinMaxResult::NoElements => None
    }

}
