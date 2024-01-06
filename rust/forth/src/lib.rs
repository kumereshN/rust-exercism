use crate::Error::StackUnderflow;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub struct Forth {
    nums: Vec<Value>,
    operations: Vec<Operations>,
    res: Vec<Value>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            nums: vec![],
            operations: vec![],
            res: vec![]
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.res
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut vec_of_nums = vec![];
        let mut vec_of_operations = vec![];

        for s in input.split_ascii_whitespace() {
            match s.parse::<Value>() {
                Ok(n) => vec_of_nums.push(n),
                Err(e) => match s {
                    "+" => vec_of_operations.push(Operations::Add),
                    "-" => vec_of_operations.push(Operations::Subtract),
                    "*" => vec_of_operations.push(Operations::Multiply),
                    "/" => vec_of_operations.push(Operations::Divide),
                    _ => panic!("Error occurred: {}", e)
                }
            }
        }

        if vec_of_operations.is_empty() {
            self.res = vec_of_nums;
            return Ok(())
        }

        if vec_of_nums.len().div_euclid(vec_of_operations.len()) != 2 {
            return Err(StackUnderflow)
        }

        let res = vec_of_nums
            .chunks(2)
            .zip(vec_of_operations)
            .map(|(v, o)| {
                let v1 = v[0];
                let v2 = v[1];
                match o {
                    Operations::Add => v.iter().sum::<Value>(),
                    Operations::Subtract => {
                        v1-v2
                    }
                    _ => panic!("Something went wrong")
                }
            })
            .collect::<Vec<Value>>();
        self.res = res;
        Ok(())
    }
}
