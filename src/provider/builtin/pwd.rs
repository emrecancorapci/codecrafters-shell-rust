use crate::{shell::core::ShellCommand, tokenizer::Token};

pub struct Pwd {}

impl ShellCommand<Token> for Pwd {
    fn run(_: &[Token]) -> Result<String, std::io::Error> {
        match std::env::current_dir() {
            Ok(path) => Ok(path.to_str().unwrap().to_string()),
            Err(err) => Err(err),
        }
    }
}
