#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct Character {
    name: String,
    level: i16,
    is_dead: bool
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {name, level: 0, is_dead: false}
    }

    fn increment_level(&mut self) {
        self.level += 1;
    }
}

#[get("/")]
fn hello() -> &'static str {
    let mut protagonist = Character::new("James".to_string());
    protagonist.increment_level();
    "Hi there!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
