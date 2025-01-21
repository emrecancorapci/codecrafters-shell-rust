#[allow(unused_assignments)]
use std::io::{Error, ErrorKind};

use super::token::Token;

#[derive(PartialEq, Eq)]
enum ParseMode {
    None,
    Value,
    SingleQuote,
    DoubleQuote,
    SingleDashArg,
    DoubleDashArg,
}

pub struct Tokenizer {
    temp: String,
    mode: ParseMode,
    sub_mode: ParseMode,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            temp: String::new(),
            mode: ParseMode::None,
            sub_mode: ParseMode::None,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, str: String) -> Result<&Vec<Token>, Error> {
        let mut iter = str.chars().into_iter().enumerate().peekable();

        while let Some((i, ch)) = iter.next() {
            match self.mode {
                ParseMode::None => match ch {
                    '\'' => self.mode = ParseMode::SingleQuote,
                    '"' => self.mode = ParseMode::DoubleQuote,
                    '\\' => {
                        self.mode = ParseMode::Value;
                        let ch = iter.peek();

                        match ch {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                self.temp.push(ch)
                            }
                            None => todo!(),
                        }
                    }
                    '-' => {
                        if iter.peek() == Some(&(i + 1, '-')) {
                            iter.next();
                            self.mode = ParseMode::DoubleDashArg
                        } else {
                            self.mode = ParseMode::SingleDashArg
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '/' | '~'
                        if self.temp.is_empty() =>
                    {
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
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | '/' => self.temp.push(ch),
                    '\\' => {
                        let ch = iter.peek();

                        match ch {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                self.temp.push(ch)
                            }
                            None => todo!(),
                        }
                    }
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
                ParseMode::SingleQuote => match ch {
                    '\'' => self.push_input(),
                    _ => self.temp.push(ch),
                },
                ParseMode::DoubleQuote => match ch {
                    '"' => self.push_input(),
                    '\\' => {
                        if self.sub_mode == ParseMode::SingleQuote {
                            self.temp.push(ch);
                            continue;
                        }

                        match iter.peek() {
                            Some((_, '\\' | '$' | '"')) => {
                                let (_index, ch) = iter.next().unwrap();

                                self.temp.push(ch)
                            }
                            Some(_) => {
                                self.temp.push(ch);
                            }
                            None => todo!(),
                        }
                    }
                    '\'' => match self.sub_mode {
                        ParseMode::None => {
                            self.temp.push(ch);
                            self.sub_mode = ParseMode::SingleQuote;
                        }
                        ParseMode::SingleQuote => {
                            self.temp.push(ch);
                            self.sub_mode = ParseMode::None;
                        }
                        _ => todo!(),
                    },
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
            ParseMode::SingleQuote => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Single quote didn't end.",
                ))
            }
            ParseMode::DoubleQuote => {
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
            ParseMode::Value => self.tokens.push(Token::Value(self.temp.to_string())),
            ParseMode::SingleQuote => self
                .tokens
                .push(Token::String(self.temp.to_string(), false)),
            ParseMode::DoubleQuote => self.tokens.push(Token::String(self.temp.to_string(), true)),
            ParseMode::SingleDashArg => self
                .tokens
                .push(Token::Argument(self.temp.to_string(), false)),
            ParseMode::DoubleDashArg => self
                .tokens
                .push(Token::Argument(self.temp.to_string(), true)),
        }
        self.temp = String::new();
        self.mode = ParseMode::None;
        self.sub_mode = ParseMode::None;
    }

    fn push_space(&mut self) {
        if self.tokens.last() != Some(&Token::Space) {
            self.tokens.push(Token::Space);
        }
    }
}
