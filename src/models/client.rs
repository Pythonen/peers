use std::{
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

use tungstenite::protocol::{Message, WebSocket};

struct Client {
    id: String,
    token: String,
    socket: Option<WebSocket<TcpStream>>,
    last_ping: u128,
}

impl Client {
    fn new(&self, id: String, token: String) -> Self {
        Self {
            id,
            token,
            socket: None,
            last_ping: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Getting current timestamp should succeed")
                .as_millis(),
        }
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_token(&self) -> &str {
        &self.token
    }

    fn get_socket(&self) -> &Option<WebSocket<TcpStream>> {
        &self.socket
    }

    fn set_socket(&mut self, socket: Option<WebSocket<TcpStream>>) {
        self.socket = socket;
    }

    fn get_last_ping(&self) -> u128 {
        self.last_ping
    }

    fn set_last_ping(&mut self, last_ping: u128) {
        self.last_ping = last_ping;
    }

    fn send<T: serde::ser::Serialize>(&mut self, data: T) -> Result<(), tungstenite::Error> {
        match &mut self.socket {
            Some(ref mut sock) => {
                sock.write_message(Message::Text(serde_json::to_string(&data).unwrap()))
            }
            None => todo!(),
        }
    }
}
