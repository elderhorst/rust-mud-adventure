use mud::rusqlite::types::ToSql;
use mud::rusqlite::{Connection, MappedRows, NO_PARAMS};
use std::path::Path;

use game::ability::Abilities;
use game::player::Player;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Database {
        let mut database = Database {
            conn: Connection::open(Path::new("data.db")).unwrap(),
        };

        database.setup_table();

        database
    }

    fn setup_table(&mut self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                    name            INTEGER UNIQUE PRIMARY KEY,
                    password        TEXT NOT NULL,
                    room            TEXT NOT NULL,
                    str             INT NOT NULL,
                    dex             INT NOT NULL,
                    con             INT NOT NULL,
                    int             INT NOT NULL,
                    wis             INT NOT NULL,
                    cha             INT NOT NULL,
                    race            TEXT NOT NULL,
                    health          INT NOT NULL,
                    max_health      INT NOT NULL,
                    level           INT NOT NULL,
                    inventory       TEXT NOT NULL,
                    )",
            NO_PARAMS,
        ).unwrap();
    }

    fn get_data_iter(&mut self, command: &String) -> Vec<Player> {
        let mut stmt = self.conn
        .prepare(&command)
        .unwrap();
        let player_iter = stmt
            .query_map(NO_PARAMS, |row| Player {
                name: row.get(0),
                password: row.get(1),
                room: row.get(2),
                abilities: Abilities::remake(row.get(3), row.get(4), row.get(5), row.get(6), row.get(7), row.get(8)),
                race: row.get(9),
                health: row.get(10),
                max_health: row.get(11),
                level: row.get(12),
                inventory: row.get(13),
            }).unwrap();

        let mut players = Vec::new();

        for plyrs in player_iter {
            players.push(plyrs.unwrap());
        }

        players
    }

    pub fn does_player_exist(&mut self, name: &String) -> bool {
        false
    }

    pub fn does_password_match(&mut self, text1: &String, text2: &String) -> bool {
        true
    }

    pub fn add_player(&mut self, player: &Player) {

    }

    pub fn update_player_room(&mut self, player: &Player) {
        //
    }

    pub fn load_player_data(&mut self, player: &Player) {
        let command = format!("SELECT name FROM players WHERE name={}", player.name);

        let players = self.get_data_iter(&command);

        if players.len() == 1 {
            player = &players[0];
        }
    }
}