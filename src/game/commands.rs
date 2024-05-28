use crate::game::{game::Game, player::PlayerState};

pub mod character_creation;
pub mod login;
pub mod social;
pub mod world;

impl Game {
	pub fn handle_command(&mut self, id: &usize, command: &String, params: &String) {
		let state = &self.players[id].state;
		
		match state {
			PlayerState::CharacterCreation => {
				if  self.handle_cc_command(&id, &command, &params) {
					return;
				}
			},
			PlayerState::Login => {
				if self.handle_login_command(&id, &command, &params) {
					return;
				}
			}
			PlayerState::World => {
				if self.handle_social_command(&id, &command, &params) {
					return;
				}
				
				if self.handle_world(&id, &command, &params) {
					return;
				}
			}
		}
		
		self.messages.queue(*id, format!("Unknown command '{}'", command));
	}
}