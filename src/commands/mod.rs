use std::{ collections::HashMap, env };
use crate::helpers::path;

type ICommand = dyn Fn(Vec<String>);

const SUPPORTED_COMMANDS: [&str; 4] = ["echo", "type", "exit", "pwd"];

pub fn get_commands() -> HashMap<&'static str, &'static ICommand> {
    let mut map: HashMap<&str, &ICommand> = HashMap::new();
    map.insert("type", &type_);
    map.insert("echo", &echo);
    map.insert("exit", &exit);
    map.insert("pwd", &pwd);
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
}

pub fn exit(inputs: Vec<String>) {
    if inputs[1] == "0" {
        std::process::exit(0);
    } else {
        print!("{}: command not found", inputs[0]);
    }
}

pub fn pwd(_inputs: Vec<String>) {
    match env::current_dir() {
        Ok(path) => print!("{}", path.to_str().unwrap()),
        Err(_) => print!("failed to get current directory"),
    }
}
