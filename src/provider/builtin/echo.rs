use std::io::{Error, ErrorKind};

use crate::{shell::core::ShellCommand, tokenizer::Token};
pub struct Echo {}

impl ShellCommand<Token> for Echo {
    fn run(tokens: &[Token]) -> Result<String, std::io::Error> {
        if tokens.len() < 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No string input found.",
            ));
        }

        let mut string = String::new();

        for (i, token) in tokens.iter().skip(2).enumerate() {
            match token {
                Token::Space => {
                    if i > 0 {
                        string.push(' ');
                    }
                }
                Token::Value(cmd) => string.push_str(cmd.as_str()),
                Token::String(str, _) => string.push_str(str.as_str()),
                Token::Appender(_) => unimplemented!(),
                Token::Redirector(_) => unimplemented!(),
                _ => {}
            }
        }

        return Ok(string);
    }
}
