use commands::status::CommandStatus;
use game::game::Game;
use game::race::Race;

impl Game {
    pub fn handle_cc_command(&mut self, id: &usize, command: &String, params: &String) -> CommandStatus {
        let mut status = CommandStatus::new();
        
        if self.players[&id].name == "" {
            let name: String = command.trim().to_string();

            if name == "" {
                status.queue(*id, "Invalid name entered".to_string());
                status.handled = true;
                return status;
            }
            else if self.database.does_player_exist(&name) {
                status.queue(*id, "A character already has that name".to_string());
                status.handled = true;
                return status;
            }

            self.players.get_mut(&id).unwrap().name = name;

            status.queue(*id, "Enter a password:".to_string());
        }
        else if self.players.get(&id).unwrap().password == "" {
            let hash = command.clone();//TODO: bcrypt.hashpw(command.encode("utf-8"), salt);

            self.players.get_mut(&id).unwrap().password = hash;

            status.queue(*id, "Enter password again to confirm".to_string());
        }
        else if self.players.get(&id).unwrap().status.confirmed_password == false {
            // TODO
            /*if bcrypt.checkpw(command.encode("utf-8"), players[id].password) == false {
                self.send(id, "Password not the same, please try again");

                return true;
            }*/
            // TEMPORARY
            if *command != self.players[&id].password {
                status.queue(*id, "Password not the same, please try again".to_string());

                status.handled = true;
                return status;
            }

            self.players.get_mut(&id).unwrap().status.confirmed_password = true;
            let text = self.get_race_selection_message();
            status.queue(*id, text);
        }
        else if self.players[&id].status.confirmed_password && self.players[&id].status.confirmed_race == false {
            if command.find("help") != None {
                let race_name = params.to_lowercase();
                let mut message = "Race not found".to_string();

                for (_key, race) in self.races.iter() {
                    if race.name.to_lowercase() == race_name {
                        message = race.description.clone();
                        break;
                    }
                }

                status.queue(*id, message);
            }
            else {
                let mut selected_race: Race = Race::empty();

                for (_key, race) in self.races.iter() {
                    if race.name.to_lowercase() == command.to_lowercase() {
                        selected_race = Race {
                            name: race.name.clone(),
                            plural: race.plural.clone(),
                            description: race.description.clone(),
                            bonuses: race.bonuses.clone(),
                        };
                        break;
                    }
                }

                if selected_race.name == Race::empty().name {
                    status.queue(*id, format!("Error processing the name '{}', please try again", selected_race.name));
                    status.handled = true;

                    return status;
                }

                self.players.get_mut(&id).unwrap().race = selected_race.name;
                self.players.get_mut(&id).unwrap().status.logged_in = true;

                status.queue(*id, "Character created successfully!".to_string());

                // go through all the players in the game
                // TODO
                /*
                for (pid, _pl) in players.items() {
                    // send each player a message to tell them about the new player
                    server_data.send(*pid, "{} entered the game".format(players[id].name));
                }*/

                // send the new player a welcome message
                status.queue(*id, format!("Welcome to the game, {}. Type 'help' for a list of commands. Have fun!", self.players[&id].name));

                self.players.get_mut(&id).unwrap().room = "Old Road".to_string();

                // send the new player the description of their current room
                let room_name = self.players[&id].room.clone();
                let description = self.rooms[&room_name].description.clone();
                status.queue(*id, description.to_string());

                self.database.add_player(&self.players.get(&id).unwrap());

                // log that a user logged in
                //println!("{}: User '{}' created character".format("some time", id.to_string()));
            }
        }
        else {
            status.handled = false;
            return status;
        }

        status.handled = true;
        return status;
    }

    fn get_race_selection_message(&self) -> String {
        let mut message = "".to_string();
        let mut count = 0;

        for (_key, race) in self.races.iter() {
            message += &format!("{}    ", race.name);

            if count % 4 == 0 && count != 0 {
                message += &'\n'.to_string();
            }

            count += 1;
        }

        message += &"\nFor more info on a race, type 'help <race>', e.g. 'help human'".to_string();
        
        message
    }
}