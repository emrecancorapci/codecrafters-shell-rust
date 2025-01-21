use std::io::Error;

pub mod path;

mod command;
mod token;
mod parser;

pub use command::Command;
use parser::ParseMode;
pub use token::Token;

pub struct Tokenizer {
    temp: String,
    mode: ParseMode,
    sub_mode: ParseMode,
    tokens: Vec<Token>,
    redirection_token: Option<(Token, Vec<Token>)>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            temp: String::new(),
            mode: ParseMode::None,
            sub_mode: ParseMode::None,
            tokens: Vec::new(),
            redirection_token: None,
        }
    }

    pub fn parse(&mut self, input: String) -> Result<Vec<Token>, Error> {
        let mut iter = input.chars().into_iter().enumerate().peekable();

        self.parse_tokens(&mut iter)
    }

    pub fn is_redirected(&self) -> bool {
        self.redirection_token.is_some()
    }

    pub fn get_redirection_type(&self) -> Option<&Token> {
        match self.redirection_token {
            Some((ref token, _)) => Some(token),
            None => None,
        }
    }

    pub fn get_redirection_tokens(&self) -> Option<Vec<Token>> {
        match self.redirection_token {
            Some((_, ref tokens)) => Some(tokens.to_vec()),
            None => None,
        }
    }
}
