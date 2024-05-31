#[allow(unused_imports)]
use std::io::{ self, Write };

mod commands;
mod helpers {
    pub mod path;
}
fn main() {
    let commands = commands::get_commands();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        match stdin.read_line(&mut input) {
            Ok(_) => {
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
                        print!("{}: command not found", input_array[0]);
                    }
                }
                print!("\n");
            }
            Err(error) => println!("error: {}", error),
        }

        input.clear();
    }
}
