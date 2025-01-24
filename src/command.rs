use std::io::Error;

use shell_starter_rust::tokenizer::Token;

pub trait Command {
    fn run(&self, tokens: &Vec<Token>) -> Result<String, Error>;
}
