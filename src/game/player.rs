use game::ability::Abilities;

#[derive(Clone)]
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
    pub status: Status,
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
            status: Status::new(),
        }
    }
}

#[derive(Clone)]
pub struct Status {
    pub main_menu: bool,
    pub logging_in: bool,
    pub confirmed_name: bool,
    pub creating_character: bool,
    pub confirmed_password: bool,
    pub confirmed_race: bool,
    pub confirmed_class: bool,
    pub logged_in: bool,
}

impl Status {
    pub fn new() -> Status {
        Status {
            main_menu: true,
            logging_in: false,
            confirmed_name: false,
            creating_character: false,
            confirmed_password: false,
            confirmed_race: false,
            confirmed_class: false,
            logged_in: false,
        }
    }
}