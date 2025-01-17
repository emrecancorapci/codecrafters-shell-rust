use std::{
    collections::HashMap,
    io::{self, Write},
    process::Command,
};

use shell_starter_rust::{
    commands::{self, ICommand},
    helpers::path::get_exec_path,
    input_parser::{Input, InputParser},
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
    if input.is_empty() {
        return println!("");
    }

    let mut input_parser = InputParser::new();
    let input = input.trim().to_string();

    let parsed_input = input_parser.parse(input);

    if parsed_input.is_err() {
        eprintln!("{}", parsed_input.unwrap_err());
        return;
    }

    let input = parsed_input.unwrap();
    let first_input = input.first();

    if first_input.is_none() {
        println!("");
        return;
    }

    let first_input = first_input.unwrap();

    match first_input {
        Input::Command(input_cmd) => {
            let cmd = commands.get(input_cmd);

            if cmd.is_none() {
                let path = get_exec_path(input_cmd.as_str());

                if path.is_err() {
                    println!("{}: command not found", input_cmd);
                }

                let input_array = input
                    .iter()
                    .skip(1)
                    .map(|i| i.get_value())
                    .collect::<Vec<String>>();

                Command::new(path.unwrap())
                    .args(input_array.iter())
                    .status()
                    .expect("failed to execute process");
            }

            cmd.unwrap()(input);
        }
        _ => {}
    }
}
