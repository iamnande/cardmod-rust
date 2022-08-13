use crate::domains::magic::{Magic, Purpose};

// The magic repository.
#[derive(Debug)]
pub struct Repository {
    magics: [Magic; 50],
}

// Repository enumerated errors.
#[derive(Debug)]
pub enum Error {
    NotFound,
}

// Repository implementation.
impl Repository {
    // Creates a new instance of a magic repository.
    pub fn new() -> Self {
        let magics: [Magic; 50] = [
            Magic::new("Water".to_string(), Purpose::Offensive),
            Magic::new("Aero".to_string(), Purpose::Offensive),
            Magic::new("Bio".to_string(), Purpose::Offensive),
            Magic::new("Demi".to_string(), Purpose::Offensive),
            Magic::new("Quake".to_string(), Purpose::Offensive),
            Magic::new("Tornado".to_string(), Purpose::Offensive),
            Magic::new("Holy".to_string(), Purpose::Offensive),
            Magic::new("Flare".to_string(), Purpose::Offensive),
            Magic::new("Meteor".to_string(), Purpose::Offensive),
            Magic::new("Ultima".to_string(), Purpose::Offensive),
            Magic::new("Apocalypse".to_string(), Purpose::Offensive),
            Magic::new("Fire".to_string(), Purpose::Offensive),
            Magic::new("Fira".to_string(), Purpose::Offensive),
            Magic::new("Firaga".to_string(), Purpose::Offensive),
            Magic::new("Blizzard".to_string(), Purpose::Offensive),
            Magic::new("Blizzara".to_string(), Purpose::Offensive),
            Magic::new("Blizzaga".to_string(), Purpose::Offensive),
            Magic::new("Thunder".to_string(), Purpose::Offensive),
            Magic::new("Thundara".to_string(), Purpose::Offensive),
            Magic::new("Thundaga".to_string(), Purpose::Offensive),
            Magic::new("Esuna".to_string(), Purpose::Restorative),
            Magic::new("Cure".to_string(), Purpose::Restorative),
            Magic::new("Cura".to_string(), Purpose::Restorative),
            Magic::new("Curaga".to_string(), Purpose::Restorative),
            Magic::new("Life".to_string(), Purpose::Restorative),
            Magic::new("Full-Life".to_string(), Purpose::Restorative),
            Magic::new("Regen".to_string(), Purpose::Restorative),
            Magic::new("Scan".to_string(), Purpose::Defensive),
            Magic::new("Sleep".to_string(), Purpose::Defensive),
            Magic::new("Blind".to_string(), Purpose::Defensive),
            Magic::new("Silence".to_string(), Purpose::Defensive),
            Magic::new("Confuse".to_string(), Purpose::Defensive),
            Magic::new("Berserk".to_string(), Purpose::Defensive),
            Magic::new("Break".to_string(), Purpose::Defensive),
            Magic::new("Zombie".to_string(), Purpose::Defensive),
            Magic::new("Death".to_string(), Purpose::Defensive),
            Magic::new("Double".to_string(), Purpose::Defensive),
            Magic::new("Triple".to_string(), Purpose::Defensive),
            Magic::new("Dispel".to_string(), Purpose::Defensive),
            Magic::new("Protect".to_string(), Purpose::Defensive),
            Magic::new("Shell".to_string(), Purpose::Defensive),
            Magic::new("Reflect".to_string(), Purpose::Defensive),
            Magic::new("Float".to_string(), Purpose::Defensive),
            Magic::new("Drain".to_string(), Purpose::Defensive),
            Magic::new("Haste".to_string(), Purpose::Defensive),
            Magic::new("Slow".to_string(), Purpose::Defensive),
            Magic::new("Stop".to_string(), Purpose::Defensive),
            Magic::new("Meltdown".to_string(), Purpose::Defensive),
            Magic::new("Pain".to_string(), Purpose::Defensive),
            Magic::new("Aura".to_string(), Purpose::Defensive),
        ];
        Self { magics: magics }
    }

    // Describe a single magic.
    pub fn describe(&self, name: String) -> Result<Magic, Error> {
        let mut magic: Option<Magic> = None;
        for c in self.magics.clone() {
            if c.name == name {
                magic = Some(c)
            }
        }
        match magic {
            Some(magic) => Ok(magic),
            None => Err(Error::NotFound),
        }
    }

    // List the magics available.
    // TODO: search filter? by purpose?
    pub fn list(&self) -> [Magic; 50] {
        self.magics.clone()
    }
}
