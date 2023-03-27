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

    for input in inputs{
        match input {
            CalculatorInput::Value(i) => res.push(*i),
            _ => {
                let y = res.pop();
                let x = res.pop();
                match (x, y) {
                    (Some(x), Some(y)) => match input {
                        CalculatorInput::Add => res.push(x + y),
                        CalculatorInput::Subtract => res.push(x - y),
                        CalculatorInput::Multiply => res.push(x * y),
                        CalculatorInput::Divide => res.push(x / y),
                        _ => (),
                    },
                    _ => return None
                }
            }
        }
    }

    match res[..] {
         [result] => Some(result),
        _ => None,
    }

}
