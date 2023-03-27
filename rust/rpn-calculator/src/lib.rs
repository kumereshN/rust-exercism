#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut res = vec![];

    inputs.iter().map(|s| match s {
        CalculatorInput::Value(n) => res.push(n),
        CalculatorInput::Add => n1 + n2,
        "-" => CalculatorInput::Subtract,
        "*" => CalculatorInput::Multiply,
        "/" => CalculatorInput::Divide,
    });
}
