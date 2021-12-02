#[macro_use]
extern crate rocket;

use std::ops;

#[derive(Debug, PartialEq)]
struct Xp(i8);

trait Limit {
    fn max() -> i8 {
        100
    }
    fn min() -> i8 {
        0
    }
}

impl Limit for Xp {}

impl ops::Add for Xp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let total = self.0 + rhs.0;
        if total >= Self::max() {
            return Self(total - Self::max());
        }
        Self(total)
    }
}

impl ops::Sub for Xp {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let total = self.0 - rhs.0;
        if total <= Self::min() {
            return Self(Self::min());
        }
        Self(total)
    }
}

trait Survival {
    fn attack() -> i16;
    fn defend() -> i16;
}

#[derive(Debug, PartialEq)]
struct Character {
    name: String,
    level: i16,
    xp: Xp,
    is_dead: bool,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: "".to_string(),
            level: 0,
            xp: Xp(0),
            is_dead: false,
        }
    }
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {
            name,
            ..Default::default()
        }
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
    fn default_player() {
        assert_eq!(
            Character::default(),
            Character {
                name: "".to_string(),
                level: 0,
                xp: Xp(0),
                is_dead: false
            }
        );
    }

    #[test]
    fn xp_ceiling() {
        let original_xp = Xp(90);
        let new_xp = Xp(20);
        let added_xp = original_xp + new_xp;
        assert_eq!(added_xp, Xp(10));
    }

    #[test]
    fn xp_floor() {
        let low_xp = Xp(20);
        let negative_xp = Xp(25);
        let subtracted_xp = low_xp - negative_xp;
        assert_eq!(subtracted_xp, Xp(0));
    }
}
