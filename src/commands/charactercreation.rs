//use std::collections::HashMap;

use game::game::Game;
//use game::race::Race;
//use game::player::Player;
//use rooms::room::Room;

impl Game {
    pub fn handle_cc_command(&mut self, id: &usize, command: &String, params: &String) -> bool {
        /*let mut players = self.get_players();

        self.send(*id, "Invalid name entered".to_string());

        self.send(*id, "Invalid name entered".to_string());

        // if the player hasn't given their name yet, use this first command as
        // their name and move them to the starting room.
        if players[*id].name == "" {

            let name: String = command.trim().to_string();

            if name == "" {
                //server_data.send(*id, "Invalid name entered".to_string());
                return true;
            }
            else if self.database.does_player_exist(&name) {
                self.send(*id, "A character already has that name".to_string());
                return true;
            }

            players[*id].name = name;

            self.send(*id, "Enter a password:".to_string());
        }*/
        /*else if players[*id].password == "" {
            let hash = command.clone();//TODO: bcrypt.hashpw(command.encode("utf-8"), salt);

            players[*id].password = hash;

            server_data.send(*id, "Enter password again to confirm".to_string());
        }
        else if players[*id].status.confirmed_password == false {
            // TODO
            /*if bcrypt.checkpw(command.encode("utf-8"), players[id].password) == false {
                mud.send_message(id, "Password not the same, please try again");

                return true;
            }*/
            // TEMPORARY
            if *command != players[*id].password {
                server_data.send(*id, "Password not the same, please try again".to_string());

                return true;
            }

            players[*id].status.confirmed_password = true;

            server_data.send(*id, get_race_selection_message());
        }
        else if players[*id].status.confirmed_password && players[*id].status.confirmed_race == false {
            if command.find("help") != None {
                let race_name = params.to_lowercase();
                let mut message = "Race not found".to_string();

                for (_key, race) in game.races.iter() {
                    if race.name.to_lowercase() == race_name {
                        message = race.description.clone();
                        break;
                    }
                }

                server_data.send(*id, message);
            }
            else {
                let mut selected_race: Race = Race::empty();

                for (_key, race) in game.races.iter() {
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

                // TODO
                /*if selected_race == None {
                    return false;
                }*/

                players[*id].race = selected_race.name;
                players[*id].status.logged_in = true;

                server_data.send(*id, "Character created successfully!".to_string());

                // go through all the players in the game
                // TODO
                /*
                for (pid, _pl) in players.items() {
                    // send each player a message to tell them about the new player
                    server_data.send(*pid, "{} entered the game".format(players[id].name));
                }*/

                // send the new player a welcome message
                server_data.send(*id, format!("Welcome to the game, {}. Type 'help' for a list of commands. Have fun!", players[*id].name));

                players[*id].room = "Old Road".to_string();

                // send the new player the description of their current room
                //TODO:
                //server_data.send(*id, game.rooms[&players[*id].room].description);

                server_data.database.add_player(&players[*id]);

                // log that a user logged in
                //println!("{}: User '{}' created character".format("some time", id.to_string()));
            }
        }
        else {
            return false;
        }*/

        return true;
    }

    fn get_race_selection_message() -> String {
        let message = "".to_string();
        /*let mut count = 0;

        for race in races:
            message += race.name + "    "

            if count % 4 == 0 and count != 0:
                message += '\n'

            count += 1

        message += "\nFor more info on a race, type 'help <race>', e.g. 'help human'"
        */
        message
    }
}