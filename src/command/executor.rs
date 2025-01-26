use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use shell_starter_rust::tokenizer::Token;

use super::builtin::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, type_::Type};
use crate::{command::Command, input_handler::run_command::RunCommand};

pub const SUPPORTED_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

pub struct Executor {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Executor {
    pub fn new() -> Executor {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        commands.insert("echo".to_string(), Box::new(Echo {}));
        commands.insert("type".to_string(), Box::new(Type {}));
        commands.insert("exit".to_string(), Box::new(Exit {}));
        commands.insert("pwd".to_string(), Box::new(Pwd {}));
        commands.insert("cd".to_string(), Box::new(Cd {}));

        Executor { commands }
    }
}

impl RunCommand for Executor {
    fn run(&self, cmd: &str, tokens: &Vec<Token>) -> Result<String, Error> {
        match self.commands.get(cmd) {
            Some(cmd) => cmd.run(tokens),
            None => Err(Error::new(ErrorKind::NotFound, "Command not found")),
        }
    }
}
