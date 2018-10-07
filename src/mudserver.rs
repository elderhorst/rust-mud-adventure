use std::net::TcpStream;
use std::vec::Vec;

use client::Client;

//#[derive(Copy, Clone)]
pub struct MudServer {
    pub clients: Vec<Client>,
    pub next_id: usize
}

impl MudServer {
    pub fn new() -> MudServer {
        MudServer {
            clients: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_client(&mut self, stream: TcpStream) -> usize {
        let client = Client::new(self.next_id, stream);
        self.clients.push(client);

        self.next_id += 1;
        self.next_id.clone()
    }

    pub fn write_to_client(&mut self, id: usize, text: String) {
        self.clients[id].write(text);
    }

    pub fn update(&mut self) {
        for id in 0..self.clients.len() {
            self.clients[id].update();
        }
    }
}