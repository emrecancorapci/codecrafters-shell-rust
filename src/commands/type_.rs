use std::io::{Error, ErrorKind};

use shell_starter_rust::tokenizer::{path, Token};

use crate::command::Command;

use crate::command_handler::SUPPORTED_COMMANDS;

pub struct Type {}

impl Command for Type {
    fn run(&self, tokens: &Vec<Token>) -> Result<String, std::io::Error> {
        if tokens.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "This command needs argument.",
            ));
        }

        for command in SUPPORTED_COMMANDS.iter() {
            if tokens.get(2) == Some(&Token::Value(command.to_string())) {
                return Ok(format!("{} is a shell builtin", command));
            }
        }

        match tokens.get(2).unwrap() {
            Token::Space => {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Third token shouldn't be a space. Fix this.",
                ))
            }
            Token::Value(input) => match path::get_exec_path_string(input.as_str()) {
                Ok(path) => return Ok(format!("{} is {}", input, path)),
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} not found", input),
                    ))
                }
            },
            Token::Argument(_, _) => todo!(),
            Token::String(input, _) => match path::get_exec_path_string(input.as_str()) {
                Ok(path) => return Ok(format!("{} is {}", input, path)),
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} not found", input),
                    ))
                }
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "bash: syntax error near unexpected token 'newline'",
                ))
            }
        }
    }
}
