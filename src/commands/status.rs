use std::collections::HashMap;
use mud::client::Client;

pub struct CommandStatus {
    pub handled: bool,
    ids: Vec<usize>,
    messages: Vec<String>,
}

impl CommandStatus {
    pub fn new() -> CommandStatus {
        CommandStatus {
            handled: false,
            ids: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn queue(&mut self, id: usize, text: String) {
        self.ids.push(id);
        self.messages.push(text);
    }

    pub fn send_messages(&mut self, clients: &mut HashMap<usize, Client>) {
        let mut sorted: HashMap<usize, String> = HashMap::new();

        for index in 0..self.ids.len() {
            if !sorted.contains_key(&index) {
                let text = &self.messages[index];
                sorted.insert(self.ids[index], text.to_string());
            }
            else {
                let mut combined = sorted.get_mut(&index).unwrap();
                combined.push_str(&self.messages[index]);
            }
        }

        for (id, text) in sorted {
            clients.get_mut(&id).unwrap().send(text);
        }
    }
}