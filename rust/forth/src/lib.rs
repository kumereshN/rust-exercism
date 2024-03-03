use std::collections::{HashMap};

pub type Value = i32;
pub type ForthResult = std::result::Result<(), Error>;
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Id>,
    definitions: Vec<Vec<Op>>
}

type Id = u16;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Push(Value),
    Call(Id),
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Run,
    Word,
    Define(String, Vec<Op>)
}

impl Forth {
    pub fn new() -> Forth {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut mode = Mode::Run;

        for token in input.to_uppercase().split_whitespace() {
            match mode {
                Mode::Run => {
                    if token == ":" {
                        mode = Mode::Word;
                    } else {
                        eval_op(
                            parse_op(token, &self.words)?,
                            &mut self.stack,
                            &self.definitions,
                        )?;
                    }
                }
                Mode::Word => {
                    // It must be a word not a number
                    if token.parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    mode = Mode::Define(token.into(), Vec::new());
                }
                Mode::Define(_, ref mut definition) => {
                    // Reach the end of defined words
                    if token == ";" {
                        if let Mode::Define(word, definition) =
                            std::mem::replace(&mut mode, Mode::Run) {
                            self.definitions.push(definition);
                            self.words.insert(word, self.definitions.len() as Id - 1);
                        }
                    } else {
                        // If there are more words or numbers to be defined
                        definition.push(parse_op(token, &self.words)?);
                    }
                }
            }
        }

        (mode == Mode::Run).then_some(()).ok_or(Error::InvalidWord)
    }
}

fn parse_op(token: &str, words: &HashMap<String, Id>) -> Result<Op, Error> {
    Ok(if let Some(id) = words.get(token) {
        Op::Call(*id)
    } else {
        match token {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "DUP" => Op::Dup,
            "DROP" => Op::Drop,
            "SWAP" => Op::Swap,
            "OVER" => Op::Over,
            _ => Op::Push(token.parse::<Value>().map_err(|_| Error::UnknownWord)?),
        }
    })
}

fn eval_op(op: Op, stack: &mut Vec<Value>, definitions: &Vec<Vec<Op>>) -> ForthResult {
    let mut pop = || stack.pop().ok_or(Error::StackUnderflow);
    match op {
        Op::Add => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs + rhs);
        },
        Op::Sub => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs - rhs);
        },
        Op::Mul => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs * rhs);
        },
        Op::Div => {
            let (rhs, lhs) = (pop()?, pop()?);
            let quotient = lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?;
            stack.push(quotient);
        },
        Op::Dup => {
            let first = *stack.last().ok_or(Error::StackUnderflow)?;
            stack.push(first);
        }
        Op::Drop => {
            pop()?;
        },
        Op::Swap => {
            let (first, second) = (pop()?, pop()?);
            stack.push(first);
            stack.push(second);
        },
        Op::Over => {
            let below = *stack.iter().nth_back(1).ok_or(Error::StackUnderflow)?;
            stack.push(below);
        },
        Op::Push(num) => {
            stack.push(num);
        },
        Op::Call(id) => {
            for op in &definitions[id as usize] {
                eval_op(*op, stack, definitions)?;
            }
        }
    }
    Ok(())
}


