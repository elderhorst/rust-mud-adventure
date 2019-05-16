//use std::collections::HashMap;

//use commands::charactercreation;
//use commands::login;
//use commands::social;
use commands::status::CommandStatus;
use game::game::Game;
//use game::player::Player;
//use mud::client::Client;
//use rooms::room::Room;

impl Game {
    pub fn handle_command(&mut self, id: &usize, command: &String, params: &String) {
        let mut status: CommandStatus = self.handle_cc_command(&id, &command, &params);

        if !status.handled {
            status = self.handle_social_command(&id, &command, &params);
        }

        if !status.handled {
            status = self.handle_login_command(&id, &command, &params);
        }
        
        if status.handled {

            status.send_messages(&mut self.clients);
            return
        }

        // 'help' command
        if command == "help" {
            // send the player back the list of possible commands
            status.queue(*id, "Commands:".to_string());
            
            status.queue(*id, "  look            - Describes the room that you are currently in, e.g. 'look'".to_string());
            status.queue(*id, "  look <object>   - Attempt to examine a specific thing in the room you are in, e.g. 'look fireplace'".to_string());
            status.queue(*id, "  go <exit>       - Moves through the exit specified, e.g. 'go outside'".to_string());
            status.queue(*id, "  status          - Prints the current status of your character, e.g. 'status'".to_string());
            status.queue(*id, "  inventory       - Prints the items and equipment you are carrying, e.g. 'inventory'".to_string());
            status.queue(*id, "  say <message>   - Sends a message to everyone in the same room as the player , e.g. 'say Hello'".to_string());
            status.queue(*id, "  shout <message> - Sends a message to every player online, e.g. 'shout Hello'".to_string());
            status.queue(*id, "  whisper <name> <message> - Sends a private message to a specific player, e.g. 'whisper Kaladrel How is the adventure going?'".to_string());
            status.queue(*id, "  emote <message> - Sends a message to everyone in the same room as you describing you character doing whatever the message is, e.g. 'emote waves'".to_string());
            status.queue(*id, "  quit            - quit and exit the game, e.g. 'quit'".to_string());
        }
        // 'look' command
        else if command == "look" {

            // store the player's current room
            let room_name = self.players.get(&id).unwrap().room.clone();

            // TODO: looking at items
            //let item = null;

            // either the player is looking at a specific object, or the whole room
            if self.rooms.get(&room_name).unwrap().features.contains_key(params) {
                let feature: String = self.rooms.get(&room_name).unwrap().features[params].clone();
                status.queue(*id, feature.to_string());
                
            }
            /*else if item != None {
                status.queue(*id, "Not yet implemented".to_string());
                
            }*/
            else {
                // send the player back the description of their current room
                let description: String = self.rooms.get(&room_name).unwrap().description.clone();
                status.queue(*id, description.to_string());

                let mut players_here = Vec::new();
                // go through every player in the game
                let mut message: String = "".to_string();
                for (_pid, pl) in self.players.iter() {
                    // if they're in the same room as the player
                    if pl.room == room_name {
                        // ...and they have a name to be shown
                        if pl.name != "" {
                            // add their name to the list
                            players_here.push(pl.name.clone());

                            // send player a message containing the list of players in the room
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
                }

                // send player a message containing the list of exits from this room
                message += &"Exits are:\n".to_string();
                for (name, _) in self.rooms.get(&room_name).unwrap().exits.iter() {
                    message += name;
                    message += &"\n".to_string();
                }

                status.queue(*id, message.to_string());
            }
        }
        // 'go' command
        else if command == "go" {
            let ex = params.to_lowercase();
            let player = self.players.get(&id).unwrap().clone();
            let player_name = player.name.clone();
            let room_name = player.room.clone();

            if self.rooms.get(&room_name).unwrap().exits.contains_key(&ex) {
                for (pid, pl) in self.players.iter() {
                    if pid != id && pl.room == room_name {
                        status.queue(*pid, format!("{} left via exit '{}'", player_name, ex));
                    }
                }

                self.players.get_mut(&id).unwrap().room = room_name.clone();

                for (pid, pl) in self.players.iter() {
                    if pid != id && pl.room == room_name {
                        status.queue(*pid, format!("{} arrived via exit '{}'", player_name, ex));
                    }
                }

                status.queue(*id, format!("You arrive at '{}'", room_name));

                //update_player_room(players[id]);
            }
            else {
                status.queue(*id, format!("Unknown exit '{}'", room_name));
            }
        }
        // 'take' command
        else if command == "take" {
            // store the player's current room
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
            status.queue(*id, "Command not yet implemented".to_string());
        }
        // 'status' command
        else if command == "status" {
            let player = self.players.get(&id).unwrap().clone();

            status.queue(*id, format!("Name: {}\n", player.name));
            status.queue(*id, format!("Race: {}\n", player.race));
            status.queue(*id, format!("Level: {}\n", player.level));
            status.queue(*id, format!("Health: {} / {}\n", player.health, player.max_health));

            let abilities = player.abilities.clone();
            status.queue(*id, format!("STR: {} DEX: {} CON: {}\n", abilities.strength.value, abilities.dexterity.value, abilities.constitution.value));
            status.queue(*id, format!("INT: {} WIS: {} CHA: {}\n", abilities.intelligence.value, abilities.wisdom.value, abilities.charisma.value));
        }
        // 'inventory' command
        else if command == "inventory" {
            status.queue(*id, format!("Name: {}\n", self.players.get(&id).unwrap().inventory));
        }
        // 'quit' command
        else if command == "quit" {
            status.queue(*id, "Goodbye! Have a nice day!".to_string());
            status.queue(*id, format!("Command '{}' not fully implemented", command));
            //mud._handle_disconnect(id);
        }
        // some other, unrecognised command
        else { 
            // send back an 'unknown command' message
            status.queue(*id, format!("Unknown command '{}'", command));
        }

        status.send_messages(&mut self.clients);
    }
}