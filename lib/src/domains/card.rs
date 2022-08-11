use uuid::Uuid;

// A card.
#[derive(Clone, Debug)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub level: i8,
}

// Card implementation to initialize a new card.
impl Card {
    // Creates a new instance of a card.
    pub fn new(name: &str, level: i8) -> Self {
        let id = Uuid::new_v4().to_string();
        let name = String::from(name);
        Self { id, name, level }
    }
}
