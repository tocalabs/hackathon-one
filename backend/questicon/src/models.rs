use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    name: String,
    level: i16,
    is_dead: bool
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {name, level: 0, is_dead: false}
    }

    pub fn increment_level(&mut self) {
        self.level += 1;
    }

	pub fn kill(&mut self) {
		self.is_dead = true;
	}
}


#[derive(Serialize, Deserialize)]
pub struct Map {
	
}