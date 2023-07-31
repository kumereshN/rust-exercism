pub enum Operations {
    Plus,
    Minus,
    Multiply,
    Divide,
    Unknown
}

impl From<&str> for Operations {
    fn from(operator: &str) -> Self {
        match operator {
            "plus" => Operations::Plus,
            "minus" => Operations::Minus,
            "multiply" => Operations::Divide,
            "divide" => Operations::Divide,
            _ => Operations::Unknown
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let numbers = command
        .split(|s| s == ' ' || s == '?')
        .filter_map(|s| {
            match s.chars().all(|c| c.is_ascii_digit()) {
                true => s.parse::<i32>().ok(),
                false => None
            }
        })
        .collect::<Vec<i32>>();

    let operations = command
        .split(|s| s == ' ' || s == '?')
        .map(|s| {
            s.into()
        })
        .collect::<Vec<Operations>>();

}
