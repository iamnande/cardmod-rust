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

// Default repository.
impl Default for Repository {
    fn default() -> Self {
        Repository::new()
    }
}

// Repository implementation.
impl Repository {
    // Creates a new instance of a limitbreak repository.
    pub fn new() -> Self {
        let limitbreaks: [LimitBreak; 15] = [
            LimitBreak::new("Ultra Waves".to_string()),
            LimitBreak::new("Electrocute".to_string()),
            LimitBreak::new("L?Death".to_string()),
            LimitBreak::new("Degenerator".to_string()),
            LimitBreak::new("Aqua Breath".to_string()),
            LimitBreak::new("Micro Missiles".to_string()),
            LimitBreak::new("Acid".to_string()),
            LimitBreak::new("Gatling Gun".to_string()),
            LimitBreak::new("Fire Breath".to_string()),
            LimitBreak::new("Bad Breath".to_string()),
            LimitBreak::new("White Wind".to_string()),
            LimitBreak::new("Homing Laser".to_string()),
            LimitBreak::new("Mighty Guard".to_string()),
            LimitBreak::new("Ray-Bomb".to_string()),
            LimitBreak::new("Shockwave Pulsar".to_string()),
        ];
        Self { limitbreaks }
    }

    // Describe a single limitbreak.
    pub fn describe(&self, name: String) -> Result<LimitBreak, Error> {
        let mut limitbreak: Option<LimitBreak> = None;
        for c in self.limitbreaks.clone() {
            if c.name == name {
                limitbreak = Some(c)
            }
        }
        if let Some(limitbreak) = limitbreak {
            return Ok(limitbreak);
        }
        Err(Error::NotFound)
    }

    // List the limitbreaks available.
    // TODO: search filter?
    pub fn list(&self) -> [LimitBreak; 15] {
        self.limitbreaks.clone()
    }
}
