use std::{
    fs,
    io::{Error, ErrorKind},
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

        if self.tokenizer.is_redirect() || self.tokenizer.is_append() {
        } else {
            if output.status.success() {
                if self.tokenizer.is_redirect() || self.tokenizer.is_append() {
                    self.redirect(&output.stdout)?;

                    return Ok(vec![]);
                }

                return Ok(output.stdout);
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    String::from_utf8(output.stderr).unwrap(),
                ));
            }
        }

        return Ok(vec![]);
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
            if fs::exists(&path)? {
                let mut content = fs::read(&path)?;

                content.extend_from_slice(contents);

                fs::write(path, content)?;

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
