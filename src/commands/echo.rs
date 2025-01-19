use std::io::{Error, ErrorKind};

use crate::shell::{Command, Token};

pub struct Echo {}

impl Command for Echo {
    fn cmd(&self, tokens: &Vec<Token>) -> Result<String, std::io::Error> {
        if tokens.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No string input found.",
            ));
        }

        if tokens.len() == 3 {
            let string = tokens.get(2).unwrap().get_value();
            return Ok(string);
        }

        let mut string = String::new();
        let mut is_string: Option<bool> = None;

        tokens
            .iter()
            .skip(2)
            .enumerate()
            .for_each(|(i, t)| match t {
                Token::Space => {
                    if i > 0 {
                        match is_string {
                            Some(true) => {
                                string.push(' ');
                            }
                            Some(false) => {
                                string.push(' ');
                            }
                            None => {}
                        }
                    }
                }
                Token::Command(cmd) => match is_string {
                    Some(true) => {}
                    Some(false) => string.push_str(cmd.as_str()),
                    None => {
                        is_string = Some(false);
                        string.push_str(cmd.as_str());
                    }
                },
                Token::Argument(_, _) => {}
                Token::String(str, _) => {
                    if is_string == None {
                        is_string = Some(true);
                        string.push_str(str.as_str());
                    } else if is_string == Some(true) {
                        string.push_str(str.as_str());
                    }
                }
            });

        return Ok(string);
    }
}
