use std::{borrow::BorrowMut, collections::HashMap};

use crate::models::client::Client;

use super::{message::Message, message_queue::MessageQueue};

pub struct Realm {
    clients: HashMap<String, Client>,
    message_queues: HashMap<String, MessageQueue>,
    // TODO: Possibly two mutexes here
}

impl Realm {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            message_queues: HashMap::new(),
        }
    }

    pub fn get_client_ids(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    pub fn get_client_by_id(&self, id: &str) -> Option<&Client> {
        match self.clients.get(id) {
            Some(client) => Some(client),
            None => None,
        }
    }
    pub fn get_clients_ids_with_queue(&self) -> Vec<String> {
        self.message_queues.keys().cloned().collect()
    }

    pub fn set_client(&mut self, client: Client, id: String) {
        self.clients.insert(id, client);
    }

    pub fn remove_client_by_id(&mut self, id: &str) -> bool {
        if let Some(client) = self.clients.remove(id) {
            return true;
        }
        return false;
    }

    pub fn get_message_queue_by_id(&self, id: &str) -> Option<&MessageQueue> {
        self.message_queues.get(id)
    }

    pub fn get_message_queue_by_id_mut(&mut self, id: &str) -> Option<&mut MessageQueue> {
        self.message_queues.get_mut(id)
    }

    pub fn add_message_to_queue(&mut self, id: String, message: Message) {
        if let Some(ref mut queue) = self.get_message_queue_by_id_mut(&id) {
            queue.add_message(message);
        }
        self.message_queues.insert(id, MessageQueue::new());
    }

    pub fn clear_message_queue(&mut self, id: String) {
        self.message_queues.remove(&id);
    }

    pub fn generate_client_id() {

    }
}
