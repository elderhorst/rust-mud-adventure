pub struct Ability {
    pub name: String,
    pub abbreviation: String,
    pub value: i32,
    pub modifier: i32,
}

pub struct Abilities {
    pub strength: Ability,
    pub dexterity: Ability,
    pub constitution: Ability,
    pub intelligence: Ability,
    pub wisdom: Ability,
    pub charisma: Ability,
}

impl Abilities {
    pub fn new() -> Abilities {
        Abilities {
            strength: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
            dexterity: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
            constitution: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
            intelligence: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
            wisdom: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
            charisma: Ability {
                name: "Strength".to_string(),
                abbreviation: "str".to_string(),
                value: 8,
                modifier: 0,
            },
        }
    }

    fn get_modifier(value: &i32) -> i32 {
        (value - 8) / 2
    }
}