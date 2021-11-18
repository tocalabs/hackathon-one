#[macro_use] extern crate rocket;



struct Character {
    name: String,
    level: i16,
    is_dead: bool
}

impl Character {
    fn new(name: String) -> Self {
        Character {name, level: 0, is_dead: false}
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hi there!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
