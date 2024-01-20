use std::collections::{BTreeMap, VecDeque};

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

const INTEGER_ARITHMETIC: [char; 4] = ['+', '-', '*', '/'];
const STACK_MANIPULATION: [&str; 4] = ["dup", "drop", "swap", "over"];

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
                    None => return Err(Error::DivisionByZero)
                }
                (Operations::Divide, _) => res /= v[0],
                _ => panic!("Error occurred in operations")
            }
        }
        Ok(vec![res])
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut input_split_on_whitespace = input.split_ascii_whitespace();
        if input_split_on_whitespace.all(|x| x.parse::<Value>().is_ok()) {
            self.stack = input.split_ascii_whitespace().map(|x| x.parse::<Value>().unwrap()).collect::<Vec<Value>>();
            return Ok(())
        }

        let is_stack_manipulation = input_split_on_whitespace
            .any(|x| {
                STACK_MANIPULATION.contains(&x)
            });

        let vec_res = input.split_inclusive(INTEGER_ARITHMETIC).collect::<VecDeque<&str>>();
        let first_value_vec_res = vec_res.front().unwrap().split_ascii_whitespace().collect::<Vec<_>>();
        match first_value_vec_res.len() {
            3 => {
                let mut res = 0;
                for val in vec_res{
                    let mut split_whitespace_deque = val.split_ascii_whitespace().collect::<VecDeque<&str>>();
                    let ops = split_whitespace_deque.pop_back();
                    let len_split_whitespace_deque = split_whitespace_deque.len();
                    let split_whitespace_deque_parsed = split_whitespace_deque
                        .iter()
                        .map(|x| x.parse::<Value>().unwrap())
                        .collect::<Vec<Value>>();
                    let (first_value, second_value) = (split_whitespace_deque_parsed[0], split_whitespace_deque_parsed.get(1).unwrap_or(&-1));

                    match (ops, len_split_whitespace_deque) {
                        (Some("+"), _) => {
                            res += split_whitespace_deque_parsed.iter().sum::<Value>();
                        },
                        (Some("-"), 2) => {
                            res += first_value - second_value
                        },
                        (Some("-"), _) => {
                            res -= first_value
                        }
                        (Some("*"), 2) => {
                            res += first_value * second_value
                        },
                        (Some("*"), _) => {
                            res *= first_value
                        },
                        (Some("/"), 2) => {
                            res += match first_value.checked_div_euclid(*second_value) {
                                Some(v) => v,
                                None => return Err(Error::DivisionByZero)
                            }
                        },
                        (Some("/"), _) => {
                            res /= first_value
                        }
                        _ => panic!("Something went wrong")
                    }
                }
                self.stack = vec![res];
            },
            _ => return Err(Error::StackUnderflow)
        }
        Ok(())
    }
}
