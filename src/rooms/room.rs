use std::collections::HashMap;
use rooms::item::Item;

pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, String>,
    pub features: HashMap<String, String>,
    pub items: HashMap<String, Item>,
}

impl Room {
    pub fn new(name: String, description: String, exits: HashMap<String, String>, features: HashMap<String, String>, items: HashMap<String, Item>) -> Room {
        Room {
            name: name,
            description: description,
            exits: exits,
            features: features,
            items: items,
        }
    }
}