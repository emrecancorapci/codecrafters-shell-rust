use std::{
    io::{Error, ErrorKind},
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub use token::Token;

mod token;

pub struct Tokenizer {
    temp: String,
    mode: ParseMode,
    sub_mode: ParseMode,
    tokens: Vec<Token>,
}

#[derive(PartialEq, Eq)]
pub(super) enum ParseMode {
    None,
    Value,
    SingleQuote,
    DoubleQuote,
    SingleDashArg,
    DoubleDashArg,
}

impl Tokenizer {
    pub fn tokenize(input: String) -> Result<Vec<Token>, Error> {
        let mut iter = input.chars().into_iter().enumerate().peekable();
        let mut tokenizer = Tokenizer {
            temp: String::new(),
            mode: ParseMode::None,
            sub_mode: ParseMode::None,
            tokens: Vec::new(),
        };

        while let Some((i, ch)) = iter.next() {
            match tokenizer.mode {
                ParseMode::None => match ch {
                    '\'' => tokenizer.mode = ParseMode::SingleQuote,
                    '"' => tokenizer.mode = ParseMode::DoubleQuote,
                    '\\' => {
                        tokenizer.mode = ParseMode::Value;
                        let ch = iter.peek();

                        match ch {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                tokenizer.temp.push(ch)
                            }
                            None => todo!(),
                        }
                    }
                    '-' => {
                        if iter.peek() == Some(&(i + 1, '-')) {
                            iter.next();
                            tokenizer.mode = ParseMode::DoubleDashArg
                        } else {
                            tokenizer.mode = ParseMode::SingleDashArg
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' | '.' | '/' | '~' if tokenizer.temp.is_empty() => {
                        tokenizer.mode = ParseMode::Value;
                        tokenizer.temp.push(ch);
                    }
                    '0'..='9' if tokenizer.temp.is_empty() => {
                        if let Some((_, '>')) = iter.peek() {
                            iter.next();
                            tokenizer.parse_redirector(&mut iter, ch)?;
                            break;
                        } else {
                            tokenizer.temp.push(ch);
                            tokenizer.mode = ParseMode::Value;
                        }
                    }
                    '>' => {
                        tokenizer.parse_redirector(&mut iter, '1')?;
                        break;
                    }
                    ' ' => {
                        tokenizer.push_space();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::Value => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' | '/' => {
                        tokenizer.temp.push(ch)
                    }
                    '\\' => {
                        let ch = iter.peek();

                        match ch {
                            Some(_) => {
                                let (_index, ch) = iter.next().unwrap();

                                tokenizer.temp.push(ch)
                            }
                            None => todo!(),
                        }
                    }
                    ' ' => {
                        tokenizer.push_input();
                        tokenizer.push_space();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid character at {}", i),
                        ))
                    }
                },
                ParseMode::SingleQuote => match ch {
                    '\'' => tokenizer.push_input(),
                    _ => tokenizer.temp.push(ch),
                },
                ParseMode::DoubleQuote => match ch {
                    '"' => tokenizer.push_input(),
                    '\\' => {
                        if tokenizer.sub_mode == ParseMode::SingleQuote {
                            tokenizer.temp.push(ch);
                            continue;
                        }

                        match iter.peek() {
                            Some((_, '\\' | '$' | '"')) => {
                                let (_index, ch) = iter.next().unwrap();

                                tokenizer.temp.push(ch)
                            }
                            Some(_) => {
                                tokenizer.temp.push(ch);
                            }
                            None => todo!(),
                        }
                    }
                    '\'' => match tokenizer.sub_mode {
                        ParseMode::None => {
                            tokenizer.temp.push(ch);
                            tokenizer.sub_mode = ParseMode::SingleQuote;
                        }
                        ParseMode::SingleQuote => {
                            tokenizer.temp.push(ch);
                            tokenizer.sub_mode = ParseMode::None;
                        }
                        _ => todo!(),
                    },
                    _ => tokenizer.temp.push(ch),
                },
                ParseMode::SingleDashArg | ParseMode::DoubleDashArg => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => tokenizer.temp.push(ch),
                    ' ' => {
                        tokenizer.push_input();
                        tokenizer.push_space();
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

        match tokenizer.mode {
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
                return Ok(tokenizer.tokens.to_vec());
            }
            _ => {
                tokenizer.push_input();
                return Ok(tokenizer.tokens.to_vec());
            }
        }
    }

    fn parse_redirector(
        &mut self,
        iter: &mut Peekable<Enumerate<Chars<'_>>>,
        prefix: char,
    ) -> Result<(), Error> {
        match iter.peek() {
            Some((_, '>')) => {
                iter.next();

                self.tokens.push(Token::Appender(prefix));
                Ok(())
            }
            Some(_) => {
                self.tokens.push(Token::Redirector(prefix));
                Ok(())
            }

            None => return Err(Error::new(ErrorKind::InvalidInput, "No redirection target")),
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
