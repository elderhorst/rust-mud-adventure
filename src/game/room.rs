use std::collections::HashMap;

pub struct Room {
	pub id: usize,
	pub name: String,
	pub description: String,
	pub exits: HashMap<String, usize>,
	pub features: HashMap<String, String>,
}