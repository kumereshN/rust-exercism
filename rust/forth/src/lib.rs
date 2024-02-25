use std::collections::{BTreeMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub struct Forth {
    stack: Vec<Value>,
    btree: BTreeMap<String, VecDeque<String>>,
    has_user_defined_words: bool
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

const INTEGER_ARITHMETIC: [char; 4] = ['+', '-', '*', '/'];
const STACK_MANIPULATION: [&str; 4] = ["dup", "drop", "swap", "over"];

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            btree: BTreeMap::new(),
            has_user_defined_words: false
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn stack_manipulation(&mut self, input: VecDeque<String>) -> Result {
        if input.len() == 1 {
            return Err(Error::StackUnderflow)
        }

        let res =
                input
        .iter()
        .try_fold(Vec::new(), |mut acc, x| {
            match x.parse::<Value>() {
                Ok(v) => {
                    acc.push(v);
                    Ok(acc)
                }
                Err(_) => {
                    match x.as_str() {
                        "dup" => {
                            let last_value = acc.iter().last().unwrap();
                            acc.push(*last_value);
                            Ok(acc)
                        },
                        "drop" => {
                            acc.pop();
                            Ok(acc)
                        },
                        "swap" if acc.len() > 1 => {
                            let first_value = acc.pop().unwrap();
                            let second_value = acc.pop().unwrap();
                            acc.push(first_value);
                            acc.push(second_value);
                            Ok(acc)
                        },
                        "over" if acc.len() > 1 => {
                            let idx = acc.len() - 2;
                            let no_to_push = acc.get(idx).unwrap();
                            acc.push(*no_to_push);
                            Ok(acc)
                        }
                        _ => Err(Error::StackUnderflow)
                    }
                }
            }
        });
        self.stack = res?;
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

    pub fn user_defined_words(&mut self, mut input: VecDeque<String>) -> Result {
        let key_word = input.pop_front().unwrap();
        self.btree = BTreeMap::from([
            (key_word, input)
        ]);
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut input_split_on_whitespace = input.split_whitespace().map(|c| c.to_lowercase()).collect::<VecDeque<String>>();
        let is_user_defined_words = (input_split_on_whitespace.contains(&":".to_string())) && (input_split_on_whitespace.contains(&";".to_string()));

        if is_user_defined_words {
            input_split_on_whitespace.pop_front();
            input_split_on_whitespace.pop_back();
            self.has_user_defined_words = true;
            Forth::user_defined_words(self, input_split_on_whitespace)?;
            return Ok(())
        }

        if self.has_user_defined_words {
            for (key, value) in self.btree {
                let v = value.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(" ");
                let lowercase_input = input_split_on_whitespace.clone().into_iter().collect::<Vec<String>>().join(" ");
                let replacement_string = lowercase_input.replace(key.as_str(), v.as_str());
                self.has_user_defined_words = false;
                Forth::eval(self, replacement_string.as_str())?;
            }
            return Ok(())
        }

        println!("Stack is {:?}", self.stack());

        let is_stack_manipulation = input_split_on_whitespace
            .iter()
            .any(|x| {
                STACK_MANIPULATION.contains(&x.as_str())
            });

        if input_split_on_whitespace.iter().all(|x| x.parse::<Value>().is_ok()) {
            self.stack = input.split_ascii_whitespace().map(|x| x.parse::<Value>().unwrap()).collect::<Vec<Value>>();
            return Ok(())
        }

        if !is_stack_manipulation {
            Forth::calculate_integer_arithmetic(self, input)?
        } else {
            Forth::stack_manipulation(self, input_split_on_whitespace)?
        }
        Ok(())
    }
}
