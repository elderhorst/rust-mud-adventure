use std::collections::HashMap;

use game::ability::Abilities;
use game::race::{Race, RaceFactory};
use game::player::Player;
use mud::client::Client;
use mud::updatedata::UpdateData;
use mud::database::Database;
use rooms::room::Room;
use rooms::roomfactory::RoomFactory;

pub struct Game {
    pub rooms: HashMap<String, Room>,
    pub races: HashMap<String, Race>,
    pub abilities: Abilities,
    pub clients: HashMap<usize, Client>,
    pub players: HashMap<usize, Player>,
    pub database: Database,
}

impl Game {
    pub fn new() -> Game {
        let race_factory = RaceFactory::new();
        let room_factory = RoomFactory::new();

        Game {
            rooms: room_factory.get_rooms(),
            races: race_factory.get_races(),
            abilities: Abilities::new(),
            clients: HashMap::new(),
            players: HashMap::new(),
            database: Database::new(),
        }
    }

    pub fn add_client(&mut self, id: usize, client: Client) {
        self.clients.insert(id, client);
        self.players.insert(id, Player::new());
    }

    pub fn update_clients(&mut self) -> Vec<UpdateData> {
        let mut data = Vec::new();

        for id in 0..self.clients.len() {
            let client = self.clients.get_mut(&id).unwrap();
            let mut client_data = client.update();

            if client_data.command.len() != 0 {
                data.push(client_data);
            }
        }

        data
    }

    pub fn send(&mut self, id: usize, text: String) {
        let client = self.clients.get_mut(&id).unwrap();
        client.send(text);
    }
}