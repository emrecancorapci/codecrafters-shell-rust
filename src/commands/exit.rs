use std::io::{Error, ErrorKind};

use crate::shell::{Command, Token};

pub struct Exit {}

impl Command for Exit {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, std::io::Error> {
        if tokens.get(2) == Some(&Token::Command("0".to_string())) {
            std::process::exit(0);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{}: command not found\n", tokens[0]),
            ));
        }
    }
}
