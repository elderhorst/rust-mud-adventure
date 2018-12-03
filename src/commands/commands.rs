use std::collections::HashMap;

use commands::charactercreation;
//use commands::login;
//use commands::social;
use game::game::Game;
//use game::player::Player;
//use mud::client::Client;
use mud::serverdata::ServerData;
use rooms::room::Room;

pub fn handle_command(id: &usize, command: &String, params: &String, server_data: &mut ServerData, game: &mut Game) {
    let rooms: &HashMap<String, Room> = &HashMap::new();// = &mut mud.game.rooms;
    
    if /*login::handle_login_command(id, mud, rooms, command, params) ||*/
        charactercreation::handle_command(&id, &command, &params, server_data, game)/* ||
        social::handle_social_command(id, mud, rooms, command, params) */{
        return
    }
    

    // 'help' command
    if command == "help" {
        // send the player back the list of possible commands
        //mud.send_message(*id, "Commands:".to_string());
        server_data.send(*id, "Commands:".to_string());
        
        server_data.send(*id, "  look            - Describes the room that you are currently in, e.g. 'look'".to_string());
        server_data.send(*id, "  look <object>   - Attempt to examine a specific thing in the room you are in, e.g. 'look fireplace'".to_string());
        server_data.send(*id, "  go <exit>       - Moves through the exit specified, e.g. 'go outside'".to_string());
        server_data.send(*id, "  status          - Prints the current status of your character, e.g. 'status'".to_string());
        server_data.send(*id, "  inventory       - Prints the items and equipment you are carrying, e.g. 'inventory'".to_string());
        server_data.send(*id, "  say <message>   - Sends a message to everyone in the same room as the player , e.g. 'say Hello'".to_string());
        server_data.send(*id, "  shout <message> - Sends a message to every player online, e.g. 'shout Hello'".to_string());
        server_data.send(*id, "  whisper <name> <message> - Sends a private message to a specific player, e.g. 'whisper Kaladrel How is the adventure going?'".to_string());
        server_data.send(*id, "  emote <message> - Sends a message to everyone in the same room as you describing you character doing whatever the message is, e.g. 'emote waves'".to_string());
        server_data.send(*id, "  quit            - quit and exit the game, e.g. 'quit'".to_string());
    }
    // 'look' command
    /*elif command == "look" {

        # store the player's current room
        rm = rooms[players[id].room]

        item = None
        for room_item in rm["items"]:
            if room_item.item.name.lower() == params.lower():
                item = room_item.item
                break

        # either the player is looking at a specific object, or the whole room
        if params in rm["features"]:
            mud.send_message(id, rm["features"][params])

        elif item != None:
            mud.send_message(id, item.description)

        else:
            # send the player back the description of their current room
            mud.send_message(id, rm["description"])

            playershere = []
            # go through every player in the game
            for pid, _pl in players.items():
                # if they're in the same room as the player
                if players[pid].room == players[id].room:
                    # ... and they have a name to be shown
                    if players[pid].name is not None:
                        # add their name to the list
                        playershere.append(players[pid].name)

            # send player a message containing the list of players in the room
            mud.send_message(id, "Players here: {}".format(
                                                    ", ".join(playershere)))

            # send player a message containing the list of exits from this room
            mud.send_message(id, "Exits are: {}".format(
                                                    ", ".join(rm["exits"])))
    }
    // 'go' command
    elif command == "go" {

        # store the exit name
        ex = params.lower()

        # store the player's current room
        rm = rooms[players[id].room]

        # if the specified exit is found in the room's exits list
        if ex in rm["exits"]:

            # go through all the players in the game
            for pid, _pl in players.items():
                # if player is in the same room and isn't the player
                # sending the command
                if players[pid].room == players[id].room \
                        and pid != id:
                    # send them a message telling them that the player
                    # left the room
                    mud.send_message(pid, "{} left via exit '{}'".format(
                                                    players[id].name, ex))

            # update the player's current room to the one the exit leads to
            players[id].room = rm["exits"][ex]
            rm = rooms[players[id].room]

            # go through all the players in the game
            for pid, _pl in players.items():
                # if player is in the same (new) room and isn't the player
                # sending the command
                if players[pid].room == players[id].room \
                        and pid != id:
                    # send them a message telling them that the player
                    # entered the room
                    mud.send_message(pid, "{} arrived via exit '{}'".format(players[id].name, ex))

            # send the player a message telling them where they are now
            mud.send_message(id, "You arrive at '{}'".format(players[id].room))

            update_player_room(players[id])

        # the specified exit wasn't found in the current room
        else:
            # send back an 'unknown exit' message
            mud.send_message(id, "Unknown exit '{}'".format(ex))
    }
    // 'take' command
    elif command == "take"{
        # store the player's current room
        rm = rooms[players[id].room]
        item = None
        
        for room_item in rm["items"]:
            if room_item.item.name.lower() == params.lower():
                item = room_item.item
                break

        if item != None:
            mud.send_message(id, "You pick up the {}".format(item.name))

            players[id].inventory.add_item(item)
        
        else:
            mud.send_message(id, "'{}' could not be found".format(item.name))
    }
    // 'status' command
    elif command == "status" {
        message = ""
        message += "Name: {}\n".format(players[id].name)
        message += "Race: {}\n".format(players[id].race.name)
        message += "Level: {}\n".format(players[id].level)
        message += "Health: {} / {}\n".format(players[id].health, players[id].max_health)

        abilities = players[id].abilities
        message += "STR: {} DEX: {} CON: {}\n".format(abilities.strength.value, abilities.dexterity.value, abilities.constitution.value)
        message += "INT: {} WIS: {} CHA: {}\n".format(abilities.intelligence.value, abilities.wisdom.value, abilities.charisma.value)

        mud.send_message(id, message)
    }
    // 'inventory' command
    elif command == "inventory" {
        mud.send_message(id, players[id].inventory.get_display_string())
    }
    // 'quit' command
    elif command == 'quit' {
        mud.send_message(id, "Goodbye! Have a nice day!")
        mud._handle_disconnect(id)
    }*/
    // some other, unrecognised command
    else { 
        // send back an 'unknown command' message
        server_data.send(*id, format!("Unknown command '{}'", command));
    }
}