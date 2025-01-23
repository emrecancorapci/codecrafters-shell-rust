use std::collections::HashMap;

use shell_starter_rust::tokenizer::{Command, Token};

use crate::{
    commands::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, type_::Type},
    input_handler::HandleCommand,
};

pub const SUPPORTED_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

pub struct CommandHandler {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert("echo".to_string(), Box::new(Echo {}));
        commands.insert("type".to_string(), Box::new(Type {}));
        commands.insert("exit".to_string(), Box::new(Exit {}));
        commands.insert("pwd".to_string(), Box::new(Pwd {}));
        commands.insert("cd".to_string(), Box::new(Cd {}));

        CommandHandler { commands }
    }
}

impl HandleCommand for CommandHandler {
    fn run(&self, cmd: &str, tokens: &Vec<Token>) -> Result<String, std::io::Error> {
        match self.commands.get(cmd) {
            Some(cmd) => cmd.run(tokens),
            None => todo!(),
        }
    }

    fn is_exist(&self, cmd: &str) -> bool {
        self.commands.contains_key(cmd)
    }
}
