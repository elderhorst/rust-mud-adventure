use std::collections::HashMap;

//use game::player::Player;
//use mud::mudserver::MudServer;
use rooms::room::Room;

//from game.database import does_name_exist, does_password_match, load_player_data

pub fn handle_login_command(id: &usize, rooms: &HashMap<String, Room>, command: &String, params: &String) -> bool {
    //let salt = b"$2b$12$A85OpoWnP7jlqI9fubV4Du";

    /*if players[id].status.main_menu and command == "1":
        mud.send_message(id, "Name:")
        players[id].status.main_menu = False
        players[id].status.logging_in = True

    elif players[id].status.main_menu and command == "2":
        mud.send_message(id, "Enter a name:")
        players[id].status.main_menu = False
        players[id].status.creating_character = True

    elif players[id].status.logging_in and players[id].status.confirmed_name == False:
        if (does_name_exist(command)):
            mud.send_message(id, "Password:")
            players[id].name = command
            players[id].status.confirmed_name = True
        else:
            mud.send_message(id, "{} does not exist, try again".format(command))

    elif players[id].status.confirmed_name and players[id].status.logged_in == False:
        hash = bcrypt.hashpw(command.encode("utf-8"), salt)

        if does_password_match(players[id].name, hash):
            players[id].password = hash
            players[id].status.confirmed_password = True
            players[id].status.logged_in = True

            for pid, _pl in players.items():
                # send each player a message to tell them about the new player
                mud.send_message(pid, "{} entered the game".format(players[id].name))

            # send the new player a welcome message
            mud.send_message(id, "Welcome to the game, {}. ".format(players[id].name) + "Type 'help' for a list of commands. Have fun!")

            # load the player data from the database
            players[id] = load_player_data(players[id])

            # send the player the description of their current room
            mud.send_message(id, rooms[players[id].room]["description"])

            # log that a user logged in
            print("{}: User '{}' logged in to character".format(time.asctime(), id))

        else:
            mud.send_message(id, "Password does not match, try again.")

    else:
        return False

    return True*/

    return false
}