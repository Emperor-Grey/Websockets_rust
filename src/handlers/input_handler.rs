use tokio::io::{stdin, AsyncBufReadExt, BufReader};

pub struct InputHandler {
    reader: tokio::io::Lines<BufReader<tokio::io::Stdin>>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(stdin()).lines(),
        }
    }

    pub async fn next_line(&mut self) -> Option<String> {
        self.reader.next_line().await.ok().flatten()
    }
}
