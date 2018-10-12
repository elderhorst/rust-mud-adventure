use std::net::TcpStream;
use std::vec::Vec;

use mud::client::Client;
use game::game::Game;

//#[derive(Copy, Clone)]
pub struct MudServer {
    pub clients: Vec<Client>,
    pub next_id: usize,
    pub game: Game,
}

impl MudServer {
    pub fn new() -> MudServer {
        MudServer {
            clients: Vec::new(),
            next_id: 0,
            game: Game::new(),
        }
    }

    pub fn add_client(&mut self, stream: TcpStream) -> usize {
        let client = Client::new(self.next_id, stream);
        self.clients.push(client);

        self.next_id += 1;
        self.next_id.clone()
    }

    pub fn send_message(&mut self, id: usize, text: String) {
        self.clients[id].send(text);
    }

    pub fn update(&mut self) {
        for id in 0..self.clients.len() {
            self.clients[id].update();
        }
    }
}