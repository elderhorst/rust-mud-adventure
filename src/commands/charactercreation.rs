use std::collections::HashMap;

use game::player::Player;
use mud::mudserver::MudServer;
use rooms::room::Room;

pub fn handle_command(id: &usize, mud: &mut MudServer, rooms: &HashMap<i32, Room>, command: &String, params: &String) -> bool {
    /*# if the player hasn't given their name yet, use this first command as
    # their name and move them to the starting room.
    if players[id].name is "":

        name = command.strip()

        if name == "":
            mud.send_message(id, "Invalid name entered")
            return True

        elif does_name_exist(name):
            mud.send_message(id, "A character already has that name")
            return True

        players[id].name = name
        
        mud.send_message(id, "Enter a password:")

    elif players[id].password is "":

        hash = bcrypt.hashpw(command.encode('utf-8'), salt)

        players[id].password = hash

        mud.send_message(id, "Enter password again to confirm")

    elif players[id].status.confirmed_password is False:

        if bcrypt.checkpw(command.encode('utf-8'), players[id].password) is False:
            mud.send_message(id, "Password not the same, please try again")

            return True

        players[id].status.confirmed_password = True

        mud.send_message(id, get_race_selection_message())

    elif players[id].status.confirmed_password and players[id].status.confirmed_race is False:

        if command.find("help") != -1:
            race_name = params.lower()
            message = "Race not found"

            for race in races:
                if race.name.lower() == race_name:
                    message = race.description
                    break

            mud.send_message(id, message)

        else:
            selected_race = None

            for race in races:
                if race.name.lower() == command.lower():
                    selected_race = race
                    break

            if selected_race is None:
                 return False

            players[id].race = selected_race
            players[id].status.logged_in = True

            mud.send_message(id, "Character created successfully!")

            # go through all the players in the game
            for pid, _pl in players.items():
                # send each player a message to tell them about the new player
                mud.send_message(pid, "{} entered the game".format(players[id].name))

            # send the new player a welcome message
            mud.send_message(id, "Welcome to the game, {}. ".format(
                                                            players[id].name)
                                + "Type 'help' for a list of commands. Have fun!")

            players[id].room = "Old Road"

            # send the new player the description of their current room
            mud.send_message(id, rooms[players[id].room]["description"])

            add_player(players[id])

            # log that a user logged in
            print("{}: User '{}' created character".format(time.asctime(), id))
    
    else:
        return False

    return True*/

    return false
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