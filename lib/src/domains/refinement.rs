use serde::{Deserialize, Serialize};

// A refinement.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Refinement {
    pub source: String,
    pub target: String,
    pub numerator: i8,
    pub denominator: i8,
}

// Refinement implementation to initialize a new refinement.
impl Refinement {
    // Creates a new instance of a refinement.
    pub fn new(source: &str, target: &str, numerator: i8, denominator: i8) -> Self {
        Self {
            source: String::from(source),
            target: String::from(target),
            numerator,
            denominator,
        }
    }
}
