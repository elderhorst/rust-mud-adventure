use game::ability::Abilities;

pub struct Player {
    pub name: String,
    pub password: String,
    pub room: String,
    pub abilities: Abilities,
    pub race: String,
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub inventory: String,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "".to_string(),
            password: "".to_string(),
            room: "Old Road".to_string(),
            abilities: Abilities::new(),
            race: "".to_string(),
            health: 1,
            max_health: 1,
            level: 1,
            inventory: "".to_string(),
        }
    }
}

pub struct Status {

}

impl Status {
    pub fn new() -> Status {
        Status {

        }
    }
}