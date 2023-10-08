
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(PartialOrd, PartialEq)]
pub struct Decimal{
    decimal: f64
    // implement your type here
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal>
    {
        match input.parse::<f64>() {
            Ok(decimal) => Some(Decimal{ decimal }),
            Err(_) => None
        }
        // todo!("Create a new decimal with a value of {input}")
    }
}

// Implement Eq and PartialEq for Decimal
impl Eq for Decimal {}

impl PartialEq<Decimal> for f64 {
    fn eq(&self, other: &Decimal) -> bool {
        let res = self - other.decimal;
        res == 0.0
        // *self == other.decimal
    }
}

// See https://exercism.org/tracks/rust/exercises/decimal/solutions/vuongkienthanh