use std::iter::Zip;
use std::slice::Chunks;
use std::vec::IntoIter;
use std::collections::BTreeMap;
use crate::Error::{DivisionByZero, StackUnderflow};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub struct Forth {
    stack: Vec<Value>,
    btree: BTreeMap<Vec<Value>, Operations>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
            stack: vec![],
            btree: BTreeMap::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn stack_manipulation(vec_of_nums: Vec<Value>, vec_of_ops: &[Operations]) -> std::result::Result<Vec<Value>, Error> {
        let mut res = vec_of_nums.to_vec();
        let last_digit = vec_of_nums.last().unwrap();
        for ops in vec_of_ops {
            match ops {
                Operations::Duplicate => {
                    res.push(*last_digit)
                },
                Operations::Drop => {
                    res.pop();
                }
                _ => panic!("Invalid ops")
            }
        }
        Ok(res)
    }

    pub fn calculate_integer_arithmetic(btree_value_ops: &BTreeMap<Vec<Value>, Operations>) -> std::result::Result<Vec<Value>, Error> {
        let mut res = 0;

        for (v, ops) in btree_value_ops {
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
        let mut temp_stack = vec![];

        for s in input.split_ascii_whitespace() {
            match s.parse::<Value>() {
                Ok(n) => temp_stack.push(n),
                Err(e) => match s.to_lowercase().as_str() {
                    "+" => {
                        self.btree.insert(temp_stack.clone(), Operations::Add);
                        temp_stack.clear()
                    },
                    "-" => {
                        self.btree.insert(temp_stack.clone(), Operations::Subtract);
                        temp_stack.clear()
                    },
                    "*" => {
                        self.btree.insert(temp_stack.clone(), Operations::Multiply);
                        temp_stack.clear()
                    },
                    "/" => {
                        self.btree.insert(temp_stack.clone(), Operations::Divide);
                        temp_stack.clear()
                    },
                    "dup" => {
                        self.btree.insert(temp_stack.clone(), Operations::Duplicate);
                        temp_stack.clear()
                    },
                    "drop" => {
                        self.btree.insert(temp_stack.clone(), Operations::Drop);
                        temp_stack.clear()
                    },
                    _ => panic!("Error occurred: {}", e)
                }
            }
        }

        let vec_of_nums = self.btree.keys().flatten().cloned().collect::<Vec<Value>>() ;
        let vec_of_operations = self.btree.values().cloned().collect::<Vec<_>>();


        if vec_of_operations.is_empty() {
            self.stack = temp_stack;
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

            match Forth::calculate_integer_arithmetic(&self.btree) {
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
            match Forth::stack_manipulation(vec_of_nums, vec_of_operations.as_slice()) {
                Ok(v) => {
                    self.stack = v;
                    Ok(())
                },
                Err(e) => Err(e)
            }
        }
    }
}
