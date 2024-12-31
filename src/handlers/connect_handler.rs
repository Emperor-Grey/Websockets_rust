use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{handlers::message_handler, WsError};

use super::input_handler::InputHandler;

pub async fn handle_connection(socket: TcpStream) -> Result<(), WsError> {
    let ws_stream = accept_async(socket).await?;
    let (outgoing, incoming) = ws_stream.split();

    spawn_sender(outgoing);
    spawn_receiver(incoming);

    Ok(())
}

pub fn spawn_sender(mut outgoing: impl SinkExt<Message> + Send + Unpin + 'static) {
    tokio::spawn(async move {
        let mut input = InputHandler::new();

        while let Some(line) = input.next_line().await {
            if !line.trim().is_empty() {
                println!("Sending message: {}", line);
                if let Err(e) = message_handler::send_message(&mut outgoing, line).await {
                    println!("Error sending message: {}", e);
                    break;
                }
            }
        }
    });
}

// Unpin trait is needed because next() needs to move the stream value
pub fn spawn_receiver<S>(incoming: S)
where
    S: StreamExt<Item = Result<Message, WsError>> + Send + 'static + Unpin,
{
    tokio::spawn(async move {
        let mut pinned = incoming;
        while let Some(Ok(message)) = pinned.next().await {
            match message {
                Message::Text(text) => println!("Received text: {}", text),
                _ => {}
            }
        }
    });
}
