use std::collections::HashMap;

use rooms::room::Room;

pub struct RoomFactory {
    rooms: HashMap<String, Room>
}

impl RoomFactory {
    pub fn new() -> RoomFactory {
        let mut room_factory = RoomFactory {
            rooms: HashMap::new()
        };

        room_factory.create_rooms();

        room_factory
    }

    pub fn get_rooms(self) -> HashMap<String, Room> {
        self.rooms
    }

    fn create_rooms(&mut self) {
        self.rooms.insert("Old Road".to_string(),
            Room {
                name: "Old Road".to_string(),
                description: "You're standing in the middle of an old worn road on the outskirts of Waterdeep, which stands before you to the north.".to_string(),
                exits: [("north".to_string(), "City South Gate".to_string())].iter().cloned().collect(),
                features: HashMap::new(),
            }
        );
    }
}

/*
"City South Gate": {
        "description": "You stand in front of the South Gate into the City of Waterdeep. Guards stand on either side of the entrance.",
        "exits": {"north": "South Street 4", "south": "Old Road"},
        "features": {"guard": "The City Guard stand at attention, looking over everyone entering the city."},
        "items": [],
    },
    "South Street 4": {
        "description": "You stand on a well worn street, lined with old buildings. To the south is the South Gate of Waterdeep.",
        "exits": {"north": "South Street 3", "south": "City South Gate"},
        "features": {},
        "items": [],
    },
    "South Street 3": {
        "description": "You stand on a well worn street, lined with old buildings.",
        "exits": {"north": "South Street 2", "south": "South Street 4"},
        "features": {},
        "items": [],
    },
    "South Street 2": {
        "description": "You stand on a well worn street, lined with old buildings.",
        "exits": {"north": "South Street 1", "south": "South Street 3", "west": "South West Street 1"},
        "features": {},
        "items": [],
    },
    "South Street 1": {
        "description": "You stand on a well worn street, lined with old buildings. To the north is the Center Square of Waterdeep.",
        "exits": {"north": "Center Square", "south": "South Street 2"},
        "features": {},
        "items": [],
    },
    "Center Square": {
        "description": "CENTER SQUARE DESCRIPTION",
        "exits": {"north": "North Street 1", "east": "East Street 1", "south": "South Street 1", "west": "West Street 1"},
        "features": {},
        "items": [],
    },
    "North Street 1": {
        "description": "NORTH STREET DESCRIPTION",
        "exits": {"south": "Center Square"},
        "features": {},
        "items": [],
    },
    "East Street 1": {
        "description": "EAST STREET DESCRIPTION",
        "exits": {"west": "Center Square"},
        "features": {},
        "items": [],
    },
    "West Street 1": {
        "description": "WEST STREET DESCRIPTION",
        "exits": {"east": "Center Square"},
        "features": {},
        "items": [],
    },
    "South West Street 1": {
        "description": "The buildings that surround you on this street are all dilapidated, some are burnt husks. Few people walk the streets here, this part of the city seems largely abandoned.",
        "exits": {"east": "South Street 2", "west": "South West Street 2"},
        "features": {},
        "items": [],
    },
    "South West Street 2": {
        "description": "Amidst the ruin of this street stands a two-storey stone tavern, what appears to be a tavern.",
        "exits": {"east": "South West Street 1", "yawning portal": "Yawning Portal"},
        "features": {},
        "items": [],
    },
    "Yawning Portal": {
        "description": "YAWNING PORTAL DESCRIPTION",
        "exits": {"outside": "South Street 2"},
        "features": {},
        "items": [],
    },
    */
