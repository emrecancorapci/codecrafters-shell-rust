use std::{
    io::{self, Write},
    process::Command,
};

use shell_starter_rust::{
    commands::{self, CommandMap},
    shell::{path::get_exec_path, Token, Tokenizer},
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
            Err(error) => eprintln!("error: {}", error),
        }

        input.clear();
    }
}

fn handle_input(input: &String, commands: &CommandMap) {
    if input.is_empty() {
        return println!("");
    }

    let mut tokenizer = Tokenizer::new();
    let input = input.trim().to_string();

    let token_result = tokenizer.parse(input);

    if token_result.is_err() {
        eprintln!("{}", token_result.unwrap_err());
        return;
    }

    let tokens = token_result.unwrap();
    let command_token = tokens.first();

    if command_token.is_none() {
        println!("");
        return;
    }

    match command_token.unwrap() {
        Token::Command(input_cmd) => {
            let cmd = commands.get(input_cmd);

            if cmd.is_none() {
                let path = get_exec_path(input_cmd.as_str());

                if path.is_err() {
                    println!("{}: command not found", input_cmd);
                    return;
                }

                let input_array = tokens
                    .iter()
                    .skip(1)
                    .map(|i| i.get_value())
                    .collect::<Vec<String>>();

                if Command::new(path.unwrap())
                    .args(input_array.iter())
                    .status()
                    .is_err()
                {
                    eprintln!("exec not found")
                }
            }

            match cmd.unwrap().as_ref().cmd(tokens) {
                Ok(response) => println!("{}", response),
                Err(err) => eprintln!("{}", err),
            }
        }
        _ => {}
    }
}
