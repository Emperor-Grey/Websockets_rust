use config::ServerConfig;

use server::WebSocketServer;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub type WsStream = WebSocketStream<TcpStream>;
pub type WsError = tokio_tungstenite::tungstenite::Error;

mod config;
mod handlers;
mod server;

#[tokio::main]
async fn main() {
    let config = ServerConfig::default();
    let server = WebSocketServer::new(config);

    if let Err(e) = server.run().await {
        eprintln!("Server error: {}", e);
    }
}
