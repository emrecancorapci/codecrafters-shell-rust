use std::io::Error;

use shell_starter_rust::tokenizer::Token;

pub trait RunCommand {
    fn run(&self, cmd: &str, tokens: &Vec<Token>) -> Result<String, Error>;
}