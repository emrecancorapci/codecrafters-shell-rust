use std::{
    fs,
    io::{Error, ErrorKind},
};

use crate::shell::{Command, Token};

pub struct Cat {}

impl Command for Cat {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, Error> {
        if tokens.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "cd: missing argument"));
        }

        let mut response = String::new();

        let mut iter = tokens.iter().skip(2).enumerate();

        while let Some((_index, token)) = iter.next() {
            match token {
                Token::Command(cmd) => {
                    let file_content = fs::read_to_string(cmd);

                    if file_content.is_err() {
                        return Err(Error::new(ErrorKind::InvalidInput, "cat: file not found"));
                    } else {
                        response.push_str(file_content.unwrap().as_str());
                    }
                }
                Token::String(str, _) => {
                    let file_content = fs::read_to_string(str);

                    if file_content.is_err() {
                        return Err(Error::new(ErrorKind::InvalidInput, "cat: file not found"));
                    } else {
                        response.push_str(file_content.unwrap().as_str());
                    }
                }
                Token::Space => {}
                _ => {
                    eprintln!("{} This should not happened", token);
                }
            }
        }

        return Ok(response);
    }
}
