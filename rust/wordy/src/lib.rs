#[derive(PartialEq)]
pub enum Operations {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Invalid
}

impl From<&str> for Operations {
    fn from(operator: &str) -> Self {
        match operator {
            "plus" => Operations::Plus,
            "minus" => Operations::Minus,
            "multiplied" => Operations::Multiply,
            "divided" => Operations::Divide,
            "raised" => Operations::Exponent,
            _ => Operations::Invalid
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let command = command
        .split(|c: char| c.is_ascii_whitespace() || c == '?')
        .skip(2)
        .take_while(|&c| c != "power")
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
            .chars()
            .filter(|s| s.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>()
            .is_err()
    {
        return None
    }

    let numbers = command
        .split_ascii_whitespace()
        .filter_map(|s| {
            s.trim_end_matches(char::is_alphabetic).parse::<i32>().ok()
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

    if numbers.len().saturating_sub(1) != operations.len() {
        return None
    }

    let mut ops = operations.iter();

    numbers
        .into_iter()
        .reduce(|acc, x| {
            match ops.next() {
                Some(Operations::Plus) => { acc + x },
                Some(Operations::Minus) => { acc - x },
                Some(Operations::Multiply) => { acc * x },
                Some(Operations::Divide) => { acc / x },
                Some(Operations::Exponent) => { acc.pow(x as u32) }
                _ => acc
            }
        })
}