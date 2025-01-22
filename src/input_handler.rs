use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use shell_starter_rust::tokenizer::{path::get_exec_path, Command, Token, Tokenizer};

use crate::commands::{self, CommandMap};

pub struct InputHandler {
    tokenizer: Tokenizer,
    commands: CommandMap,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            tokenizer: Tokenizer::new(),
            commands: commands::get_commands(),
        }
    }

    pub fn clear(&mut self) {
        self.tokenizer.clear()
    }

    pub fn handle_input(&mut self, input: &String) -> Result<Vec<u8>, Error> {
        let input = input.trim().to_string();

        self.tokenizer.parse(input)?;

        match self.tokenizer.get_tokens_ref().first() {
            Some(Token::Value(input_cmd) | Token::String(input_cmd, _)) => {
                let cmd = self.commands.get(input_cmd);

                if cmd.is_some() {
                    return self.handle_builtin_output(cmd.unwrap().as_ref());
                } else {
                    return self.handle_external_output(input_cmd);
                }
            }
            Some(_) => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
            None => return Ok(vec![]),
        }
    }

    fn handle_builtin_output(&self, cmd: &dyn Command) -> Result<Vec<u8>, Error> {
        let tokens = self.tokenizer.get_tokens_ref();

        match cmd.cmd(tokens) {
            Ok(response) if self.tokenizer.is_append_ok() || self.tokenizer.is_redirect_ok() => {
                self.redirect(response.as_bytes())?;
                return Ok(vec![]);
            }
            Ok(response) if self.tokenizer.is_append_err() || self.tokenizer.is_redirect_err() => {
                return Ok(response.as_bytes().to_vec());
            }
            Ok(response) => {
                return Ok(response.as_bytes().to_vec());
            }
            Err(err) if self.tokenizer.is_append_err() || self.tokenizer.is_redirect_err() => {
                self.redirect(err.to_string().as_bytes())?;
                return Ok(vec![]);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn handle_external_output(&self, cmd: &String) -> Result<Vec<u8>, Error> {
        get_exec_path(cmd.as_str())?;

        let input_array = self
            .tokenizer
            .get_tokens_ref()
            .iter()
            .skip(2)
            .filter(|i| !matches!(i, Token::Space))
            .map(|i| i.get_value());

        let output = std::process::Command::new(cmd).args(input_array).output()?;

        let mut error_array = output.stderr.to_vec();
        let mut output_array = output.stdout.to_vec();

        if output_array.last() == Some(&10) {
            output_array.pop();
        }

        if error_array.last() == Some(&10) {
            error_array.pop();
        }

        if self.tokenizer.is_redirect_ok() || self.tokenizer.is_append_ok() {
            self.redirect(&output_array)?;

            if !error_array.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    String::from_utf8(error_array).unwrap(),
                ));
            }

            return Ok(vec![]);
        } else if self.tokenizer.is_redirect_err() || self.tokenizer.is_append_err() {
            self.redirect(&error_array)?;

            if !output_array.is_empty() {
                return Ok(output_array);
            }

            return Ok(vec![]);
        }

        return Ok(output_array);
    }

    fn redirect(&self, contents: &[u8]) -> Result<(), Error> {
        let path = self
            .tokenizer
            .get_redirection_tokens()
            .get(1)
            .unwrap()
            .get_value();

        let is_path_exist = Path::new(&path).exists();

        match (self.tokenizer.is_redirect(), self.tokenizer.is_append()) {
            (true, false) => {
                fs::write(path, contents)?;
            }
            (false, true) if is_path_exist => {
                let mut file_content = fs::read(&path)?;
                file_content.extend_from_slice(&contents[..]);

                fs::write(path, file_content)?;
            }
            (false, true) => {
                return Err(Error::new(ErrorKind::NotFound, "bash: file not found"));
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "bash: redirect used without proper checking",
                ));
            }
        }

        return Ok(());
    }
}
