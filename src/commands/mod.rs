use std::collections::HashMap;
use crate::helpers::path;

type ICommand = dyn Fn(Vec<String>);

const SUPPORTED_COMMANDS: [&str; 3] = ["echo", "type", "exit"];

pub fn get_commands() -> HashMap<&'static str, &'static ICommand> {
    let mut map: HashMap<&str, &ICommand> = HashMap::new();
    map.insert("type", &type_);
    map.insert("echo", &echo);
    map.insert("exit", &exit);
    map
}

pub fn echo(inputs: Vec<String>) {
    print!("{}", inputs[1..].join(" "));
}

pub fn type_(inputs: Vec<String>) {
    for command in SUPPORTED_COMMANDS.iter() {
        if &inputs[1] == command {
            print!("{} is a shell builtin", command);
            return;
        }
    }

    match path::get_exec_path_string(&inputs[1]) {
        Ok(path) => print!("{} is {}", inputs[1], path),
        Err(_) => print!("{} not found", inputs[1]),
    }

    print!("{} not found", inputs[1]);
}

pub fn exit(inputs: Vec<String>) {
    if inputs[1] == "0" {
        std::process::exit(0);
    } else {
        print!("{}: command not found", inputs[0]);
    }
}
