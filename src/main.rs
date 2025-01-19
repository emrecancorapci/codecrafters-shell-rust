use std::{
    io::{self, Error, Write},
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
    print!("$ ");

    loop {
        io::stdout().flush().unwrap();

        if let Err(err) = stdin.read_line(&mut input) {
            eprint!("error: {}", err);
        } else if input.is_empty() {
            print!("\n$ ");
        } else {
            match handle_input(&input, &commands) {
                Ok(output) => print!("{}\n$ ", output),
                Err(err) => eprint!("{}\n$ ", err),
            }
        }

        input.clear();
    }
}

fn handle_input(input: &String, commands: &CommandMap) -> Result<String, Error> {
    let mut tokenizer = Tokenizer::new();
    let input = input.trim().to_string();

    let tokens = tokenizer.parse(input)?;

    match tokens.first() {
        Some(Token::Command(input_cmd)) => {
            let cmd = commands.get(input_cmd);

            if cmd.is_none() {
                let path = get_exec_path(input_cmd.as_str())?;

                let input_array = tokens
                    .iter()
                    .skip(1)
                    .map(|i| i.get_value())
                    .collect::<Vec<String>>();

                Command::new(path).args(input_array.iter()).status()?;

                return Ok(String::new());
            }

            return cmd.unwrap().as_ref().cmd(tokens);
        }
        Some(_) => return Err(Error::new(io::ErrorKind::InvalidInput, "error: invalid input")),
        None => return Ok(String::new()),
    }
}
