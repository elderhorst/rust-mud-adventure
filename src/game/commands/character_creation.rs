use crate::game::game::Game;

impl Game {
    pub fn handle_cc_command(&mut self, id: &usize, command: &String, params: &String) -> bool {

		if self.players[&id].heritage == usize::MIN {
			if self.choose_heritage(&id, &command, &params) {
				self.queue_new_player_message(id);
				self.set_new_player_to_world_state(id);
			}

			return true;
		}
		
        false
    }
	
	fn choose_heritage(&mut self, id: &usize, command: &String, params: &String) -> bool {
		if command == "help" {
			for (_key, heritage) in self.heritages.iter() {
				if params == &heritage.name {
					let mut abilities = "".to_string();
					
					for (name, value) in heritage.bonuses.iter() {
						abilities = format!("{}{}: {}   ", abilities, name, value);
					}
					
					self.messages.queue(*id, format!("Name: {}", &heritage.name));
					self.messages.queue(*id, format!("Description: {}", &heritage.description));
					self.messages.queue(*id, format!("Bonuses: {}", &abilities));
					
					return false;
				}
			}
			
			self.messages.queue(*id, format!("Error processing the heritage '{}', please try again", params));
			
			return false;
		}
		
		let mut selected_heritage = usize::MIN;

		for (_key, heritage) in self.heritages.iter() {
			if heritage.name.to_lowercase() == command.to_lowercase() {
				selected_heritage = heritage.id;
				break;
			}
		}
		
		if selected_heritage == usize::MIN {
			self.messages.queue(*id, format!("Error processing the name '{}', please try again", command));
			
			return false;
		}
		else {
			self.players.get_mut(&id).unwrap().heritage = selected_heritage;
			
			return true;
		}
	}
	
	fn queue_new_player_message(&mut self, id: &usize) {
		let new_player_name = &self.players[&id].name;
		for (pid, _) in self.players.iter() {
			if pid != id {
				self.messages.queue(*pid, format!("{} entered the game", new_player_name));
			}
		}

		self.messages.queue(*id, format!("Welcome to the game, {}. Type 'help' for a list of commands. Have fun!", new_player_name));
	}
}