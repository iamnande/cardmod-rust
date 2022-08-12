use crate::domains::limitbreak::LimitBreak;

// The limitbreak repository.
#[derive(Debug)]
pub struct Repository {
    limitbreaks: [LimitBreak; 15],
}

// Repository enumerated errors.
#[derive(Debug)]
pub enum Error {
    NotFound,
}

// Repository implementation.
impl Repository {
    // Creates a new instance of a limitbreak repository.
    pub fn new() -> Self {
        let limitbreaks: [LimitBreak; 15] = [
            LimitBreak::new("Ultra Waves"),
            LimitBreak::new("Electrocute"),
            LimitBreak::new("L?Death"),
            LimitBreak::new("Degenerator"),
            LimitBreak::new("Aqua Breath"),
            LimitBreak::new("Micro Missiles"),
            LimitBreak::new("Acid"),
            LimitBreak::new("Gatling Gun"),
            LimitBreak::new("Fire Breath"),
            LimitBreak::new("Bad Breath"),
            LimitBreak::new("White Wind"),
            LimitBreak::new("Homing Laser"),
            LimitBreak::new("Mighty Guard"),
            LimitBreak::new("Ray-Bomb"),
            LimitBreak::new("Shockwave Pulsar"),
        ];
        Self {
            limitbreaks: limitbreaks,
        }
    }

    // Describe a single limitbreak.
    pub fn describe(&self, name: &str) -> Result<LimitBreak, Error> {
        let mut limitbreak: Option<LimitBreak> = None;
        for c in self.limitbreaks.clone() {
            if c.name == name {
                limitbreak = Some(c)
            }
        }
        match limitbreak {
            Some(limitbreak) => Ok(limitbreak),
            None => Err(Error::NotFound),
        }
    }

    // List the limitbreaks available.
    // TODO: search filter?
    pub fn list(&self) -> [LimitBreak; 15] {
        self.limitbreaks.clone()
    }
}
