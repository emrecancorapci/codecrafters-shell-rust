use std::env;

use shell_starter_rust::tokenizer::Token;

use crate::command::Command;

pub struct Pwd {}

impl Command for Pwd {
    fn run(&self, _: &Vec<Token>) -> Result<String, std::io::Error> {
        match env::current_dir() {
            Ok(path) => Ok(format!("{}", path.to_str().unwrap())),
            Err(err) => Err(err),
        }
    }
}
