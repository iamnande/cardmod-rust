use crate::domains::{card};

// The card repository.
#[derive(Debug)]
pub struct Repository {
    cards: Vec<card::Card>,
}

impl Repository {

    // Creates a new instance of a card repository.
    pub fn new() -> Self {
        let cards = vec![
            card::Card::new("MiniMog", 9),
        ];
        Self {
            cards: cards,
        }
    }

    // get single
    pub fn describe(&self, name: &str) -> &card::Card {
        for i in 0..self.cards.len() {
            if self.cards[i].name == name.to_string() {
                return self.cards.get(i).unwrap();
            }
        }
        self.cards.get(0).unwrap()
    }

}