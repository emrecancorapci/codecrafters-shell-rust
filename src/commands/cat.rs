use std::{fs, io::{Error, ErrorKind}};

use crate::shell::{Command, Token};

pub struct Cat {}

impl Command for Cat {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, Error> {
        if tokens.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "cd: missing argument"));
        }

        match tokens.get(2) {
            Some(Token::Command(cmd)) => {
                return fs::read_to_string(cmd);
            },
            Some(Token::String(str, _)) => {
                return fs::read_to_string(str);
            },
            _ => todo!(),
        }
    }
}
