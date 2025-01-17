use crate::{helpers::path, input_parser::Input};
use std::{collections::HashMap, env};

pub type ICommand = dyn for<'a> Fn(&'a Vec<Input>);

const SUPPORTED_COMMANDS: [&str; 5] = ["echo", "type", "exit", "pwd", "cd"];

pub fn get_commands() -> HashMap<String, Box<ICommand>> {
    let mut map: HashMap<String, Box<ICommand>> = HashMap::new();
    map.insert("echo".to_string(), Box::new(echo));
    map.insert("type".to_string(), Box::new(type_));
    map.insert("exit".to_string(), Box::new(exit));
    map.insert("pwd".to_string(), Box::new(pwd));
    map.insert("cd".to_string(), Box::new(cd));
    map
}

pub fn echo(inputs: &Vec<Input>) {
    if let Some(Input::String(value, _)) = inputs.iter().find(|i| matches!(i, Input::String(_, _)))
    {
        println!("{}", value);
    } else if let Some(Input::Command(value)) = inputs.get(1) {
        println!("{}", value);
    } else {
        eprintln!("No string input found.");
    }
}

pub fn type_(inputs: &Vec<Input>) {
    for command in SUPPORTED_COMMANDS.iter() {
        if inputs[1] == Input::Command(command.to_string()) {
            print!("{} is a shell builtin\n", command);
            return;
        }
    }

    if let Some(Input::Command(input)) = inputs.get(1) {
        match path::get_exec_path_string(input.as_str()) {
            Ok(path) => print!("{} is {}\n", inputs[1], path),
            Err(_) => print!("{} not found\n", inputs[1]),
        }
    }
}

pub fn exit(inputs: &Vec<Input>) {
    if inputs[1] == Input::Command("0".to_string()) {
        std::process::exit(0);
    } else {
        print!("{}: command not found\n", inputs[0]);
    }
}

pub fn pwd(_: &Vec<Input>) {
    match env::current_dir() {
        Ok(path) => print!("{}\n", path.to_str().unwrap()),
        Err(_) => print!("failed to get current directory\n"),
    }
}

pub fn cd(inputs: &Vec<Input>) {
    if inputs.len() < 2 {
        print!("cd: missing argument\n");
        return;
    }
    match inputs.get(1) {
        Some(Input::Command(cmd)) => {
            match cmd.as_str() {
                "~" => {
                    // Doesn't work on windows. There is a crate called homedir that can be used to get the home directory.
                    env::set_current_dir(env::var("HOME").unwrap()).unwrap();
                }
                path => match env::set_current_dir(&path) {
                    Ok(_) => {}
                    Err(_) => print!("{}: No such file or directory\n", &path),
                },
            }
        }
        _ => println!("Invalid input"),
    }
}
