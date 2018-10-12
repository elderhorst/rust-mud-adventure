pub struct Player {
    pub name: String,
    pub current_room: String,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "".to_string(),
            current_room: "Old Road".to_string(),
        }
    }
}