use rusqlite::Connection;
use rusqlite::Result;
use std::path::Path;

use crate::game::ability::Abilities;
use crate::game::player::Player;
use crate::game::player::PlayerState;

pub struct Database {
	conn: Connection,
}

impl Database {
	pub fn new() -> Self {
		let mut database = Database {
			conn: Connection::open(Path::new("data.db")).unwrap(),
		};

		database.setup_table();

		database
	}

	// TODO: IMPLEMENT RESULT RETURN TYPE
	fn setup_table(&mut self) {
		self.conn.execute(
			"CREATE TABLE IF NOT EXISTS players (
					name            TEXT PRIMARY KEY,
					password        TEXT NOT NULL,
					room            INT NOT NULL,
					str             INT NOT NULL,
					dex             INT NOT NULL,
					con             INT NOT NULL,
					int             INT NOT NULL,
					wis             INT NOT NULL,
					cha             INT NOT NULL,
					heritage        INT NOT NULL,
					health          INT NOT NULL,
					max_health      INT NOT NULL,
					level           INT NOT NULL,
					inventory       TEXT NOT NULL
					)",
			()
		).unwrap();
	}
	/*
	fn get_data_iter(&mut self, command: &String) -> Vec<Player> {
		let mut stmt = self.conn
		.prepare(&command)
		.unwrap();

		let player_iter = stmt
			.query_map(NO_PARAMS, |row|
				Ok (
					Player {
						name: row.get(0).unwrap(),
						password: row.get(1).unwrap(),
						room: row.get(2).unwrap(),
						abilities: Abilities::remake(row.get(3).unwrap(), row.get(4).unwrap(), row.get(5).unwrap(), row.get(6).unwrap(), row.get(7).unwrap(), row.get(8).unwrap()),
						race: row.get(9).unwrap(),
						health: row.get(10).unwrap(),
						max_health: row.get(11).unwrap(),
						level: row.get(12).unwrap(),
						inventory: row.get(13).unwrap(),
						status: Status::new(),
			})).unwrap();

		let mut players = Vec::new();

		for player in player_iter {
			players.push(player.unwrap());
		}

		players
	}
	*/
	pub fn does_player_exist(&mut self, player_name: &String) -> bool {
		let command = &format!("SELECT name FROM players WHERE name='{player_name}'");
		
		let result: Result<String> = self.conn.query_row(command, [], |row| row.get(0));
		
		result.is_ok()
	}

	pub fn does_password_match(&mut self, player_name: &String, password: &String) -> bool {
		let command = &format!("SELECT password FROM players WHERE name='{player_name}'");
		
		let result: Result<String> = self.conn.query_row(command, [], |row| row.get(0));
		
		match result {
			Ok(saved_hash) => {
				saved_hash == *password
			},
			Err(error) => {
				println!("Error getting saved password hash: {}", error);
				false
			}
		}
	}

	pub fn add_player(&mut self, player: &Player) {
		let result = self.conn.execute(
			"INSERT INTO players (name, password, room, str, dex, con, int, wis, cha, heritage, health, max_health, level, inventory) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
			(
				&player.name, &player.password, &player.room_id,
				&player.abilities.strength.value, &player.abilities.dexterity.value, &player.abilities.constitution.value, &player.abilities.wisdom.value, &player.abilities.intelligence.value, &player.abilities.charisma.value,
				&player.heritage, &player.health, &player.max_health, &player.level, &player.inventory
			),
		);
		
		if result.is_err() {
			println!("Error inserting new player to the database: {:?}", result.err());
		}
	}

	pub fn update_player_room(&mut self, player_name: String, room_id: usize) {
		let command = format!("UPDATE players SET room = '{}' where name = '{}'", room_id, player_name);
		
		match self.conn.execute(&command, []) {
			Ok(_) => {},
			Err(err) => println!("Update player room failed: {}", err),
		}
	}

	pub fn load_player_data(&mut self, player_name: &String) -> Result<Player> {
		let command = &format!("SELECT * FROM players WHERE name='{player_name}'");
		
		self.conn.query_row(command, [], |row| Ok(
			Player {
				name: row.get(0)?,
				room_id: row.get(2)?,
				abilities: Abilities::new(
					row.get(3)?,
					row.get(4)?,
					row.get(5)?,
					row.get(6)?,
					row.get(7)?,
					row.get(8)?
				),
				heritage: row.get(9)?,
				health: row.get(10)?,
				max_health: row.get(11)?,
				level: row.get(2)?,
				inventory: row.get(13)?,
				
				password: "".to_string(),
				state: PlayerState::Login,
				verified: true,
			}
		))
	}
}