use std::fmt::{Display, Formatter, Result};

// Look up on: https://medium.com/@tomas.langkaas/eight-algorithms-for-roman-numerals-b06c83db12dd

#[derive(Debug)]
pub struct Roman(String);

const ROMAN_NUMERALS: [(u32, &str); 13] = [
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
];


impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.0.as_str())
        // unimplemented!("Return a roman-numeral string representation of the Roman object");
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {

        let mut res = String::new();
        for (number, roman_number) in ROMAN_NUMERALS {
            while num >= number {
                num -= number;
                res.push_str(roman_number);
            }
        }

        Self(res)
        // unimplemented!("Construct a Roman object from the '{num}' number");
    }
}
