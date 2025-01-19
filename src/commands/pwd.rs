use std::env;

use crate::shell::{Command, Token};

pub struct Pwd {}

impl Command for Pwd {
    fn cmd(&self, _: &Vec<Token>) -> Result<String, std::io::Error> {
        match env::current_dir() {
            Ok(path) => Ok(format!("{}\n", path.to_str().unwrap())),
            Err(err) => Err(err),
        }
    }
}
