use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use super::message::Message;
use crate::util::util::random_token;

pub struct MessageQueue {
    last_read_at: u128,
    messages: Vec<Message>,
    // TODO: Add later
    // lock: Mutex,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            last_read_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Getting current timestamp should succeed")
                .as_millis(),
            messages: Vec::new(),
        }
    }
    pub fn get_last_read_at(&self) -> u128 {
        self.last_read_at
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    // Read last message
    pub fn read_message(&mut self) -> Option<Message> {
        if self.messages.len() > 0 {
            self.last_read_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Getting current timestamp should succeed")
                .as_millis();
            let msg = self.messages[0].clone();
            self.messages.drain(1..);
            return Some(msg);
        }
        return None;
    }

    pub fn generate_client_id() {
        return random_token();
    }
}
