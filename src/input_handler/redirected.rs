use std::{
    fs,
    io::{Error, ErrorKind},
};

use shell_starter_rust::{
    tokenizer::Token,
    util::{error::AsBytes, output::SplitOutput, path::ExecutionPath},
};

use super::InputHandler;
impl InputHandler {
    pub(super) fn handle_redirected_input(&self, tokens: &[Token]) -> Result<Vec<u8>, Error> {
        let redirection = tokens
            .iter()
            .position(|t| t.is_redirection_token())
            .unwrap();

        let (tokens, redirection_tokens) = tokens.split_at(redirection);

        let (response, error) = match tokens.first() {
            Some(Token::Value(cmd) | Token::String(cmd, _)) if cmd.get_exec_path().is_some() => {
                let output = InputHandler::execute_external(tokens, cmd)?;

                output.split_output()
            }
            Some(Token::Value(cmd) | Token::String(cmd, _)) => {
                match self.command_handler.run(cmd, &tokens.to_vec()) {
                    Ok(response) => (Some(response.as_bytes().to_vec()), None),
                    Err(err) => (None, Some(err)),
                }
            }
            Some(_) => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
            None => return Ok(vec![]),
        };

        execute_redirected(redirection_tokens, response, error)
    }
}

fn execute_redirected(
    redirection_tokens: &[Token],
    output: Option<Vec<u8>>,
    error: Option<Error>,
) -> Result<Vec<u8>, Error> {
    let path = redirection_tokens.get(2).unwrap().serialize();

    match redirection_tokens.first().unwrap() {
        Token::Redirector('1') => {
            fs::write(path, output.unwrap())?;

            match error {
                Some(err) => Err(err),
                None => Ok(vec![]),
            }
        }
        Token::Redirector('2') => {
            fs::write(path, error.unwrap().to_string().as_bytes())?;

            match output {
                Some(output) => Ok(output),
                None => Ok(vec![]),
            }
        }
        Token::Appender('1') => {
            append_to_file(&path, &output.unwrap())?;

            match error {
                Some(err) => Err(err),
                None => Ok(vec![]),
            }
        }
        Token::Appender('2') => {
            append_to_file(&path, &error.unwrap().as_bytes())?;

            match output {
                Some(output) => Ok(output),
                None => Ok(vec![]),
            }
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "error: invalid redirection",
            ))
        }
    }
}

fn append_to_file(path: &str, content: &[u8]) -> Result<(), Error> {
    let mut contents = fs::read(&path)?;

    if !contents.is_empty() {
        contents.extend_from_slice(&[10]);
    }

    contents.extend_from_slice(content);
    fs::write(path, contents)?;

    Ok(())
}
