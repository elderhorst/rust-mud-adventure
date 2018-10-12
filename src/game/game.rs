use std::collections::HashMap;

use game::race::{Race, RaceFactory};
use game::ability::Abilities;
use rooms::room::Room;
use rooms::roomfactory::RoomFactory;

pub struct Game {
    pub rooms: HashMap<String, Room>,
    pub races: HashMap<String, Race>,
    pub abilities: Abilities
}

impl Game {
    pub fn new() -> Game {
        let race_factory = RaceFactory::new();
        let room_factory = RoomFactory::new();

        Game {
            rooms: room_factory.get_rooms(),
            races: race_factory.get_races(),
            abilities: Abilities::new(),
        }
    }
}