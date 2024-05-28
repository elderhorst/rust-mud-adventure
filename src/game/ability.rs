#[derive(Clone)]
pub struct Ability {
    pub name: String,
    pub abbreviation: String,
    pub value: i32,
    pub modifier: i32,
}

#[derive(Clone)]
pub struct Abilities {
    pub strength: Ability,
    pub dexterity: Ability,
    pub constitution: Ability,
    pub intelligence: Ability,
    pub wisdom: Ability,
    pub charisma: Ability,
}

impl Abilities {
    pub fn new() -> Self {
        let mut abilities = Abilities {
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
        };

        abilities.strength.modifier = abilities.get_modifier(&abilities.strength.value);
        abilities.dexterity.modifier = abilities.get_modifier(&abilities.dexterity.value);
        abilities.constitution.modifier = abilities.get_modifier(&abilities.constitution.value);
        abilities.intelligence.modifier = abilities.get_modifier(&abilities.intelligence.value);
        abilities.wisdom.modifier = abilities.get_modifier(&abilities.wisdom.value);
        abilities.charisma.modifier = abilities.get_modifier(&abilities.charisma.value);

        abilities
    }

    fn get_modifier(&self, value: &i32) -> i32 {
        (value - 8) / 2
    }
}