use command::executor::Executor;
use event_handler::EventHandler;
use input_handler::InputHandler;

pub mod command;
mod event_handler;
mod input_handler;

#[tokio::main]
async fn main() {
    let executor = Executor::new();
    let input_handler = InputHandler::new(executor);
    let mut event_handler = EventHandler::new(input_handler);

    let _ = event_handler.init();

    let _ = event_handler.run().await;
}
