use std::collections::HashMap;

pub struct MessageQueue {
	messages: HashMap<usize, String>,
}

impl MessageQueue {
	pub fn new() -> Self {
		MessageQueue {
			messages: HashMap::new(),
		}
	}

	pub fn queue(&mut self, id: usize, text: String) {
		if !self.messages.contains_key(&id) {
			self.messages.insert(id, text);
		}
		else {
			let new_line = &format!("\n{text}");
			self.messages.get_mut(&id).unwrap().push_str(new_line);
		}
	}
	
	pub fn clear(&mut self) {
		self.messages.clear();
	}

	pub fn get_messages(&mut self) -> HashMap<usize, String> {
		let mut sorted: HashMap<usize, String> = HashMap::new();
		
		for (index, message) in self.messages.iter() {
			if !sorted.contains_key(&index) {
				sorted.insert(*index, message.to_string());
			}
			else {
				sorted.get_mut(index).unwrap().push_str(message);
			}
		}
		
		sorted
	}
}