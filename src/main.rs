#[allow(unused_imports)]
use std::io::{ self, Write };

fn main() {
    print!("$ ");

    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).is_ok() {
        run_command(input.trim());

        // Cleaning
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}

fn run_command(command: &str) {
    match command {
        "cd" => println!("cd command found"),
        "exit" => println!("exit command found"),
        "help" => println!("help command found"),
        "ls" => println!("ls command found"),
        "pwd" => println!("pwd command found"),
        _ => println!("{}: command not found", command),
    }
}