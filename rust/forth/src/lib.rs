use std::collections::HashMap;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

const ADD: &str = "+";
const SUBTRACT: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";
const DROP: &str = "drop";
const DUP: &str = "dup";
const SWAP: &str = "swap";
const OVER: &str = "over";
const WORD_DEF_START: &str = ":";
const WORD_DEF_END: &str = ";";

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut is_adding_word: bool = false;
        let mut new_word = String::new();
        let mut word_definition = String::new();

        for entry in input.split_whitespace() {
            let entry_lowercase = &entry.clone().to_lowercase()[..];
            match entry_lowercase {
                WORD_DEF_START if !is_adding_word => {
                    is_adding_word = true;
                    new_word = String::new();
                    word_definition = String::new();
                }

                WORD_DEF_END if is_adding_word => {
                    is_adding_word = false;
                    //if the 'new word' already exists, check the definitions of other words in the dictionary
                    //and expand any occurance of the 'new word' with it's old definition.
                    if let Some(old_definition) =
                        self.words.insert(new_word.clone(), word_definition.clone())
                    {
                        self.replace(&new_word, &old_definition);
                    }
                }

                e if is_adding_word && new_word.is_empty() && e.parse::<i32>().is_ok() => {
                    return Err(Error::InvalidWord)
                }

                e if is_adding_word && new_word.is_empty() => new_word = e.into(),
                e if is_adding_word => word_definition = format!("{} {}", word_definition, e),
                e if self.words.contains_key(e) => {
                    let input = self.words.get(e).unwrap().to_owned();
                    self.eval(&input)?
                }
                e if e.parse::<Value>().is_ok() => self.stack.push(e.parse::<Value>().unwrap()),
                ADD | SUBTRACT | MULTIPLY | DIVIDE => self.do_arithmetic(entry_lowercase)?,
                DROP | DUP | SWAP | OVER => self.do_stack_op(entry_lowercase)?,
                _ => return Err(Error::UnknownWord),
            }
        }

        match is_adding_word {
            true => Err(Error::InvalidWord),
            false => Ok(()),
        }
    }

    fn do_arithmetic(&mut self, operator: &str) -> Result {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        let (lhs, rhs) = match (lhs, rhs) {
            (None, _) => return Err(Error::StackUnderflow),
            (_, None) => return Err(Error::StackUnderflow),
            (_, Some(0)) if operator == DIVIDE => return Err(Error::DivisionByZero),
            (Some(lhs), Some(rhs)) => (lhs, rhs),
        };

        match operator {
            ADD => self.stack.push(lhs + rhs),
            SUBTRACT => self.stack.push(lhs - rhs),
            MULTIPLY => self.stack.push(lhs * rhs),
            DIVIDE => self.stack.push(lhs / rhs),
            _ => return Err(Error::InvalidWord),
        }

        Ok(())
    }

    fn do_stack_op(&mut self, command: &str) -> Result {
        let rhs = match self.stack.pop() {
            None => return Err(Error::StackUnderflow),
            Some(i) => i,
        };

        match command {
            DROP => Ok(()),
            DUP => {
                self.stack.push(rhs);
                self.stack.push(rhs);
                Ok(())
            }
            SWAP => match self.stack.pop() {
                None => Err(Error::StackUnderflow),
                Some(lhs) => {
                    self.stack.push(rhs);
                    self.stack.push(lhs);
                    Ok(())
                }
            },
            OVER => match self.stack.pop() {
                None => Err(Error::StackUnderflow),
                Some(lhs) => {
                    self.stack.push(lhs);
                    self.stack.push(rhs);
                    self.stack.push(lhs);
                    Ok(())
                }
            },
            _ => Err(Error::InvalidWord),
        }
    }

    fn replace(&mut self, word: &str, old_definition: &str) {
        // self.words.clone().iter().for_each(|(command, expansion)| {
        //     if expansion.split_whitespace().any(|term| term == word) {
        //         let definition = expansion
        //             .split_whitespace()
        //             .map(|term| match term.to_string() {
        //                 t if t == word => old_definition.to_owned(),
        //                 t => t,
        //             })
        //             .collect::<Vec<String>>()
        //             .join(" ");
        //         self.words.insert(command.clone(), definition);
        //     }
        // });
        
        for (command, expansion) in self.words.clone() {
            if expansion.split_whitespace().any(|term| term == word) {
                let definition = expansion
                    .split_whitespace()
                    .map(|term| match term.to_string() {
                        t if t == word => old_definition.to_owned(),
                        t => t,
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                self.words.insert(command.clone(), definition);
            }
        }
    }
}