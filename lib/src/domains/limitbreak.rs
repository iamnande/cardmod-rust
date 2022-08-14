use serde::{Deserialize, Serialize};

// A limitbreak.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LimitBreak {
    pub name: String,
}

// LimitBreak implementation to initialize a new limitbreak.
impl LimitBreak {
    // Creates a new instance of a limitbreak.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
