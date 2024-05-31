#[allow(unused_imports)]
use std::io::{ self, Write };

mod commands;
fn main() {
    print!("$ ");

    let commands = commands::get_commands();

    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).is_ok() {
        let input_array: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match commands.get(input_array[0].as_str()) {
            Some(command) => {
                command(input_array);
            }
            None => {
                println!("{}: command not found", input_array[0]);
            }
        }

        // Cleaning
        input.clear();
        print!("\n$ ");
        io::stdout().flush().unwrap();
    }
}
