use std::collections::HashMap;
use std::error::Error;
use std::{fs, io};
use csv::ReaderBuilder;

use crate::game::heritage::Heritage;
use crate::game::room::Room;

pub struct DataLoader {
	
}

#[derive(serde::Deserialize)]
struct RoomData {
	id: usize,
	name: String,
	description: String,
	exits: String,
	features: String,
}

#[derive(serde::Deserialize)]
struct HeritageData {
	id: usize,
	name_singular: String,
	name_plural: String,
	description: String,
	bonuses: String,
}

impl DataLoader {
	pub fn load_rooms(&self, file_path: &str) -> Result<HashMap<usize, Room>, Box<dyn Error>> {
		let mut rooms: HashMap<usize, Room> = HashMap::new();
		
		let data = self.load_file(file_path).unwrap();
		let mut rdr = ReaderBuilder::new()
			.delimiter(b'\t')
			.from_reader(data.as_bytes());
		
		for result in rdr.deserialize() {
			let room_data: RoomData = result?;
			
			let room = Room {
				id: room_data.id,
				name: room_data.name,
				description: room_data.description,
				exits: self.parse_room_exits(&room_data.exits),
				features: self.parse_room_features(&room_data.features),
			};
			
			rooms.insert(room.id, room);
		}
		
		Ok(rooms)
	}
	
	pub fn load_heritages(&self, file_path: &str) -> Result<HashMap<usize, Heritage>, Box<dyn Error>> {
		let mut heritages: HashMap<usize, Heritage> = HashMap::new();
		
		let data = self.load_file(file_path).unwrap();
		let mut rdr = ReaderBuilder::new()
			.delimiter(b'\t')
			.from_reader(data.as_bytes());
		
		for result in rdr.deserialize() {
			let heritage_data: HeritageData = result?;
			
			let heritage = Heritage {
				id: heritage_data.id,
				name: heritage_data.name_singular,
				plural: heritage_data.name_plural,
				description: heritage_data.description,
				bonuses: HashMap::new(),
			};
			
			heritages.insert(heritage.id, heritage);
		}
		
		Ok(heritages)
	}
	
	fn load_file(&self, file_path: &str) -> Result<String, io::Error> {
		let file_contents = fs::read_to_string(file_path);
		
		file_contents
	}
	
	fn parse_room_exits(&self, text: &str) -> HashMap<String, usize> {
		let mut exits = HashMap::new();
		
		if text.len() == 0 || !text.contains(':') {
			return exits;
		}
		
		let parts = text.split(',');
		
		for part in parts {
			let room_exit = part.split(":");
			let collection: Vec<&str> = room_exit.collect();
			
			if collection.len() == 2 {
				let name = collection[0].trim().to_string();
				let id = collection[1].trim().parse::<usize>().unwrap();
				
				exits.insert(name, id);
			}
		}
		
		exits
	}
	
	fn parse_room_features(&self, _text: &str) -> HashMap<String, String> {
		let features = HashMap::new();
		
		features
	}
}