use super::ability::Abilities;

#[derive(Clone)]
pub enum PlayerState {
	Login,
	CharacterCreation,
	World,
}

#[derive(Clone)]
pub struct Player {
	pub name: String,
	pub room_id: usize,
	pub state: PlayerState,
	
	pub abilities: Abilities,
    pub heritage: usize,
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
	pub inventory: String,
	
	// TODO REMOVE
	pub password: String,
	pub verified: bool,
}

impl Player {
	pub fn new() -> Self {
		Player {
			name: "DefaultName".to_string(),
			room_id: 0,
			state: PlayerState::Login,
			
			abilities: Abilities::new(),
			heritage: usize::MIN,
			health: 1,
			max_health: 1,
			level: 1,
			inventory: "".to_string(),
			
			password: "".to_string(),
			verified: false,
		}
	}
}