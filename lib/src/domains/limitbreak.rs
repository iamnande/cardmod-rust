use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A limitbreak.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LimitBreak {
    pub id: String,
    pub name: String,
}

// LimitBreak implementation to initialize a new limitbreak.
impl LimitBreak {
    // Creates a new instance of a limitbreak.
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self { id, name }
    }
}
