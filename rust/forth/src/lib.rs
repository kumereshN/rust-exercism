use std::iter::Zip;
use std::slice::Chunks;
use std::vec::IntoIter;
use crate::Error::{DivisionByZero, StackUnderflow};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub struct Forth {
    stack: Vec<Value>,
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
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![]
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn stack_manipulation()

    pub fn calculate_integer_arithmetic(num_ops_zip: Zip<Chunks<Value>, IntoIter<Operations>>) -> std::result::Result<Vec<Value>, Error> {
        let mut res = 0;

        for (v, ops) in num_ops_zip {
            match (ops, v.len()) {
                (Operations::Add, _) => res += v.iter().sum::<Value>(),
                (Operations::Subtract, 2) => res += v[0] - v[1],
                (Operations::Subtract, _) => res -= v[0],
                (Operations::Multiply, 2) => res += v[0] * v[1],
                (Operations::Multiply, _) => res *= v[0],
                (Operations::Divide, 2) => match v[0].checked_div_euclid(v[1]) {
                    Some(v) => res += v,
                    None => return Err(DivisionByZero)
                }
                (Operations::Divide, _) => res /= v[0],
                _ => panic!("Error occurred in operations")
            }
        }
        Ok(vec![res])
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut vec_of_nums = vec![];
        let mut vec_of_operations = vec![];

        for s in input.split_ascii_whitespace() {
            match s.parse::<Value>() {
                Ok(n) => vec_of_nums.push(n),
                Err(e) => match s.to_lowercase().as_str() {
                    "+" => vec_of_operations.push(Operations::Add),
                    "-" => vec_of_operations.push(Operations::Subtract),
                    "*" => vec_of_operations.push(Operations::Multiply),
                    "/" => vec_of_operations.push(Operations::Divide),
                    "dup" => vec_of_operations.push(Operations::Duplicate),
                    _ => panic!("Error occurred: {}", e)
                }
            }
        }

        if vec_of_operations.is_empty() {
            self.stack = vec_of_nums;
            return Ok(())
        }

        if vec_of_nums.len().saturating_sub(1) != (vec_of_operations.len()) {
            return Err(StackUnderflow)
        }

        let is_stack_manipulation = vec_of_operations
            .iter()
            .any(|x| {
                x == &Operations::Duplicate || x == &Operations::Drop || x == &Operations::Over || x == &Operations::Swap
            });

        if !is_stack_manipulation {
            let nums_ops_zip = vec_of_nums
                .chunks(2)
                .zip(vec_of_operations);
            match Forth::calculate_integer_arithmetic(nums_ops_zip) {
                Ok(v) => {
                    self.stack = v;
                    Ok(())
                },
                Err(e) => Err(e)
            }
        } else {
            Err(Error::UnknownWord)
        }
    }
}
