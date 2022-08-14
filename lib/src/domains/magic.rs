use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

// Magic purpose enumerated values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Purpose {
    Offensive,
    Restorative,
    Defensive,
}

impl fmt::Display for Purpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A magic.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Magic {
    pub name: String,
    pub purpose: Purpose,
}

// Magic implementation to initialize a new magic.
impl Magic {
    // Creates a new instance of a magic.
    pub fn new(name: String, purpose: Purpose) -> Self {
        Self { name, purpose }
    }
}
