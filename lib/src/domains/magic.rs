use std::fmt::{self, Debug};
use uuid::Uuid;

// Magic purpose enumerated values.
#[derive(Debug, Clone)]
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
#[derive(Clone, Debug)]
pub struct Magic {
    pub id: String,
    pub name: String,
    pub purpose: Purpose,
}

// Magic implementation to initialize a new magic.
impl Magic {
    // Creates a new instance of a magic.
    pub fn new(name: &str, purpose: Purpose) -> Self {
        let id = Uuid::new_v4().to_string();
        let name = String::from(name);
        Self { id, name, purpose }
    }
}
