use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::WsError;

pub async fn send_message<T>(outgoing: &mut T, message: String) -> Result<(), WsError>
where
    T: SinkExt<Message> + Unpin,
{
    let _ = outgoing.send(Message::Text(message.into())).await;
    Ok(())
}

// pub async fn read_message(incoming: &mut WsStream) -> Result<Message, WsError> {
//     incoming
//         .next()
//         .await
//         .ok_or_else(|| WsError::Protocol(ProtocolError::HandshakeIncomplete))?
// }
