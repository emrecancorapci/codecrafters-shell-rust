use std::io::{self, Write};

use input_handler::InputHandler;

pub mod command_handler;
pub mod commands;
mod input_handler;

async fn main() {
    let command_handler = CommandHandler::new();
    let mut input_handler: InputHandler = InputHandler::new(command_handler);

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut input = String::new();

    print!("$ ");

    loop {
        stdout.flush().unwrap();

        if let Err(err) = stdin.read_line(&mut input) {
            eprint!("error: {}\n$ ", err);
        } else if input.is_empty() {
            print!("\n$ ");
        } else {
            let _ = match input_handler.handle_input(&input) {
                Ok(output) if output.is_empty() => stdout.write_all(b"$ "),
                Ok(output) => {
                    let _ = stdout.write_all(&output[..]);
                    stdout.write_all(b"\n$ ")
                }
                Err(err) => {
                    let _ = stderr.write_all(err.to_string().as_bytes());
                    stdout.write_all(b"\n$ ")
                }
            };
        }

        input_handler.clear();
        input.clear();
    }
}
