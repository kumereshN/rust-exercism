#[derive(PartialEq)]
pub enum Operations {
    Plus,
    Minus,
    Multiply,
    Divide,
    Invalid,
    Unknown
}

impl From<&str> for Operations {
    fn from(operator: &str) -> Self {
        match operator {
            "plus" => Operations::Plus,
            "minus" => Operations::Minus,
            "multiplied" => Operations::Multiply,
            "divided" => Operations::Divide,
            "cubed" => Operations::Unknown,
            "exponent" => Operations::Unknown,
            _ => Operations::Invalid
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let numbers = command
        .split(|s| s == ' ' || s == '?')
        .filter_map(|s| {
            match s.chars().any(|c| c.is_ascii_digit()) {
                true => s.parse::<i32>().ok(),
                false => None
            }
        })
        .collect::<Vec<i32>>();

    let operations = command
        .split(|s| s == ' ' || s == '?')
        .filter_map(|s| {
            match s.into() {
                Operations::Invalid => None,
                _ => Some(s.into())
            }
        })
        .collect::<Vec<Operations>>();

    if operations.iter().any(|o| *o == Operations::Unknown) ||
        numbers.len() <= operations.len(){
        return None
    }

    let mut ops = operations.iter();

    numbers
        .into_iter()
        .reduce(|acc, x| {
            match ops.next() {
                Some(Operations::Plus) => { acc + x },
                Some(Operations::Minus) => { acc - x},
                Some(Operations::Multiply) => {acc * x},
                Some(Operations::Divide) => {acc / x},
                _ => acc
            }
        })
}