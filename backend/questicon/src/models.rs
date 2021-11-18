use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
enum Class {
    Wizard, 
    Witch,
    Rogue,
    Fighter,
    Bard,
    Barbarian,
}

#[derive(Serialize, Deserialize)]
enum Enemies {
    Orc,
    Goblin,
    Troll,
    Kobold,
    GelatinousCube,
    Dragon,
    Giant
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    damage: i16,
    quality: i16,
    critical_hit_percentage: i16,
    percentage_change: i16,
    strong_vs: Vec<Enemies>,
    weak_vs: Vec<Enemies>
}

#[derive(Serialize, Deserialize)]
pub struct Armour {
    damage: i16,
    quality: i16,
    percentage_change: i16,
    strong_vs: Vec<Enemies>,
    weak_vs: Vec<Enemies>
}

#[derive(Serialize, Deserialize)]
pub struct MagicItem {
    damage: i16,
    charges_remaining: i16,
    percentage_change: i16,
    strong_vs: Vec<Enemies>,
    weak_vs: Vec<Enemies>
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    armour: Armour,
    weapon: Weapon,
    magic_item: MagicItem
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    name: String,
    level: i16,
    experience: i16,
    class: Class,
    hitpoints: i16,
    is_dead: bool,
    equipment: Inventory
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    levelsCompleted: i16,
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