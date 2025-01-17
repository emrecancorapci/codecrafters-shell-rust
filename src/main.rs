use std::{
    collections::HashMap,
    io::{self, Write},
    process::Command,
};

use shell_starter_rust::{
    commands::{self, ICommand},
    helpers::path::get_exec_path,
    input_parser::InputParser,
};

fn main() {
    let commands = commands::get_commands();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut input) {
            Ok(_) => {
                handle_input(&input, &commands);
            }
            Err(error) => println!("error: {}", error),
        }

        input.clear();
    }
}

fn handle_input(input: &String, commands: &HashMap<String, Box<ICommand>>) {
    let input_array: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let first_input = input_array[0].as_str();

    match commands.get(first_input) {
        Some(command) => match InputParser::new().parse(input) {
            Ok(parsed_input) => command(parsed_input),
            Err(err) => eprintln!("{}", err.to_string()),
        },
        None => match get_exec_path(first_input) {
            Ok(path) => {
                Command::new(path)
                    .args(input_array[1..].iter())
                    .status()
                    .expect("failed to execute process");
            }
            Err(_) => {
                println!("{}: command not found", first_input);
            }
        },
    }
}
