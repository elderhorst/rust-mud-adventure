use game::{game_stream::GameStream, local_stream::LocalStream};

use crate::game::game::Game;

mod game {
	pub mod commands;
	pub mod ability;
	pub mod game;
	pub mod game_stream;
	pub mod heritage;
	pub mod local_stream;
	pub mod player;
	pub mod room;
}
mod server;
mod util;

fn main() {
    let mut game = Game::new();
	
	game.add_player(GameStream::Local(LocalStream::new()));
	
	game.run();
}
