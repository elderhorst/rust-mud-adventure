pub struct Player {
    pub name: String
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "".to_string()
        }
    }
}