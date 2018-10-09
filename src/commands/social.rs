use std::collections::HashMap;

use game::player::Player;
use mud::mudserver::MudServer;
use rooms::room::Room;

pub fn handle_social_command(id: &usize, mud: &mut MudServer, rooms: &HashMap<i32, Room>, command: &String, params: &String) -> bool {

    // 'say' command
    if command == "say" {
        // go through every player in the game
        /*for pid, _pl in players.items() {
            // if they're in the same room as the player
            if players[pid].room == players[id].room {
                // send them a message telling them what the player said
                mud.send_message(pid, "{} says: {}".format(players[id].name, params))
            }
        }*/
    }
    /*                               
    // 'emote' command
    elif command == "emote":

        emote = params.lower()

        // go through all the players in the game
        for pid, _pl in players.items():
            // display emote to all in the room
            if players[pid].room == players[id].room:
                // send them a message telling them that the player
                // left the room
                mud.send_message(pid, "{} {}".format(players[id].name, emote))

    // 'shout' command
    elif command == "shout":

        // go through every player in the game
        for pid, _pl in players.items():
            // send them a message telling them what the player said
            mud.send_message(pid, "{} shouts: {}".format(players[id].name, params))

    // 'whisper' command
    elif command == "whisper":

        text = params
        spaceIndex = text.find(" ")

        if spaceIndex is not -1:
            recipient = text[0:spaceIndex]
            message = text[spaceIndex + 1:len(text)]
            found = False

            for pid, _pl in players.items():
                if players[pid].name == recipient:
                    mud.send_message(pid, "{} whispers: {}".format(players[id].name, message))
                    mud.send_message(id, "{} whispers: {}".format(players[id].name, message))
                    found = True
                    break

            if not found:
                mud.send_message(id, "Player '{}' not found".format(recipient))
        else:
            mud.send_message(id, "Error parsing whisper command, try 'help'")

    // 'who' command
    elif command == "who":

        mud.send_message(id, "Currently online:")
        playerCount = 0

        # go through every player in the game
        for pid, _pl in players.items():
            # print every online name
            mud.send_message(id, "{}".format(players[pid].name))

            playerCount += 1

        mud.send_message(id, "{} player(s) are online".format(playerCount))
    */
    else {
        return false;
    }

    return true
}