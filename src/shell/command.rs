use std::io::Error;

use super::token::Token;

pub trait Command {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, Error>;
}
