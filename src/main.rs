use command_handler::CommandHandler;
use event_handler::EventHandler;
use input_handler::InputHandler;

pub mod command;
pub mod command_handler;
pub mod commands;
mod event_handler;
mod input_handler;

#[tokio::main]
async fn main() {
    let command_handler = CommandHandler::new();
    let input_handler = InputHandler::new(command_handler);
    let mut event_handler = EventHandler::new(input_handler);

    let _ = event_handler.init();

    let _ = event_handler.run().await;
}
