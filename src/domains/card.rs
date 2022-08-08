use uuid::Uuid;
use serde::{Serialize};

#[derive(Serialize, Debug)]
// A card.
pub struct Card {

    // Unique ID of the card.
    id: String,

    // The name of the card.
    name: String,

    // The level of the card.
    level: i8,

}

// Creates a new instance of a card.
pub fn new(name: String, level: i8) -> Card {
    return Card { 
        id: Uuid::new_v4().to_string(),
        name,
        level,
    }
}