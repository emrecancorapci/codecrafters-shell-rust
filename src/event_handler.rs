use std::io::{self, Error, Stderr, Stdout};

use crossterm::execute;

use crate::input_handler::InputHandler;

pub mod run;

pub struct EventHandler {
    buffer: String,
    input_handler: InputHandler,
    stdout: Stdout,
    stderr: Stderr,
}

impl EventHandler {
    pub fn new(input_handler: InputHandler) -> EventHandler {
        EventHandler {
            buffer: String::new(),
            input_handler,
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        execute!(self.stdout)?;
        Ok(())
    }
}
