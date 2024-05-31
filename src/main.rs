#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let existing_commands = vec!["cd", "exit", "help", "ls", "pwd"];
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    for cmd in existing_commands {
        if input.trim() == cmd {
            println!("{} command found", cmd);
            return;
        } else {
            print!("{}: command not found", input.trim());
            return;
        }
    }
}
