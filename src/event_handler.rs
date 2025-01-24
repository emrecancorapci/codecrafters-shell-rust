use std::io::{self, ErrorKind, Stderr, Write};
use std::io::{Error, Stdout};

use crossterm::event::{self, Event, KeyEvent};

use crate::input_handler::InputHandler;

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

    pub fn stdout_as_ref(&self) -> &Stdout {
        &self.stdout
    }

    pub async fn run(&mut self) -> Result<String, Error> {
        print!("$ ");

        loop {
            self.stdout.flush()?;

            match event::read()? {
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Key(key_event) => {
                    let KeyEvent { code, .. } = key_event;

                    match code {
                        event::KeyCode::Char(ch) => {
                            self.buffer.push(ch);
                        }
                        event::KeyCode::Backspace => {}
                        event::KeyCode::Enter => {
                            let response = self.input_handler.handle_input(&self.buffer);
                            self.buffer.clear();

                            match response {
                                Ok(ok) => {
                                    self.stdout.write(&ok)?;
                                }
                                Err(err) => {
                                    if err.kind() == ErrorKind::Interrupted {
                                        break;
                                    }
                                    self.stderr.write(&err.to_string().as_bytes())?;
                                }
                            }

                            self.stdout.write(b"\n$ ")?;
                            self.input_handler.clear();
                        }
                        event::KeyCode::Left => {}
                        event::KeyCode::Right => {}
                        event::KeyCode::Up => todo!(),
                        event::KeyCode::Down => todo!(),
                        event::KeyCode::Home => todo!(),
                        event::KeyCode::End => todo!(),
                        event::KeyCode::PageUp => todo!(),
                        event::KeyCode::PageDown => todo!(),
                        event::KeyCode::Tab => todo!(),
                        event::KeyCode::BackTab => todo!(),
                        event::KeyCode::Delete => todo!(),
                        event::KeyCode::Insert => todo!(),
                        event::KeyCode::F(_) => todo!(),
                        event::KeyCode::Null => todo!(),
                        event::KeyCode::Esc => todo!(),
                        event::KeyCode::CapsLock => todo!(),
                        event::KeyCode::ScrollLock => todo!(),
                        event::KeyCode::NumLock => todo!(),
                        event::KeyCode::PrintScreen => todo!(),
                        event::KeyCode::Pause => todo!(),
                        event::KeyCode::Menu => todo!(),
                        event::KeyCode::KeypadBegin => todo!(),
                        event::KeyCode::Media(_) => todo!(),
                        event::KeyCode::Modifier(_) => todo!(),
                    }
                }
                Event::Mouse(_) => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(_, _) => todo!(),
            }
        }

        return Ok(String::new());
    }
}
