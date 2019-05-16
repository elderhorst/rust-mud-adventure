use commands::status::CommandStatus;
use game::game::Game;

impl Game {
    pub fn handle_social_command(&mut self, id: &usize, command: &String, params: &String) -> CommandStatus {
        let mut status = CommandStatus::new();
        
        // 'say' command
        if command == "say" {
            let sender = self.players[&id].name.clone();
            let current_room = self.players[&id].room.clone();

            for (id, pl) in self.players.iter() {
                if current_room == pl.room {
                    status.queue(*id, format!("{} says: {}", sender, params));
                }
            }
        }                              
        // 'emote' command
        else if command == "emote" {
            let emote = params.to_lowercase();

            for (pid, _pl) in self.players.iter() {
                if self.players[pid].room == self.players[id].room {
                    status.queue(*pid, format!("{} {}", self.players[id].name, emote));
                }
            }
        }
        // 'shout' command
        else if command == "shout" {
            for (pid, _pl) in self.players.iter() {
                status.queue(*pid, format!("{} shouts: {}", self.players[id].name, params));
            }
        }
        // 'whisper' command
        else if command == "whisper" {
            let text = params;
            let space_index = text.find(" ").unwrap();

            if space_index != 0 && text.len() >= space_index {
                let recipient = &text[..space_index];
                let message = &text[(space_index + 1)..text.len()];
                let mut player_found = false;

                for (pid, _pl) in self.players.iter() {
                    if self.players[pid].name == recipient {
                        status.queue(*pid, format!("{} whispers: {}", self.players[id].name, message));
                        status.queue(*id, format!("{} whispers: {}", self.players[id].name, message));
                        player_found = true;
                        break;
                    }
                }

                if !player_found {
                    status.queue(*id, format!("Player '{}' not found", recipient));
                }
                else {
                    status.queue(*id, "Error parsing whisper command, try 'help'".to_string());
                }
            }
        }
        // 'who' command
        else if command == "who" {
            status.queue(*id, "Currently online:".to_string());
            let mut player_count = 0;

            for (_, pl) in self.players.iter() {
                status.queue(*id, format!("{}", pl.name));

                player_count += 1;
            }

           status.queue(*id, format!("{} player(s) are online", player_count));
        }
        else {
            status.handled = false;
            return status;
        }

        status.handled = true;
        return status;
    }
}