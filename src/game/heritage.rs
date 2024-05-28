use std::collections::HashMap;

#[derive(Clone)]
pub struct Heritage {
	pub id: usize,
    pub name: String,
    pub plural: String,
    pub description: String,
    pub bonuses: HashMap<String, i32>,
}

impl PartialEq for Heritage {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}