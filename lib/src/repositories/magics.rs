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
            Magic::new("Water", Purpose::Offensive),
            Magic::new("Aero", Purpose::Offensive),
            Magic::new("Bio", Purpose::Offensive),
            Magic::new("Demi", Purpose::Offensive),
            Magic::new("Quake", Purpose::Offensive),
            Magic::new("Tornado", Purpose::Offensive),
            Magic::new("Holy", Purpose::Offensive),
            Magic::new("Flare", Purpose::Offensive),
            Magic::new("Meteor", Purpose::Offensive),
            Magic::new("Ultima", Purpose::Offensive),
            Magic::new("Apocalypse", Purpose::Offensive),
            Magic::new("Fire", Purpose::Offensive),
            Magic::new("Fira", Purpose::Offensive),
            Magic::new("Firaga", Purpose::Offensive),
            Magic::new("Blizzard", Purpose::Offensive),
            Magic::new("Blizzara", Purpose::Offensive),
            Magic::new("Blizzaga", Purpose::Offensive),
            Magic::new("Thunder", Purpose::Offensive),
            Magic::new("Thundara", Purpose::Offensive),
            Magic::new("Thundaga", Purpose::Offensive),
            Magic::new("Esuna", Purpose::Restorative),
            Magic::new("Cure", Purpose::Restorative),
            Magic::new("Cura", Purpose::Restorative),
            Magic::new("Curaga", Purpose::Restorative),
            Magic::new("Life", Purpose::Restorative),
            Magic::new("Full-Life", Purpose::Restorative),
            Magic::new("Regen", Purpose::Restorative),
            Magic::new("Scan", Purpose::Defensive),
            Magic::new("Sleep", Purpose::Defensive),
            Magic::new("Blind", Purpose::Defensive),
            Magic::new("Silence", Purpose::Defensive),
            Magic::new("Confuse", Purpose::Defensive),
            Magic::new("Berserk", Purpose::Defensive),
            Magic::new("Break", Purpose::Defensive),
            Magic::new("Zombie", Purpose::Defensive),
            Magic::new("Death", Purpose::Defensive),
            Magic::new("Double", Purpose::Defensive),
            Magic::new("Triple", Purpose::Defensive),
            Magic::new("Dispel", Purpose::Defensive),
            Magic::new("Protect", Purpose::Defensive),
            Magic::new("Shell", Purpose::Defensive),
            Magic::new("Reflect", Purpose::Defensive),
            Magic::new("Float", Purpose::Defensive),
            Magic::new("Drain", Purpose::Defensive),
            Magic::new("Haste", Purpose::Defensive),
            Magic::new("Slow", Purpose::Defensive),
            Magic::new("Stop", Purpose::Defensive),
            Magic::new("Meltdown", Purpose::Defensive),
            Magic::new("Pain", Purpose::Defensive),
            Magic::new("Aura", Purpose::Defensive),
        ];
        Self { magics: magics }
    }

    // Describe a single magic.
    pub fn describe(&self, name: &str) -> Result<Magic, Error> {
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
