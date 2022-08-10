use crate::domains::{card};

// The card repository.
#[derive(Debug)]
pub struct Repository {
    cards: Vec<card::Card>,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
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
    pub fn describe(&self, name: &str) -> Result<&card::Card, Error> {
        let mut res: Option<&card::Card> = None;
        for i in 0..self.cards.len() {
            if self.cards[i].name == name.to_string() {
                res = self.cards.get(i);
            }
        }
        match res {
            Some(card) => {
                Ok(card)
            }
            None => {
                Err(Error::NotFound)
            }
        }
    }

}