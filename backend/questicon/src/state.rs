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
pub struct Page;

/// Previous state will contain previous page and previous character
///
/// It will only appear if level is easy in game state
pub struct PreviousState;
