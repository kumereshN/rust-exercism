// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter
        .enumerate()
        .filter_map(|(i, e)| {
            if i % 2 == 0 {
                return Some(e);
            }
            None
        })
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    // std::iter::empty()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let origin: (i16, i16) = (0, 0);
        (self.0 - origin.0).abs() + (self.1 - origin.1).abs()
    }
}
