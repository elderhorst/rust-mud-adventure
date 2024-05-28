use crate::game::game::Game;

impl Game {
	pub fn handle_world(&mut self, id: &usize, command: &String, params: &String) -> bool {
		match command.as_str() {
			"help" => self.help(id),
			"look" => self.look(id, params),
			"go" => self.go(id, params),
			"take" => self.take(id, params),
			"status" => self.status(id),
			"inventory" => self.messages.queue(*id, format!("Name: {}\n", self.players[id].inventory)),
			&_ => return false,
		}
		
		true
	}
	
	fn go(&mut self, id: &usize, params: &String) {
		let ex = params.to_lowercase();
		let player = self.players[id].clone();
		let player_name = player.name.clone();
		let room_id = player.room_id.clone();

		if self.rooms[&room_id].exits.contains_key(&ex) {
			for (pid, pl) in self.players.iter() {
				if pid != id && pl.room_id == room_id {
					self.messages.queue(*pid, format!("{} left via exit '{}'", player_name, ex));
				}
			}

			self.players.get_mut(&id).unwrap().room_id = room_id.clone();

			for (pid, pl) in self.players.iter() {
				if pid != id && pl.room_id == room_id {
					self.messages.queue(*pid, format!("{} arrived via exit '{}'", player_name, ex));
				}
			}

			self.messages.queue(*id, format!("You arrive at '{}'", room_id));

			// TODO
			//update_player_room(players[id]);
		}
		else {
			self.messages.queue(*id, format!("Unknown exit '{}'", room_id));
		}
	}
	
	fn help(&mut self, id: &usize) {
		self.messages.queue(*id, "Commands:".to_string());
			
		self.messages.queue(*id, "  look            - Describes the room that you are currently in, e.g. 'look'".to_string());
		self.messages.queue(*id, "  look <object>   - Attempt to examine a specific thing in the room you are in, e.g. 'look fireplace'".to_string());
		self.messages.queue(*id, "  go <exit>       - Moves through the exit specified, e.g. 'go outside'".to_string());
		self.messages.queue(*id, "  status          - Prints the current status of your character, e.g. 'status'".to_string());
		self.messages.queue(*id, "  inventory       - Prints the items and equipment you are carrying, e.g. 'inventory'".to_string());
		self.messages.queue(*id, "  say <message>   - Sends a message to everyone in the same room as the player , e.g. 'say Hello'".to_string());
		self.messages.queue(*id, "  shout <message> - Sends a message to every player online, e.g. 'shout Hello'".to_string());
		self.messages.queue(*id, "  whisper <name> <message> - Sends a private message to a specific player, e.g. 'whisper Kaladrel How is the adventure going?'".to_string());
		self.messages.queue(*id, "  emote <message> - Sends a message to everyone in the same room as you describing you character doing whatever the message is, e.g. 'emote waves'".to_string());
		self.messages.queue(*id, "  quit            - quit and exit the game, e.g. 'quit'".to_string());
	}
	
	fn look(&mut self, id: &usize, params: &String) {
		let room_id = self.players[&id].room_id.clone();

		// TODO: looking at items
		//let item = null;

		if self.rooms[&room_id].features.contains_key(params) {
			let feature: String = self.rooms[&room_id].features[params].clone();
			self.messages.queue(*id, feature.to_string());
		}
		/*else if item != None {
			status.queue(*id, "Not yet implemented".to_string());
		}*/
		else {
			let description: String = self.rooms[&room_id].description.clone();
			self.messages.queue(*id, description.to_string());

			let mut players_here = Vec::new();
			let mut message: String = "".to_string();

			for (_pid, pl) in self.players.iter() {
				if pl.room_id == room_id && pl.name != "" {
					players_here.push(pl.name.clone());

					message += &"Players here:\n".to_string();

					if players_here.len() != 0 {
						for player_here in players_here.iter() {
							message += player_here;
							message += &"\n".to_string();
						}
					}
					else {
						message += &"None\n".to_string();
					}
				}
			}

			message += &"Exits are:\n".to_string();

			for (name, _) in self.rooms[&room_id].exits.iter() {
				message += name;
				message += &"\n".to_string();
			}

			self.messages.queue(*id, message.to_string());
		}
	}
	
	fn status(&mut self, id: &usize) {
		let player = self.players[id].clone();

		self.messages.queue(*id, format!("Name: {}\n", player.name));
		self.messages.queue(*id, format!("Heritage: {}\n", self.heritages[&player.heritage].name));
		self.messages.queue(*id, format!("Level: {}\n", player.level));
		self.messages.queue(*id, format!("Health: {} / {}\n", player.health, player.max_health));

		let abilities = player.abilities.clone();
		self.messages.queue(*id, format!("STR: {} DEX: {} CON: {}\n", abilities.strength.value, abilities.dexterity.value, abilities.constitution.value));
		self.messages.queue(*id, format!("INT: {} WIS: {} CHA: {}\n", abilities.intelligence.value, abilities.wisdom.value, abilities.charisma.value));
	}
	
	fn take(&mut self, id: &usize, _params: &String) {
		// store the player's current room
			// TODO
			/*let rm = rooms[players[id].room];
			let item = None;
			
			for room_item in rm["items"] {
				if room_item.item.name.lower() == params.lower() {
					item = room_item.item;
					break;
				}
			}

			if item != None {
				mud.send_message(id, "You pick up the {}".format(item.name));

				players[id].inventory.add_item(item);
			}
			else {
				mud.send_message(id, "'{}' could not be found".format(item.name));
			}

			status.queue(*id, message.to_string());*/
			self.messages.queue(*id, "Command not yet implemented".to_string());
	}
}