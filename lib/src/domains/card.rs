use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use uuid::Uuid;

// Magic purpose enumerated values.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::One => write!(f, "{}", 1),
            Level::Two => write!(f, "{}", 2),
            Level::Three => write!(f, "{}", 3),
            Level::Four => write!(f, "{}", 4),
            Level::Five => write!(f, "{}", 5),
            Level::Six => write!(f, "{}", 6),
            Level::Seven => write!(f, "{}", 7),
            Level::Eight => write!(f, "{}", 8),
            Level::Nine => write!(f, "{}", 9),
            Level::Ten => write!(f, "{}", 10),
        }
    }
}

// A card.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub level: Level,
}

// Card implementation to initialize a new card.
impl Card {
    // Creates a new instance of a card.
    pub fn new(name: String, level: Level) -> Self {
        let id = Uuid::new_v4().to_string();
        Self { id, name, level }
    }
}
