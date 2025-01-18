use crate::{helpers::path, tokenizer::Token};
use std::{collections::HashMap, env};

type TokenVector<'a> = &'a Vec<Token>;
pub type ICommand = dyn for<'a> Fn(TokenVector<'a>);

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

pub fn echo(inputs: TokenVector) {
    if inputs.len() >= 3 {
        let mut string = String::new();
        let mut is_string: Option<bool> = None;

        inputs
            .iter()
            .skip(2)
            .enumerate()
            .for_each(|(i, t)| match t {
                Token::Space => {
                    if i > 0 {
                        match is_string {
                            Some(true) => {
                                string.push(' ');
                            }
                            Some(false) => {
                                string.push(' ');
                            }
                            None => {}
                        }
                    }
                }
                Token::Command(cmd) => match is_string {
                    Some(true) => {}
                    Some(false) => string.push_str(cmd.as_str()),
                    None => {
                        is_string = Some(false);
                        string.push_str(cmd.as_str());
                    }
                },
                Token::Argument(_, _) => {}
                Token::String(str, _) => {
                    if is_string == None {
                        is_string = Some(true);
                        string.push_str(str.as_str());
                    } else if is_string == Some(true) {
                        string.push_str(str.as_str());
                    }
                }
            });

        println!("{}", string);
    } else if inputs.len() == 3 {
        let string = inputs.get(2).unwrap().get_value();
        println!("{}", string);
    } else {
        eprintln!("No string input found.");
    }
}

pub fn type_(inputs: TokenVector) {
    for command in SUPPORTED_COMMANDS.iter() {
        if inputs[2] == Token::Command(command.to_string()) {
            print!("{} is a shell builtin\n", command);
            return;
        }
    }

    if let Some(Token::Command(input)) = inputs.get(2) {
        match path::get_exec_path_string(input.as_str()) {
            Ok(path) => print!("{} is {}\n", inputs[2], path),
            Err(_) => print!("{} not found\n", inputs[2]),
        }
    }
}

pub fn exit(inputs: TokenVector) {
    if inputs[2] == Token::Command("0".to_string()) {
        std::process::exit(0);
    } else {
        print!("{}: command not found\n", inputs[0]);
    }
}

pub fn pwd(_: TokenVector) {
    match env::current_dir() {
        Ok(path) => print!("{}\n", path.to_str().unwrap()),
        Err(_) => print!("failed to get current directory\n"),
    }
}

pub fn cd(inputs: TokenVector) {
    if inputs.len() < 3 {
        print!("cd: missing argument\n");
        return;
    }
    match inputs.get(2) {
        Some(Token::Command(cmd)) => {
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
