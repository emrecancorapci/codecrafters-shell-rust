use std::{
    io::{Error, ErrorKind},
    process::Output,
};

use run_command::RunCommand;
use shell_starter_rust::{
    tokenizer::{tokenize, Token},
    util::path::ExecutionPath,
};

mod redirected;
pub mod run_command;

pub struct InputHandler {
    command_handler: Box<dyn RunCommand>,
}

impl InputHandler {
    pub fn new(command_handler: impl RunCommand + 'static) -> InputHandler {
        InputHandler {
            command_handler: Box::new(command_handler),
        }
    }

    pub fn handle_input(&mut self, input: &String) -> Result<Vec<u8>, Error> {
        let tokens = tokenize(input.trim())?;

        if tokens.is_empty() {
            return Ok(vec![]);
        }

        let redirection_token = tokens.iter().any(|t| t.is_redirection_token());

        match redirection_token {
            true => self.handle_redirected_input(&tokens),
            false => self.handle_direct_input(&tokens),
        }
    }

    fn handle_direct_input(&self, tokens: &Vec<Token>) -> Result<Vec<u8>, Error> {
        match tokens.first().unwrap() {
            Token::Value(cmd) | Token::String(cmd, _) if cmd.get_exec_path().is_some() => {
                let output = InputHandler::execute_external(tokens, cmd)?;

                if output.status.success() {
                    let mut output_array = output.stdout.to_vec();

                    if output_array.last() == Some(&10) {
                        output_array.pop();
                    }

                    return Ok(output_array);
                }

                let mut error_array = output.stderr.to_vec();

                if error_array.last() == Some(&10) {
                    error_array.pop();
                }

                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    String::from_utf8(error_array).unwrap(),
                ));
            }
            Token::Value(cmd) | Token::String(cmd, _) => {
                match self.command_handler.run(cmd, &tokens) {
                    Ok(response) => return Ok(response.as_bytes().to_vec()),
                    Err(err) => return Err(err),
                }
            }
            _ => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
        }
    }

    fn execute_external(tokens: &[Token], cmd: &String) -> Result<Output, Error> {
        let input_array = tokens
            .iter()
            .skip(2)
            .filter(|i| !matches!(i, Token::Space))
            .map(|i| i.serialize());

        std::process::Command::new(cmd).args(input_array).output()
    }
}
