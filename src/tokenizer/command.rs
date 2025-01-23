use std::io::Error;

use super::Token;

pub trait Command {
    fn run(&self, tokens: &Vec<Token>) -> Result<String, Error>;
}
