use std::iter::Zip;
use std::slice::Chunks;
use std::vec::IntoIter;
use std::collections::BTreeMap;
use crate::Error::{DivisionByZero, StackUnderflow};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub struct Forth<'a> {
    stack: Vec<Value>,
    btree: BTreeMap<&'a str, Vec<u32>>
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

impl<'a> Forth<'a> {
    pub fn new() -> Forth<'a> {
        Forth {
            stack: vec![],
            btree: BTreeMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn stack_manipulation(vec_of_nums: &[Value], vec_of_ops: &[Operations]) -> std::result::Result<Vec<Value>, Error> {
        let mut res = vec_of_nums.to_vec();
        let last_digit = *vec_of_nums.last().unwrap();
        for ops in vec_of_ops {
            match ops {
                Operations::Duplicate => {
                    res.push(last_digit)
                },
                Operations::Drop => {
                    res.pop();
                }
                _ => panic!("Invalid ops")
            }
        }
        Ok(res)
    }

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
                    "drop" => vec_of_operations.push(Operations::Drop),
                    _ => panic!("Error occurred: {}", e)
                }
            }
        }

        if vec_of_operations.is_empty() {
            self.stack = vec_of_nums;
            return Ok(())
        }

        let is_stack_manipulation = vec_of_operations
            .iter()
            .any(|x| {
                x == &Operations::Duplicate || x == &Operations::Drop || x == &Operations::Over || x == &Operations::Swap
            });

        if !is_stack_manipulation {

            if vec_of_nums.len().saturating_sub(1) != (vec_of_operations.len()) {
                return Err(StackUnderflow)
            }

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
            if vec_of_nums.is_empty() || vec_of_operations.is_empty() {
                return Err(Error::StackUnderflow)
            }
            match Forth::stack_manipulation(&vec_of_nums, &vec_of_operations) {
                Ok(v) => {
                    self.stack = v;
                    Ok(())
                },
                Err(e) => Err(e)
            }
        }
    }
}
