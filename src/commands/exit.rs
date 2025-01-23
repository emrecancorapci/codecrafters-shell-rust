use std::io::{Error, ErrorKind};

use shell_starter_rust::tokenizer::{Command, Token};

pub struct Exit {}

impl Command for Exit {
    fn run(&self, tokens: &Vec<Token>) -> Result<String, std::io::Error> {
        if tokens.get(2) == Some(&Token::Value("0".to_string())) {
            std::process::exit(0);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{}: command not found", tokens[0]),
            ));
        }
    }
}
