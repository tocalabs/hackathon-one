#[macro_use] extern crate rocket;

use std::ops;

struct Xp(i8);

trait Limit {
    fn max() -> Self {
        Self(100)
    }
    fn min() -> Self {
        Self(0)
    }
}

impl Limit for Xp {
    fn max() -> Self {
        Self(100)
    }
    fn min() -> Self {
        Self(0)
    }
}

impl ops::Add for Xp {
    type Output = i8;
    fn add(self, rhs: Self) -> Self::Output {
        let total = self.0 + rhs.0;
        if total >= Self::max() {
            return total - Self::max()
        }
        total
    }
}

impl ops::Sub for Xp {
    type Output = i8;
    fn sub(self, rhs: Self) -> Self::Output {
        let total = self.0 - rhs.0;
        if total <= Self::min() {
            return Self::min();
        }
        total
    }
}

trait Survival {
    fn attack() -> i16;
    fn defend() -> i16;
}

struct Character {
    name: String,
    level: i16,
    xp: Xp,
    is_dead: bool
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: "".to_string(),
            level: 0,
            xp: Xp(0),
            is_dead: false
        }
    }
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {name, level: 0, is_dead: false, ..Default::default()}
    }
    fn increment_level(&mut self) {
        self.level += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_player(){
        assert_eq!(Character::default(), Character {"".to_string(), });
    }
}