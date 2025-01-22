use std::io::Error;

pub mod path;

mod command;
mod parser;
mod token;

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

    pub fn parse(&mut self, input: String) -> Result<(), Error> {
        let mut iter = input.chars().into_iter().enumerate().peekable();

        self.parse_tokens(&mut iter)?;

        return Ok(());
    }

    pub fn clear(&mut self) {
        self.temp = String::new();
        self.mode = ParseMode::None;
        self.sub_mode = ParseMode::None;
        self.tokens = Vec::new();
        self.redirection_token = None;
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.to_vec()
    }

    pub fn get_tokens_ref(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn is_redirect(&self) -> bool {
        matches!(self.redirection_token, Some((Token::Redirector(_), _)))
    }

    pub fn is_append(&self) -> bool {
        matches!(self.redirection_token, Some((Token::Appender(_), _)))
    }

    pub fn is_redirect_err(&self) -> bool {
        matches!(self.redirection_token, Some((Token::Redirector(2), _)))
    }

    pub fn is_append_err(&self) -> bool {
        matches!(self.redirection_token, Some((Token::Appender(2), _)))
    }

    pub fn get_redirection_type(&self) -> Option<&Token> {
        match self.redirection_token {
            Some((ref token, _)) => Some(token),
            None => None,
        }
    }

    pub fn get_redirection_tokens(&self) -> Vec<Token> {
        match self.redirection_token {
            Some((_, ref tokens)) => tokens.to_vec(),
            None => vec![],
        }
    }
}
