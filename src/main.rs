#[allow(unused_imports)]
use std::io::{ self, Write };

mod commands {
    pub mod echo;
}

fn main() {
    print!("$ ");

    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).is_ok() {
        let input_array: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match input_array[0].as_str() {
            "exit" => {
                if input_array[1] == "0" {
                    break;
                }
            }
            _ => {}
        }

        run_command(input_array);

        // Cleaning
        input.clear();
        print!("\n$ ");
        io::stdout().flush().unwrap();
    }
}

fn run_command(command: Vec<String>) {
    match command[0].as_str() {
        "cd" => println!("cd command found"),
        "echo" => commands::echo::echo(command),
        "exit" => println!("exit command found"),
        "help" => println!("help command found"),
        "ls" => println!("ls command found"),
        "pwd" => println!("pwd command found"),
        _ => println!("{}: command not found", command[0]),
    }
}
