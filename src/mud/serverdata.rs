//use game::player::Player;
use mud::client::Client;
use mud::updatedata::UpdateData;
use mud::database::Database;

pub struct ServerData {
    pub clients: Vec<Client>,
    pub database: Database,
}

impl ServerData {
    pub fn new() -> ServerData {
        ServerData {
            clients: Vec::new(),
            database: Database::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn update_clients(&mut self) -> Vec<UpdateData> {
        let mut data = Vec::new();

        for id in 0..self.clients.len() {
            let mut client_data = self.clients[id].update();

            if client_data.command.len() != 0 {
                data.push(client_data);
            }
        }

        data
    }

    pub fn send(&mut self, id: usize, text: String) {
        self.clients[id].send(text);
    }
    /* TODO
    pub fn get_players(&mut self) -> &mut Vec<Player> {
        let mut players = Vec::new();

        for client in self.clients.iter() {
            players.push(client.player);
        }

        &mut players
    }*/
}