//use mud::rusqlite::types::ToSql;
use mud::rusqlite::{Connection, /*MappedRows,*/ NO_PARAMS};
use std::path::Path;

use game::ability::Abilities;
use game::player::Player;
use game::player::Status;

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
                    name            TEXT PRIMARY KEY,
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
            //player = &players[0];
            //TODO
        }
    }
}