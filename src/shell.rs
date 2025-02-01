use std::io::{self, Error, ErrorKind, Stderr, Stdout, Write};

use core::{ShellCommandProvider, ShellInterpreter, ShellTokenizer};
use crossterm::{
    cursor::{self, MoveRight, MoveToColumn},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

const PREFIX: &str = "$ ";

pub mod core;

pub struct Shell {
    buffer: String,
    stdout: Stdout,
    stderr: Stderr,
    // history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            stdout: io::stdout(),
            stderr: io::stderr(),
            // history: Vec::new(),
        }
    }

    pub async fn run<
        T,
        SI: ShellInterpreter<T>,
        ST: ShellTokenizer<T>,
        SCC: ShellCommandProvider<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        self.init()?;

        loop {
            self.stdout.flush()?;

            let result = self.shell_loop::<T, SI, ST, SCC>();

            if result.is_err() && result.unwrap_err().kind() == ErrorKind::Interrupted {
                break;
            }
        }

        self.uninit()?;

        Ok(())
    }

    fn shell_loop<
        T,
        SI: ShellInterpreter<T>,
        ST: ShellTokenizer<T>,
        SCC: ShellCommandProvider<T>,
    >(
        &mut self,
    ) -> Result<(), Error> {
        match event::read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key_event) => {
                self.handle_modifiers(key_event)?;
                self.handle_keys::<T, SI, ST, SCC>(key_event)?;

                Ok(())
            }
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }

    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;

        execute!(self.stdout, Print("$ "),)?;
        Ok(())
    }

    fn uninit(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;

        execute!(self.stdout)?;
        Ok(())
    }

    fn handle_keys<
        T,
        Interpreter: ShellInterpreter<T>,
        Tokenizer: ShellTokenizer<T>,
        CommandProvider: ShellCommandProvider<T>,
    >(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<(), Error> {
        let KeyEvent { code, .. } = key_event;

        match code {
            KeyCode::Char(ch) => {
                let relative_cursor_x = cursor::position()?.0 as usize - PREFIX.len();

                if relative_cursor_x < self.buffer.len() {
                    self.buffer.insert(relative_cursor_x, ch);

                    execute!(
                        self.stdout,
                        Clear(ClearType::CurrentLine),
                        MoveToColumn(0),
                        Print(PREFIX),
                        Print(&self.buffer),
                        MoveToColumn((relative_cursor_x + 3) as u16)
                    )?;
                } else {
                    self.buffer.push(ch);
                    execute!(self.stdout, Print(ch))?;
                }
            }
            KeyCode::Enter => {
                if !self.buffer.is_empty() {
                    execute!(self.stdout, Print("\r\n"))?;

                    match Tokenizer::tokenize(self.buffer.trim())? {
                        tokens => match Interpreter::run::<CommandProvider>(&tokens) {
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
                }
                execute!(self.stdout, Print("\r\n"), Print(PREFIX))?;

                self.buffer.clear();
            }
            KeyCode::Tab => {
                match CommandProvider::get_commands()
                    .iter()
                    .find(|c| c.starts_with(self.buffer.trim()))
                {
                    Some(found_command) => {
                        let rest_of_the_command = &found_command[self.buffer.trim().len()..];

                        self.stdout.write(rest_of_the_command.as_bytes())?;
                        self.buffer.push_str(rest_of_the_command);
                    }
                    None => {
                        self.stdout.write(&[7])?;
                    }
                }
            }
            KeyCode::Backspace => {
                self.buffer.pop();

                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(PREFIX),
                    Print(&self.buffer)
                )?;
            }
            KeyCode::Left => {
                execute!(self.stdout, cursor::MoveLeft(1))?;
            }
            KeyCode::Right => {
                let relative_cursor_x = cursor::position()?.0 as usize - PREFIX.len();

                if relative_cursor_x < self.buffer.len() {
                    execute!(self.stdout, MoveRight(1))?;
                }
            }
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
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

    fn handle_modifiers(&self, key_event: KeyEvent) -> Result<(), Error> {
        let KeyEvent {
            code, modifiers, ..
        } = key_event;

        if modifiers.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
            return Err(io::Error::new(ErrorKind::Interrupted, "ctrl-c"));
        }

        Ok(())
    }
}
