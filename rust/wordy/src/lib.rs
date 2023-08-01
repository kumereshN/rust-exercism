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
            _ => Operations::Invalid
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let command = command
        .split(|c: char| c.is_ascii_whitespace() || c == '?')
        .skip(2)
        .collect::<Vec<&str>>()
        .join(" ");

    if command.is_empty() ||
        command
            .split_ascii_whitespace()
            .take(1)
            .next()
            .unwrap()
            .parse::<i32>()
            .is_err() ||
        command
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .last()
            .unwrap()
            .parse::<i32>()
            .is_err()
        {
        return None
    }

    let numbers = command
        .split_ascii_whitespace()
        .filter_map(|s| {
            match s.chars().any(|c| c.is_ascii_digit()) {
                true => s.parse::<i32>().ok(),
                false => None
            }
        })
        .collect::<Vec<i32>>();

    let operations = command
        .split_ascii_whitespace()
        .filter_map(|s| {
            match s.into() {
                Operations::Invalid => None,
                _ => Some(s.into())
            }
        })
        .collect::<Vec<Operations>>();

    if numbers.len().saturating_sub(1) != operations.len(){
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