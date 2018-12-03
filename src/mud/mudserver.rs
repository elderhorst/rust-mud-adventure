use std::net::TcpStream;

use commands::commands;
use mud::client::Client;
use mud::serverdata::ServerData;
use game::game::Game;

pub struct MudServer {
    pub next_id: usize,
    pub game: Game,
    pub server_data: ServerData,
}

impl MudServer {
    pub fn new() -> MudServer {
        MudServer {
            next_id: 0,
            game: Game::new(),
            server_data: ServerData::new(),
        }
    }

    pub fn add_client(&mut self, stream: TcpStream) -> usize {
        let client = Client::new(self.next_id, stream);
        self.server_data.add_client(client);

        self.next_id += 1;
        self.next_id.clone()
    }

    pub fn send_message(&mut self, id: usize, text: String) {
        self.server_data.clients[id].send(text);
    }

    pub fn update(&mut self) {
        let client_data = self.server_data.update_clients();

        for data in &client_data {
            commands::handle_command(&data.id, &data.command, &data.params, &mut self.server_data, &mut self.game);
        }
    }
}