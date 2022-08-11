use crate::domains::card::Card;

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
            Card::new("Geezard", 1),
            Card::new("Funguar", 1),
            Card::new("Bite Bug", 1),
            Card::new("Red Bat", 1),
            Card::new("Gayla", 1),
            Card::new("Gesper", 1),
            Card::new("Fastitocalon-F", 1),
            Card::new("Blood Soul", 1),
            Card::new("Caterchipillar", 1),
            Card::new("Cockatrice", 1),
            Card::new("Grat", 2),
            Card::new("Buel", 2),
            Card::new("Mesmerize", 2),
            Card::new("Glacial Eye", 2),
            Card::new("Belhelmel", 2),
            Card::new("Thrustaevis", 2),
            Card::new("Anacondaur", 2),
            Card::new("Creeps", 2),
            Card::new("Grendel", 2),
            Card::new("Jelleye", 2),
            Card::new("Grand Mantis", 2),
            Card::new("Forbidden", 3),
            Card::new("Armadodo", 3),
            Card::new("Tri-Face", 3),
            Card::new("Fastitocalon", 3),
            Card::new("Snow Lion", 3),
            Card::new("Ochu", 3),
            Card::new("Death Claw", 3),
            Card::new("Cactuar", 3),
            Card::new("Tonberry", 3),
            Card::new("Abyss Worm", 3),
            Card::new("Turtapod", 4),
            Card::new("Vysage", 4),
            Card::new("T-Rexaur", 4),
            Card::new("Bomb", 4),
            Card::new("Blitz", 4),
            Card::new("Wendigo", 4),
            Card::new("Torama", 4),
            Card::new("Imp", 4),
            Card::new("Blue Dragon", 4),
            Card::new("Adamantoise", 4),
            Card::new("Hexadragon", 4),
            Card::new("Iron Giant", 5),
            Card::new("Behemoth", 5),
            Card::new("Chimera", 5),
            Card::new("PuPu", 5),
            Card::new("Elastoid", 5),
            Card::new("GIM47N", 5),
            Card::new("Malboro", 5),
            Card::new("Elnoyle", 5),
            Card::new("Tonberry King", 5),
            Card::new("Wedge, Biggs", 5),
            Card::new("Fujin, Raijin", 6),
            Card::new("Elvoret", 6),
            Card::new("X-ATM092", 6),
            Card::new("Granaldo", 6),
            Card::new("Gerogero", 6),
            Card::new("Iguion", 6),
            Card::new("Abadon", 6),
            Card::new("Trauma", 6),
            Card::new("Oilboyle", 6),
            Card::new("Shumi Tribe", 6),
            Card::new("Krysta", 6),
            Card::new("Propagator", 7),
            Card::new("Jumbo Cactuar", 7),
            Card::new("Tri-Point", 7),
            Card::new("Gargantua", 7),
            Card::new("Mobile Type 8", 7),
            Card::new("Sphinxara", 7),
            Card::new("Tiamat", 7),
            Card::new("BGH251F2", 7),
            Card::new("Red Giant", 7),
            Card::new("Catoblepas", 7),
            Card::new("Ultima Weapon", 7),
            Card::new("Chubby Chocobo", 8),
            Card::new("Angelo", 8),
            Card::new("Gilgamesh", 8),
            Card::new("MiniMog", 8),
            Card::new("Chicobo", 8),
            Card::new("Quezacotl", 8),
            Card::new("Shiva", 8),
            Card::new("Ifrit", 8),
            Card::new("Siren", 8),
            Card::new("Sacred", 8),
            Card::new("Minotaur", 8),
            Card::new("Carbuncle", 9),
            Card::new("Diablos", 9),
            Card::new("Leviathan", 9),
            Card::new("Odin", 9),
            Card::new("Pandemona", 9),
            Card::new("Cerberus", 9),
            Card::new("Alexander", 9),
            Card::new("Phoenix", 9),
            Card::new("Bahamut", 9),
            Card::new("Doomtrain", 9),
            Card::new("Eden", 9),
            Card::new("Ward", 10),
            Card::new("Kiros", 10),
            Card::new("Laguna", 10),
            Card::new("Selphie", 10),
            Card::new("Quistis", 10),
            Card::new("Irvine", 10),
            Card::new("Zell", 10),
            Card::new("Rinoa", 10),
            Card::new("Edea", 10),
            Card::new("Seifer", 10),
            Card::new("Squall", 10),
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
