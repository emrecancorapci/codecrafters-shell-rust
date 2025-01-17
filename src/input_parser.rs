use std::fmt::Display;
#[allow(unused_assignments)]
use std::io::{Error, ErrorKind};

#[derive(PartialEq, Eq, Debug)]
pub enum Input {
    Command(String),
    Argument(String, bool),
    String(String, bool),
}

impl Input {
    pub fn get_value(&self) -> String {
        match self{
            Input::Command(val) => val.to_string(),
            Input::Argument(val, _) => val.to_string(),
            Input::String(val, _) => val.to_string(),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq, Eq)]
enum ParseMode {
    None,
    Value,
    StringSingle,
    StringDouble,
    SingleDashArg,
    DoubleDashArg,
}

pub struct InputParser {
    temp: String,
    mode: ParseMode,
    result: Vec<Input>,
}

impl InputParser {
    pub fn new() -> InputParser {
        InputParser {
            temp: String::new(),
            mode: ParseMode::None,
            result: Vec::new(),
        }
    }

    pub fn parse(&mut self, str: String) -> Result<&Vec<Input>, Error> {
        let mut iter = str.chars().into_iter().enumerate().peekable();

        while let Some((i, ch)) = iter.next() {
            match self.mode {
                ParseMode::None => match ch {
                    '\'' => self.mode = ParseMode::StringSingle,
                    '"' => self.mode = ParseMode::StringDouble,
                    '-' => {
                        if iter.peek() == Some(&(i + 1, '-')) {
                            iter.next();
                            self.mode = ParseMode::DoubleDashArg
                        } else {
                            self.mode = ParseMode::SingleDashArg
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' if self.temp.is_empty() => {
                        self.mode = ParseMode::Value;
                        self.temp.push(ch);
                    }
                    ' ' => {}
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::Value => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => self.temp.push(ch),
                    ' ' => self.push_input(),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::StringSingle => match ch {
                    '\'' => self.push_input(),
                    _ => self.temp.push(ch),
                },
                ParseMode::StringDouble => match ch {
                    '"' => self.push_input(),
                    _ => self.temp.push(ch),
                },
                ParseMode::SingleDashArg => match ch {
                    'a'..='z' | 'A'..='Z' | '_' | '-' => self.temp.push(ch),
                    ' ' => self.push_input(),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::DoubleDashArg => match ch {
                    'a'..='z' | 'A'..='Z' | '_' | '-' => self.temp.push(ch),
                    ' ' => self.push_input(),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
            }
        }

        match self.mode {
            ParseMode::StringSingle => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Single quote didn't end.",
                ))
            }
            ParseMode::StringDouble => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Double quote didn't end.",
                ))
            }
            ParseMode::None => {
                return Ok(&self.result);
            }
            _ => {
                self.push_input();
                return Ok(&self.result);
            }
        }
    }

    fn push_input(&mut self) {
        match self.mode {
            ParseMode::None => panic!("This shouldn't have happened!"),
            ParseMode::Value => self.result.push(Input::Command(self.temp.to_string())),
            ParseMode::StringSingle => self
                .result
                .push(Input::String(self.temp.to_string(), false)),
            ParseMode::StringDouble => self.result.push(Input::String(self.temp.to_string(), true)),
            ParseMode::SingleDashArg => self
                .result
                .push(Input::Argument(self.temp.to_string(), false)),
            ParseMode::DoubleDashArg => self
                .result
                .push(Input::Argument(self.temp.to_string(), true)),
        }
        self.temp = String::new();
        self.mode = ParseMode::None;
    }
}
