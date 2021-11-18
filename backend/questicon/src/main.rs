#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hi there!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}