/// This file will contain the structs for the game state
///
/// The game state will include:
/// 1. Character struct
/// 2. Current Page which will be a Page struct
/// 3. Option of Previous State
pub struct GameState {
    character: Character,
    current_page: Page,
    previous_state: Option<PreviousState>,
    level: usize,
}

pub struct Character;

/// A page represents a room/biome/environment that the character may find themselves in.
///
/// The page is akin to a page in a CYOA book.
/// The page needs a:
/// 1. Name
/// 2. Description
/// 3. Type (enum of type of envs)
/// 4. List of events that can happen to the character
pub struct Page {
    name: String,
    description: String,
    kind: Environment,
    events: Vec<Event>,
}

/// Environment descibes what type of biome the character finds themselves in
///
/// Enum is size of usize - it stores a number representing which variant is selected
pub enum Environment {
    City,
    Outskirts,
    Forest,
    Cavern,
}

trait Move {
    fn move_env(&self) -> Self;
}

impl Move for Environment {
    fn move_env(&self) -> Self {
        match &self {
            Environment::City => {}
            Environment::Outskirts => {}
            _ => {
                println!("I've moved!!")
            }
        }

        Self::City
    }
}

/// Event dictates the type of event your character is interacting with
pub enum Event {
    /// Moving from one page to another
    Transition { next_room: Environment },
    /// Something that happens to the character
    Encounter { kind: EncounterType },
}

pub enum EncounterType {
    Combat,
    Friendly,
    BinaryDecision(String),
    UseItem(Item),
    Puzzle(String),
}

pub struct Item {
    damage: usize,
    health_restoration: usize,
}

/// Previous state will contain previous page and previous character
///
/// It will only appear if level is easy in game state
pub struct PreviousState;
