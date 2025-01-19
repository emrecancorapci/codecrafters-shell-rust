use cat::Cat;
use cd::Cd;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use type_::Type;

use std::collections::HashMap;

use crate::shell::Command;

pub mod echo;
pub mod type_;
pub mod exit;
pub mod pwd;
pub mod cd;
pub mod cat;

pub const SUPPORTED_COMMANDS: [&str; 6] = ["echo", "type", "exit", "pwd", "cd", "cat"];

pub type CommandMap = HashMap<String, Box<dyn Command>>;

pub fn get_commands() -> CommandMap {
    let mut map: CommandMap = HashMap::new();
    map.insert("echo".to_string(), Box::new(Echo {}));
    map.insert("type".to_string(), Box::new(Type {}));
    map.insert("exit".to_string(), Box::new(Exit {}));
    map.insert("pwd".to_string(), Box::new(Pwd {}));
    map.insert("cd".to_string(), Box::new(Cd {}));
    map.insert("cat".to_string(), Box::new(Cat {}));
    map
}
