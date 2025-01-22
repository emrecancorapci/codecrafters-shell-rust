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

        if !(self.tokenizer.is_redirect() || self.tokenizer.is_append()) {
            let response = cmd.cmd(tokens)?;
            return Ok(response.as_bytes().to_vec());
        } else {
            match cmd.cmd(tokens) {
                Ok(response) => {
                    self.redirect(response.as_bytes())?;
                    return Ok(vec![]);
                }
                Err(err) => {
                    self.redirect(err.to_string().as_bytes())?;
                    return Ok(vec![]);
                }
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

        if output.status.success() {
            let mut output_array = output.stdout.to_vec();

            if output_array.last() == Some(&10) {
                output_array.pop();
            }

            if self.tokenizer.is_redirect() || self.tokenizer.is_append() {
                self.redirect(&output_array)?;

                return Ok(vec![]);
            }

            return Ok(output_array);
        } else {
            let mut error_array = output.stderr.to_vec();
            let mut output_array = output.stdout.to_vec();

            if output_array.last() == Some(&10) {
                output_array.pop();
            }

            if error_array.last() == Some(&10) {
                error_array.pop();
            }

            if self.tokenizer.is_redirect() || self.tokenizer.is_append() {
                self.redirect(&output_array)?;
            }

            return Err(Error::new(
                ErrorKind::InvalidInput,
                String::from_utf8(error_array).unwrap(),
            ));
        }
    }

    fn redirect(&self, contents: &[u8]) -> Result<(), Error> {
        let path = self
            .tokenizer
            .get_redirection_tokens()
            .get(1)
            .unwrap()
            .get_value();

        if self.tokenizer.is_redirect() {
            fs::write(path, contents)?;
            return Ok(());
        } else if self.tokenizer.is_append() {
            if Path::new(&path).exists() {
                let mut file_content = fs::read(&path)?;

                file_content.extend_from_slice(&contents[..]);

                fs::write(path, file_content)?;

                return Ok(());
            } else {
                return Err(Error::new(ErrorKind::NotFound, "bash: file not found"));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "bash: redirect used without proper checking",
            ));
        }
    }
}
