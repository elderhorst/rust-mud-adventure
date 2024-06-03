use std::{collections::HashMap, io::{self, Read}};

use crate::{server::client::Client, util::{data_loader::DataLoader, database::Database, message_queue::MessageQueue}};

use super::{game_stream::GameStream, heritage::Heritage, player::{Player, PlayerState}, room::Room};

pub struct Game {
	status: bool,
	pub players: HashMap<usize, Player>,
	pub clients: HashMap<usize, Client>,
	pub rooms: HashMap<usize, Room>,
	pub database: Database,
	pub messages: MessageQueue,
	
	// TODO MOVE
	pub heritages: HashMap<usize, Heritage>,
	
	id_counter: usize,
}

impl Game {
	pub fn new() -> Self {
		let loader = DataLoader { };
	
		let rooms = loader.load_rooms("data/tad-rooms.tsv").unwrap();
		let heritages = loader.load_heritages("data/tad-heritages.tsv").unwrap();
		
		Game {
			status: true,
			players: HashMap::new(),
			clients: HashMap::new(),
			rooms: rooms,
			database: Database::new(),
			messages: MessageQueue::new(),
			
			heritages: heritages,
			
			id_counter: 0,
		}
	}
	
	pub fn add_player(&mut self, stream: GameStream) {
		self.clients.insert(self.id_counter, Client::new(stream));
		self.players.insert(self.id_counter, Player::new());
		
		let id = self.id_counter;
		self.set_player_to_login_state(&id);
		
		self.id_counter += 1;
	}
	
	pub fn run(&mut self) {
		println!("Game starting...");
		
		loop {
			let input = self.get_input(&0).unwrap();
			
			if !input.is_empty() {
				if input == "exit" || input == "quit" {
					self.status = false;
					println!("Goodbye!");
					break;
				}
				
				let command = self.process_input(&input);
				
				self.handle_command(&0, &command.0, &command.1);
			}
			
			self.print_messages();
		}
	}
	
	fn get_input(&mut self, id: &usize) -> io::Result<String> {
		let mut line = String::new();
		self.clients.get_mut(id).unwrap().stream.read_to_string(&mut line)?;
		line = line.trim_end().to_string();
		
		Ok(line)
	}
	
	fn process_input(&mut self, input: &String) -> (String, String) {
        let mut command : String;
        let params : String;
		let offset = input.find(' ').unwrap_or(input.len());
		
		if offset != input.len() {
			command = input.to_string();
			params = command.split_off(offset).trim().to_string();
		}
		else {
			command = input.to_string();
			params = "".to_string();
		}

        (command, params)
    }
	
	fn print_messages(&mut self) {
		let output = self.messages.get_messages();
		
		for (_, message) in output {
			println!("{}", message);
		}
		
		self.messages.clear();
	}
	
	pub fn set_player_to_login_state(&mut self, id: &usize) {
		self.messages.queue(*id, "Enter a username: ".to_string());
		
		self.players.get_mut(&id).unwrap().state = PlayerState::Login;
	}
	
	pub fn set_player_to_character_creation_state(&mut self, id: &usize) {
		self.messages.queue(*id, self.get_heritage_selection_message());
		
		self.players.get_mut(&id).unwrap().state = PlayerState::CharacterCreation;
	}
	
	pub fn set_new_player_to_world_state(&mut self, id: &usize) {
		self.players.get_mut(&id).unwrap().room_id = 1;
		
		self.database.add_player(&self.players[&id]);
		
		let room = &self.rooms[&self.players[&id].room_id];
		self.messages.queue(*id, room.description.to_string());
		
		self.players.get_mut(&id).unwrap().state = PlayerState::World;
	}
	
	fn get_heritage_selection_message(&self) -> String {
        let mut message = "Choose a heritage: ".to_string();
        let mut count = 0;

        for (_key, heritage) in self.heritages.iter() {
            message += &format!("{}    ", heritage.name);

            if count % 4 == 0 && count != 0 {
                message += &'\n'.to_string();
            }

            count += 1;
        }

        message += &"\nFor more info on a heritage, type 'help <heritage>', e.g. 'help human'".to_string();
        
        message
    }
}