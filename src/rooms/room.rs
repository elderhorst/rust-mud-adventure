use std::collections::HashMap;

pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, String>,
    pub features: HashMap<String, String>,
    //items: Vec<Item>,
}

impl Room {
    pub fn new(name: String, description: String, exits: HashMap<String, String>, features: HashMap<String, String>) -> Room {
        Room {
            name: name,
            description: description,
            exits: exits,
            features: features,
        }
    }
}