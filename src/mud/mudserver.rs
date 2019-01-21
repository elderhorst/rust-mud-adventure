use std::net::TcpStream;

use mud::client::Client;
use game::game::Game;

pub struct MudServer {
    pub next_id: usize,
    pub game: Game,
}

impl MudServer {
    pub fn new() -> MudServer {
        MudServer {
            next_id: 0,
            game: Game::new(),
        }
    }

    pub fn add_client(&mut self, stream: TcpStream) -> usize {
        let client = Client::new(self.next_id, stream);
        self.game.add_client(self.next_id, client);

        self.next_id += 1;
        self.next_id.clone()
    }

    pub fn send_message(&mut self, id: usize, text: String) {
        self.game.send(id, text);
    }

    pub fn update(&mut self) {
        let client_data = self.game.update_clients();

        for data in &client_data {
            self.game.handle_command(&data.id, &data.command, &data.params);
        }
    }
}