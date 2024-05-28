use crate::game::game::Game;

impl Game {
	pub fn handle_social_command(&mut self, id: &usize, command: &String, params: &String) -> bool {
		match command.as_str() {
			"say" => self.say(id, params),
			"emote" => self.emote(id, params),
			"shout" => self.shout(id, params),
			"whisper" => self.whisper(id, params),
			"who" => self.who(id),
			&_ => return false,
		}
		
		true
	}
	
	fn say(&mut self, id: &usize, params: &String) {
		let sender = self.players[&id].name.clone();
		let current_room = self.players[&id].room_id.clone();

		for (id, pl) in self.players.iter() {
			if current_room == pl.room_id {
				self.messages.queue(*id, format!("{} says: {}", sender, params));
			}
		}
	}
	
	fn emote(&mut self, id: &usize, params: &String) {
		let emote = params.to_lowercase();

		for (pid, _pl) in self.players.iter() {
			if self.players[pid].room_id == self.players[id].room_id {
				self.messages.queue(*pid, format!("{} {}", self.players[id].name, emote));
			}
		}
	}
	
	fn shout(&mut self, id: &usize, params: &String) {
		for (pid, _pl) in self.players.iter() {
			self.messages.queue(*pid, format!("{} shouts: {}", self.players[id].name, params));
		}
	}
	
	fn whisper(&mut self, id: &usize, params: &String) {
		let text = params;
		let space_index = text.find(" ").unwrap();

		if space_index != 0 && text.len() >= space_index {
			let recipient = &text[..space_index];
			let message = &text[(space_index + 1)..text.len()];
			let mut player_found = false;

			for (pid, _pl) in self.players.iter() {
				if self.players[pid].name == recipient {
					self.messages.queue(*pid, format!("{} whispers: {}", self.players[id].name, message));
					self.messages.queue(*id, format!("{} whispers: {}", self.players[id].name, message));
					player_found = true;
					break;
				}
			}

			if !player_found {
				self.messages.queue(*id, format!("Player '{}' not found", recipient));
			}
			else {
				self.messages.queue(*id, "Error parsing whisper command, try 'help'".to_string());
			}
		}
	}
	
	fn who(&mut self, id: &usize) {
		self.messages.queue(*id, "Currently online:".to_string());
		let mut player_count = 0;

		for (_, pl) in self.players.iter() {
			self.messages.queue(*id, format!("{}", pl.name));

			player_count += 1;
		}

		self.messages.queue(*id, format!("{} player(s) are online", player_count));
	}
}