#[allow(unused_assignments)]
use std::io::{Error, ErrorKind};

use super::token::Token;

#[derive(PartialEq, Eq)]
enum ParseMode {
    None,
    Value,
    StringSingle,
    StringDouble,
    SingleDashArg,
    DoubleDashArg,
}

pub struct Tokenizer {
    temp: String,
    mode: ParseMode,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            temp: String::new(),
            mode: ParseMode::None,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, str: String) -> Result<&Vec<Token>, Error> {
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
                    ' ' => {
                        self.push_space();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::Value => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' => self.temp.push(ch),
                    ' ' => {
                        self.push_input();
                        self.push_space();
                    }
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
                    ' ' => {
                        self.push_input();
                        self.push_space();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::DoubleDashArg => match ch {
                    'a'..='z' | 'A'..='Z' | '_' | '-' => self.temp.push(ch),
                    ' ' => {
                        self.push_input();
                        self.push_space();
                    }
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
                return Ok(&self.tokens);
            }
            _ => {
                self.push_input();
                return Ok(&self.tokens);
            }
        }
    }

    fn push_input(&mut self) {
        match self.mode {
            ParseMode::None => panic!("This shouldn't have happened!"),
            ParseMode::Value => self.tokens.push(Token::Command(self.temp.to_string())),
            ParseMode::StringSingle => self
                .tokens
                .push(Token::String(self.temp.to_string(), false)),
            ParseMode::StringDouble => self.tokens.push(Token::String(self.temp.to_string(), true)),
            ParseMode::SingleDashArg => self
                .tokens
                .push(Token::Argument(self.temp.to_string(), false)),
            ParseMode::DoubleDashArg => self
                .tokens
                .push(Token::Argument(self.temp.to_string(), true)),
        }
        self.temp = String::new();
        self.mode = ParseMode::None;
    }

    fn push_space(&mut self) {
        if self.tokens.last() != Some(&Token::Space) {
            self.tokens.push(Token::Space);
        }
    }
}
