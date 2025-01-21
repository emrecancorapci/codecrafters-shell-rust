use std::io::Error;

use super::Token;

pub trait Command {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, Error>;
}
