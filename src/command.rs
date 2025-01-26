use std::io::Error;

use shell_starter_rust::tokenizer::Token;

pub mod executor;
pub mod builtin {
    pub mod cd;
    pub mod echo;
    pub mod exit;
    pub mod pwd;
    pub mod type_;
}


pub trait Command {
    fn run(&self, tokens: &Vec<Token>) -> Result<String, Error>;
}
