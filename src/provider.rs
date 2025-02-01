use crate::{
    shell::core::{ShellCommand, ShellCommandProvider},
    tokenizer::Token,
};

use builtin::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, type_::Type};

pub mod builtin;

pub const SUPPORTED_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

pub struct CommandProvider {}

impl ShellCommandProvider<Token> for CommandProvider {
    fn run(cmd: &str, tokens: &[Token]) -> Result<String, std::io::Error> {
        match cmd {
            "echo" => Echo::run(tokens),
            "type" => Type::run(tokens),
            "exit" => Exit::run(tokens),
            "pwd" => Pwd::run(tokens),
            "cd" => Cd::run(tokens),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "shell: command not found",
            )),
        }
    }

    fn get_commands() -> Vec<&'static str> {
        return SUPPORTED_COMMANDS.to_vec();
    }
}
