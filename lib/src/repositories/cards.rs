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
            Card::new("Geezard".to_string(), Level::One),
            Card::new("Funguar".to_string(), Level::One),
            Card::new("Bite Bug".to_string(), Level::One),
            Card::new("Red Bat".to_string(), Level::One),
            Card::new("Gayla".to_string(), Level::One),
            Card::new("Gesper".to_string(), Level::One),
            Card::new("Fastitocalon-F".to_string(), Level::One),
            Card::new("Blood Soul".to_string(), Level::One),
            Card::new("Caterchipillar".to_string(), Level::One),
            Card::new("Cockatrice".to_string(), Level::One),
            Card::new("Grat".to_string(), Level::Two),
            Card::new("Buel".to_string(), Level::Two),
            Card::new("Mesmerize".to_string(), Level::Two),
            Card::new("Glacial Eye".to_string(), Level::Two),
            Card::new("Belhelmel".to_string(), Level::Two),
            Card::new("Thrustaevis".to_string(), Level::Two),
            Card::new("Anacondaur".to_string(), Level::Two),
            Card::new("Creeps".to_string(), Level::Two),
            Card::new("Grendel".to_string(), Level::Two),
            Card::new("Jelleye".to_string(), Level::Two),
            Card::new("Grand Mantis".to_string(), Level::Two),
            Card::new("Forbidden".to_string(), Level::Three),
            Card::new("Armadodo".to_string(), Level::Three),
            Card::new("Tri-Face".to_string(), Level::Three),
            Card::new("Fastitocalon".to_string(), Level::Three),
            Card::new("Snow Lion".to_string(), Level::Three),
            Card::new("Ochu".to_string(), Level::Three),
            Card::new("Death Claw".to_string(), Level::Three),
            Card::new("Cactuar".to_string(), Level::Three),
            Card::new("Tonberry".to_string(), Level::Three),
            Card::new("Abyss Worm".to_string(), Level::Three),
            Card::new("Turtapod".to_string(), Level::Four),
            Card::new("Vysage".to_string(), Level::Four),
            Card::new("T-Rexaur".to_string(), Level::Four),
            Card::new("Bomb".to_string(), Level::Four),
            Card::new("Blitz".to_string(), Level::Four),
            Card::new("Wendigo".to_string(), Level::Four),
            Card::new("Torama".to_string(), Level::Four),
            Card::new("Imp".to_string(), Level::Four),
            Card::new("Blue Dragon".to_string(), Level::Four),
            Card::new("Adamantoise".to_string(), Level::Four),
            Card::new("Hexadragon".to_string(), Level::Four),
            Card::new("Iron Giant".to_string(), Level::Five),
            Card::new("Behemoth".to_string(), Level::Five),
            Card::new("Chimera".to_string(), Level::Five),
            Card::new("PuPu".to_string(), Level::Five),
            Card::new("Elastoid".to_string(), Level::Five),
            Card::new("GIM47N".to_string(), Level::Five),
            Card::new("Malboro".to_string(), Level::Five),
            Card::new("Elnoyle".to_string(), Level::Five),
            Card::new("Tonberry King".to_string(), Level::Five),
            Card::new("Wedge, Biggs".to_string(), Level::Five),
            Card::new("Fujin, Raijin".to_string(), Level::Six),
            Card::new("Elvoret".to_string(), Level::Six),
            Card::new("X-ATM092".to_string(), Level::Six),
            Card::new("Granaldo".to_string(), Level::Six),
            Card::new("Gerogero".to_string(), Level::Six),
            Card::new("Iguion".to_string(), Level::Six),
            Card::new("Abadon".to_string(), Level::Six),
            Card::new("Trauma".to_string(), Level::Six),
            Card::new("Oilboyle".to_string(), Level::Six),
            Card::new("Shumi Tribe".to_string(), Level::Six),
            Card::new("Krysta".to_string(), Level::Six),
            Card::new("Propagator".to_string(), Level::Seven),
            Card::new("Jumbo Cactuar".to_string(), Level::Seven),
            Card::new("Tri-Point".to_string(), Level::Seven),
            Card::new("Gargantua".to_string(), Level::Seven),
            Card::new("Mobile Type 8".to_string(), Level::Seven),
            Card::new("Sphinxara".to_string(), Level::Seven),
            Card::new("Tiamat".to_string(), Level::Seven),
            Card::new("BGH251F2".to_string(), Level::Seven),
            Card::new("Red Giant".to_string(), Level::Seven),
            Card::new("Catoblepas".to_string(), Level::Seven),
            Card::new("Ultima Weapon".to_string(), Level::Seven),
            Card::new("Chubby Chocobo".to_string(), Level::Eight),
            Card::new("Angelo".to_string(), Level::Eight),
            Card::new("Gilgamesh".to_string(), Level::Eight),
            Card::new("MiniMog".to_string(), Level::Eight),
            Card::new("Chicobo".to_string(), Level::Eight),
            Card::new("Quezacotl".to_string(), Level::Eight),
            Card::new("Shiva".to_string(), Level::Eight),
            Card::new("Ifrit".to_string(), Level::Eight),
            Card::new("Siren".to_string(), Level::Eight),
            Card::new("Sacred".to_string(), Level::Eight),
            Card::new("Minotaur".to_string(), Level::Eight),
            Card::new("Carbuncle".to_string(), Level::Nine),
            Card::new("Diablos".to_string(), Level::Nine),
            Card::new("Leviathan".to_string(), Level::Nine),
            Card::new("Odin".to_string(), Level::Nine),
            Card::new("Pandemona".to_string(), Level::Nine),
            Card::new("Cerberus".to_string(), Level::Nine),
            Card::new("Alexander".to_string(), Level::Nine),
            Card::new("Phoenix".to_string(), Level::Nine),
            Card::new("Bahamut".to_string(), Level::Nine),
            Card::new("Doomtrain".to_string(), Level::Nine),
            Card::new("Eden".to_string(), Level::Nine),
            Card::new("Ward".to_string(), Level::Ten),
            Card::new("Kiros".to_string(), Level::Ten),
            Card::new("Laguna".to_string(), Level::Ten),
            Card::new("Selphie".to_string(), Level::Ten),
            Card::new("Quistis".to_string(), Level::Ten),
            Card::new("Irvine".to_string(), Level::Ten),
            Card::new("Zell".to_string(), Level::Ten),
            Card::new("Rinoa".to_string(), Level::Ten),
            Card::new("Edea".to_string(), Level::Ten),
            Card::new("Seifer".to_string(), Level::Ten),
            Card::new("Squall".to_string(), Level::Ten),
        ];
        Self { cards: cards }
    }

    // Describe a single card.
    pub fn describe(&self, name: String) -> Result<Card, Error> {
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
