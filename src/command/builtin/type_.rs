use std::io::{Error, ErrorKind};

use shell_starter_rust::{tokenizer::Token, util::path::ExecutionPath};

use crate::command::{executor::SUPPORTED_COMMANDS, Command};

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
            Token::Value(input) => match input.get_exec_path() {
                Some(path) => return Ok(format!("{} is {}", input, path.to_str().unwrap())),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} not found", input),
                    ))
                }
            },
            Token::Argument(_, _) => todo!(),
            Token::String(input, _) => match input.get_exec_path() {
                Some(path) => return Ok(format!("{} is {}", input, path.to_str().unwrap())),
                None => {
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
