use std::io::{Error, ErrorKind, Write};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use super::EventHandler;

impl EventHandler {
    pub async fn run(&mut self) -> Result<String, Error> {
        print!("$ ");

        loop {
            self.stdout.flush()?;

            match match event::read()? {
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Key(key_event) => self.handle_keys(key_event),
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

    fn handle_keys(&mut self, key_event: KeyEvent) -> Result<(), Error> {
        let KeyEvent { code, .. } = key_event;

        match code {
            KeyCode::Char(ch) => {
                self.buffer.push(ch);
            }
            KeyCode::Backspace => {}
            KeyCode::Enter => {
                match self.input_handler.handle_input(&self.buffer) {
                    Ok(ok) => {
                        self.stdout.write(&ok)?;
                    }
                    Err(err) => {
                        if err.kind() == ErrorKind::Interrupted {
                            return Err(err);
                        }
                        self.stderr.write(&err.to_string().as_bytes())?;
                    }
                }

                self.stdout.write(b"\n$ ")?;

                self.buffer.clear();
                self.input_handler.clear();
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
