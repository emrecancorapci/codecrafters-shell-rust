use std::io::{self, Error, ErrorKind, Stderr, Stdout, Write};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
};

pub mod core;

pub struct Shell {
    buffer: String,
    stdout: Stdout,
    stderr: Stderr,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }

    pub async fn run<T, SI: ShellInterpreter<T>, ST: ShellTokenizer<T>, SCC: ShellCommandProvider<T>>(
        &mut self,
    ) -> Result<String, Error> {
        self.init()?;

        print!("$ ");

        loop {
            self.stdout.flush()?;

            match match event::read()? {
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Key(key_event) => self.handle_keys::<T, SI, ST, SCC>(key_event),
                Event::Mouse(_) => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(_, _) => todo!(),
            } {
                Ok(_) => {}
                Err(err) => {
                    if err.kind() == ErrorKind::Interrupted {
                        return Err(err);
                    }
                }
            }
        }
    }

    fn init(&mut self) -> Result<(), Error> {
        execute!(self.stdout)?;
        Ok(())
    }

    fn handle_keys<T, SI: ShellInterpreter<T>, ST: ShellTokenizer<T>, SCC: ShellCommandProvider<T>>(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<(), Error> {
        let KeyEvent { code, .. } = key_event;

        match code {
            KeyCode::Char(ch) => {
                self.buffer.push(ch);
            }
            KeyCode::Backspace => {}
            KeyCode::Enter => {
                match ST::tokenize(self.buffer.trim())? {
                    tokens if tokens.is_empty() => {
                        self.stdout.write(b"")?;
                    }
                    tokens => match SI::run::<SCC>(&tokens) {
                        Ok(ok) => {
                            self.stdout.write(&ok)?;
                        }
                        Err(err) => {
                            if err.kind() == ErrorKind::Interrupted {
                                return Err(err);
                            }
                            self.stderr.write(&err.to_string().as_bytes())?;
                        }
                    },
                }

                self.stdout.write(b"\n$ ")?;
                self.buffer.clear();
            }
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            KeyCode::CapsLock => todo!(),
            KeyCode::ScrollLock => todo!(),
            KeyCode::NumLock => todo!(),
            KeyCode::PrintScreen => todo!(),
            KeyCode::Pause => todo!(),
            KeyCode::Menu => todo!(),
            KeyCode::KeypadBegin => todo!(),
            KeyCode::Media(_) => todo!(),
            KeyCode::Modifier(_) => todo!(),
        }

        Ok(())
    }
}
