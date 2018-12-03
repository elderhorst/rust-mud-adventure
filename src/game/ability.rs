pub struct Ability {
    pub name: String,
    pub abbreviation: String,
    pub value: i32,
    pub modifier: i32,
}

impl Ability {
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
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

    pub fn remake(ste: i32, dex: i32, con: i32, int: i32, wis: i32, cha: i32) -> Abilities {
        let mut abilities = Abilities::new();

        abilities.strength.set_value(ste);
        abilities.dexterity.set_value(dex);
        abilities.constitution.set_value(con);
        abilities.intelligence.set_value(int);
        abilities.wisdom.set_value(wis);
        abilities.charisma.set_value(cha);

        abilities
    }

    fn get_modifier(value: &i32) -> i32 {
        (value - 8) / 2
    }
}