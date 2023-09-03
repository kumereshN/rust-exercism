use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use lazy_static::lazy_static;

// Look up on: https://medium.com/@tomas.langkaas/eight-algorithms-for-roman-numerals-b06c83db12dd

#[derive(Debug)]
pub struct Roman<'a>(Vec<&'a str>);

lazy_static! {
    static ref ROMAN_NUMERAL_HASHMAP: HashMap<u32, &'static str> = [
        (1_000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ].iter().copied().collect();
}

impl<'a> Display for Roman<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.0)
        // unimplemented!("Return a roman-numeral string representation of the Roman object");
    }
}

impl<'a> From<u32> for Roman<'a> {
    fn from(num: u32) -> Self {
        let res = *ROMAN_NUMERAL_HASHMAP.get(&num).unwrap();
        Self(res)
        // unimplemented!("Construct a Roman object from the '{num}' number");
    }
}
