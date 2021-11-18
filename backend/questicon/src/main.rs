#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
mod models;
use models::{Character};

#[get("/")]
async fn hello() -> &'static str {
    let mut protagonist = Character::new("James".to_string());
    protagonist.increment_level();
    "Hi there!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
