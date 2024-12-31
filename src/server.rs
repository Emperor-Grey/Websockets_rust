use crate::{config::ServerConfig, handlers::connect_handler};
use tokio::net::TcpListener;

pub struct WebSocketServer {
    config: ServerConfig,
}

impl WebSocketServer {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.config.addr).await?;
        println!("Listening on: {}", self.config.addr);

        while let Ok((socket, _addr)) = listener.accept().await {
            if let Err(e) = connect_handler::handle_connection(socket).await {
                eprintln!("Error handling connection: {:#?}", e);
            }
        }

        Ok(())
    }
}
