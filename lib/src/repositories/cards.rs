use crate::domains::card::{Card, Level};

// The card repository.
#[derive(Debug)]
pub struct Repository {
    cards: [Card; 107],
}

// Repository enumerated errors.
#[derive(Debug)]
pub enum Error {
    NotFound,
}

// Repository implementation.
impl Repository {
    // Creates a new instance of a card repository.
    pub fn new() -> Self {
        let cards: [Card; 107] = [
            Card::new("Geezard", Level::One),
            Card::new("Funguar", Level::One),
            Card::new("Bite Bug", Level::One),
            Card::new("Red Bat", Level::One),
            Card::new("Gayla", Level::One),
            Card::new("Gesper", Level::One),
            Card::new("Fastitocalon-F", Level::One),
            Card::new("Blood Soul", Level::One),
            Card::new("Caterchipillar", Level::One),
            Card::new("Cockatrice", Level::One),
            Card::new("Grat", Level::Two),
            Card::new("Buel", Level::Two),
            Card::new("Mesmerize", Level::Two),
            Card::new("Glacial Eye", Level::Two),
            Card::new("Belhelmel", Level::Two),
            Card::new("Thrustaevis", Level::Two),
            Card::new("Anacondaur", Level::Two),
            Card::new("Creeps", Level::Two),
            Card::new("Grendel", Level::Two),
            Card::new("Jelleye", Level::Two),
            Card::new("Grand Mantis", Level::Two),
            Card::new("Forbidden", Level::Three),
            Card::new("Armadodo", Level::Three),
            Card::new("Tri-Face", Level::Three),
            Card::new("Fastitocalon", Level::Three),
            Card::new("Snow Lion", Level::Three),
            Card::new("Ochu", Level::Three),
            Card::new("Death Claw", Level::Three),
            Card::new("Cactuar", Level::Three),
            Card::new("Tonberry", Level::Three),
            Card::new("Abyss Worm", Level::Three),
            Card::new("Turtapod", Level::Four),
            Card::new("Vysage", Level::Four),
            Card::new("T-Rexaur", Level::Four),
            Card::new("Bomb", Level::Four),
            Card::new("Blitz", Level::Four),
            Card::new("Wendigo", Level::Four),
            Card::new("Torama", Level::Four),
            Card::new("Imp", Level::Four),
            Card::new("Blue Dragon", Level::Four),
            Card::new("Adamantoise", Level::Four),
            Card::new("Hexadragon", Level::Four),
            Card::new("Iron Giant", Level::Five),
            Card::new("Behemoth", Level::Five),
            Card::new("Chimera", Level::Five),
            Card::new("PuPu", Level::Five),
            Card::new("Elastoid", Level::Five),
            Card::new("GIM47N", Level::Five),
            Card::new("Malboro", Level::Five),
            Card::new("Elnoyle", Level::Five),
            Card::new("Tonberry King", Level::Five),
            Card::new("Wedge, Biggs", Level::Five),
            Card::new("Fujin, Raijin", Level::Six),
            Card::new("Elvoret", Level::Six),
            Card::new("X-ATM092", Level::Six),
            Card::new("Granaldo", Level::Six),
            Card::new("Gerogero", Level::Six),
            Card::new("Iguion", Level::Six),
            Card::new("Abadon", Level::Six),
            Card::new("Trauma", Level::Six),
            Card::new("Oilboyle", Level::Six),
            Card::new("Shumi Tribe", Level::Six),
            Card::new("Krysta", Level::Six),
            Card::new("Propagator", Level::Seven),
            Card::new("Jumbo Cactuar", Level::Seven),
            Card::new("Tri-Point", Level::Seven),
            Card::new("Gargantua", Level::Seven),
            Card::new("Mobile Type 8", Level::Seven),
            Card::new("Sphinxara", Level::Seven),
            Card::new("Tiamat", Level::Seven),
            Card::new("BGH251F2", Level::Seven),
            Card::new("Red Giant", Level::Seven),
            Card::new("Catoblepas", Level::Seven),
            Card::new("Ultima Weapon", Level::Seven),
            Card::new("Chubby Chocobo", Level::Eight),
            Card::new("Angelo", Level::Eight),
            Card::new("Gilgamesh", Level::Eight),
            Card::new("MiniMog", Level::Eight),
            Card::new("Chicobo", Level::Eight),
            Card::new("Quezacotl", Level::Eight),
            Card::new("Shiva", Level::Eight),
            Card::new("Ifrit", Level::Eight),
            Card::new("Siren", Level::Eight),
            Card::new("Sacred", Level::Eight),
            Card::new("Minotaur", Level::Eight),
            Card::new("Carbuncle", Level::Nine),
            Card::new("Diablos", Level::Nine),
            Card::new("Leviathan", Level::Nine),
            Card::new("Odin", Level::Nine),
            Card::new("Pandemona", Level::Nine),
            Card::new("Cerberus", Level::Nine),
            Card::new("Alexander", Level::Nine),
            Card::new("Phoenix", Level::Nine),
            Card::new("Bahamut", Level::Nine),
            Card::new("Doomtrain", Level::Nine),
            Card::new("Eden", Level::Nine),
            Card::new("Ward", Level::Ten),
            Card::new("Kiros", Level::Ten),
            Card::new("Laguna", Level::Ten),
            Card::new("Selphie", Level::Ten),
            Card::new("Quistis", Level::Ten),
            Card::new("Irvine", Level::Ten),
            Card::new("Zell", Level::Ten),
            Card::new("Rinoa", Level::Ten),
            Card::new("Edea", Level::Ten),
            Card::new("Seifer", Level::Ten),
            Card::new("Squall", Level::Ten),
        ];
        Self { cards: cards }
    }

    // Describe a single card.
    pub fn describe(&self, name: &str) -> Result<Card, Error> {
        let mut card: Option<Card> = None;
        for c in self.cards.clone() {
            if c.name == name {
                card = Some(c)
            }
        }
        match card {
            Some(card) => Ok(card),
            None => Err(Error::NotFound),
        }
    }

    // List the cards available.
    // TODO: search filter?
    pub fn list(&self) -> [Card; 107] {
        self.cards.clone()
    }
}
