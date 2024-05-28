use argon2::{self, Config};

use crate::game::game::Game;

impl Game {
	pub fn handle_login_command(&mut self, id: &usize, command: &String, params: &String) -> bool { 
		if params != "" {
			self.messages.queue(*id, "Invalid command entered. Paramaters are not accepted here.".to_string());
			return true;
		}
		else if self.players[&id].name == "DefaultName" {
			self.set_name(&id, &command);
		}
		else if self.players[&id].password == "" {
			self.set_password(&id, &command);
		}
		else if self.players[&id].verified == false {
			self.confirm_password(&id, &command);
		}
		else {
			return false;
		}
		
		true
	}
	
	fn set_name(&mut self, id: &usize, command: &String) {
		let name: String = command.trim().to_string();

		if name == "" {
			self.messages.queue(*id, "Invalid name entered".to_string());
		}
		else if self.database.does_player_exist(&name) {
			self.messages.queue(*id, "A character already has that name".to_string());
		}
		else {
			self.players.get_mut(&id).unwrap().name = name;

			self.messages.queue(*id, "Enter a password:".to_string());
		}
	}
	
	fn set_password(&mut self, id: &usize, command: &String) {
		let config = Config::default();
		let salt = b"randomsalt";
		let hash = argon2::hash_encoded(command.as_bytes(), salt, &config).unwrap();

		self.players.get_mut(&id).unwrap().password = hash;

		self.messages.queue(*id, "Enter password again to confirm".to_string());
	}
	
	fn confirm_password(&mut self, id: &usize, command: &String) {
		let hash = &self.players.get(&id).unwrap().password;
		let matches = argon2::verify_encoded(hash, command.as_bytes()).unwrap();
		
		if !matches {
			self.messages.queue(*id, "Password not the same, please try again".to_string());
		}
		else {
			self.players.get_mut(&id).unwrap().verified = true;
			
			self.set_player_to_character_creation_state(&id);
		}
	}
}