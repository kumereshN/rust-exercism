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

    pub fn stack_manipulation(&mut self, input: Vec<&str>) -> Result {
        if input.len() == 1 {
            return Err(Error::StackUnderflow)
        }

        self.stack = input
        .iter()
        .fold(Vec::new(), |mut acc, &x| {
            match x.parse::<Value>() {
                Ok(v) => {
                    acc.push(v);
                    acc
                }
                Err(e) => {
                    match x {
                        "dup" => {
                            let last_value = acc.iter().last().unwrap();
                            acc.push(*last_value);
                            acc
                        }
                        "drop" => {
                            acc.pop();
                            acc
                        }
                        _ => panic!("Invalid operation due to {}", e)
                    }
                }
            }
        });
        Ok(())
    }

    pub fn calculate_integer_arithmetic(&mut self, input: &str) -> Result {
        let vec_res = input.split_inclusive(INTEGER_ARITHMETIC).collect::<VecDeque<&str>>();
        let first_value_vec_res = vec_res.front().unwrap().split_ascii_whitespace().collect::<Vec<_>>();
        match first_value_vec_res.len() {
            3 => {
                let mut res = 0;
                for val in vec_res {
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
                Ok(())
            },
            _ => Err(Error::StackUnderflow)
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let lowercase_input = input.to_ascii_lowercase();
        let input_split_on_whitespace = lowercase_input.as_str().split_whitespace().collect::<Vec<&str>>();
        if input_split_on_whitespace.iter().all(|x| x.parse::<Value>().is_ok()) {
            self.stack = input.split_ascii_whitespace().map(|x| x.parse::<Value>().unwrap()).collect::<Vec<Value>>();
            return Ok(())
        }

        let is_stack_manipulation = input_split_on_whitespace
            .iter()
            .any(|&x| {
                STACK_MANIPULATION.contains(&x)
            });

        if !is_stack_manipulation {
            Forth::calculate_integer_arithmetic(self, input)?
        } else {
            Forth::stack_manipulation(self, input_split_on_whitespace)?;
        }
        Ok(())
    }
}
