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
	pub fn new(str: i32, dex: i32, con: i32, wis: i32, int: i32, cha: i32) -> Self {
		Abilities {
			strength: Ability {
				name: "Strength".to_string(),
				abbreviation: "str".to_string(),
				value: str,
				modifier: get_modifier(&str),
			},
			dexterity: Ability {
				name: "Dexterity".to_string(),
				abbreviation: "dex".to_string(),
				value: dex,
				modifier: get_modifier(&dex),
			},
			constitution: Ability {
				name: "Constitution".to_string(),
				abbreviation: "con".to_string(),
				value: con,
				modifier: get_modifier(&con),
			},
			wisdom: Ability {
				name: "wisdom".to_string(),
				abbreviation: "wis".to_string(),
				value: wis,
				modifier: get_modifier(&wis),
			},
			intelligence: Ability {
				name: "Intelligence".to_string(),
				abbreviation: "int".to_string(),
				value: int,
				modifier: get_modifier(&int),
			},
			charisma: Ability {
				name: "Charisma".to_string(),
				abbreviation: "cha".to_string(),
				value: cha,
				modifier: get_modifier(&cha),
			},
		}
	}
	
	pub fn default() -> Self {
		Abilities {
			strength: Ability {
				name: "Strength".to_string(),
				abbreviation: "str".to_string(),
				value: 10,
				modifier: 0,
			},
			dexterity: Ability {
				name: "Dexterity".to_string(),
				abbreviation: "dex".to_string(),
				value: 10,
				modifier: 0,
			},
			constitution: Ability {
				name: "Constitution".to_string(),
				abbreviation: "con".to_string(),
				value: 10,
				modifier: 0,
			},
			wisdom: Ability {
				name: "wisdom".to_string(),
				abbreviation: "wis".to_string(),
				value: 10,
				modifier: 0,
			},
			intelligence: Ability {
				name: "Intelligence".to_string(),
				abbreviation: "int".to_string(),
				value: 10,
				modifier: 0,
			},
			charisma: Ability {
				name: "Charisma".to_string(),
				abbreviation: "cha".to_string(),
				value: 10,
				modifier: 0,
			},
		}
	}
}

fn get_modifier(value: &i32) -> i32 {
	(value - 8) / 2
}