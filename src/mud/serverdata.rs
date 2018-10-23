use mud::client::Client;
use mud::updatedata::UpdateData;

pub struct ServerData {
    pub clients: Vec<Client>,
}

impl ServerData {
    pub fn new() -> ServerData {
        ServerData {
            clients: Vec::new(),
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
}