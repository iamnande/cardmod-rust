use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A refinement.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Refinement {
    pub id: String,
    pub source: String,
    pub target: String,
    pub numerator: i8,
    pub denominator: i8,
}

// Refinement implementation to initialize a new refinement.
impl Refinement {
    // Creates a new instance of a refinement.
    pub fn new(source: &str, target: &str, numerator: i8, denominator: i8) -> Self {
        let id = Uuid::new_v4().to_string();
        let source = String::from(source);
        let target = String::from(target);
        Self {
            id,
            source,
            target,
            numerator,
            denominator,
        }
    }
}
