use std::{
    io::{self, Error, ErrorKind, Write},
    process::Command,
};

use shell_starter_rust::{
    commands::{self, CommandMap},
    tokenizer::{path::get_exec_path, Token, Tokenizer},
};

fn main() {
    let mut input_handler = InputHandler::new();

    let stdin = io::stdin();
    let mut input = String::new();
    print!("$ ");

    loop {
        io::stdout().flush().unwrap();

        if let Err(err) = stdin.read_line(&mut input) {
            eprint!("error: {}\n$ ", err);
        } else if input.is_empty() {
            print!("\n$ ");
        } else {
            match input_handler.handle_input(&input) {
                Ok(output) if output.is_empty() => print!("$ "),
                Ok(output) => print!("{}\n$ ", output),
                Err(err) => eprint!("{}\n$ ", err),
            }
        }

        input.clear();
    }
}

struct InputHandler {
    tokenizer: Tokenizer,
    commands: CommandMap,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            tokenizer: Tokenizer::new(),
            commands: commands::get_commands(),
        }
    }

    fn handle_input(&mut self, input: &String) -> Result<String, Error> {
    let input = input.trim().to_string();

        self.tokenizer.parse(input)?;

        let tokens = self.tokenizer.get_tokens_ref();

    match tokens.first() {
        Some(Token::Value(input_cmd) | Token::String(input_cmd, _)) => {
                let cmd = self.commands.get(input_cmd);

            if cmd.is_none() {
                get_exec_path(input_cmd.as_str())?;

                let input_array = tokens
                    .iter()
                    .skip(2)
                    .filter(|i| !matches!(i, Token::Space))
                    .map(|i| i.get_value());

                Command::new(input_cmd).args(input_array).status()?;

                return Ok(String::new());
            }

            return cmd.unwrap().as_ref().cmd(tokens);
        }
        Some(_) => return Err(Error::new(ErrorKind::InvalidInput, "error: invalid input")),
        None => return Ok(String::new()),
        }
    }
}
