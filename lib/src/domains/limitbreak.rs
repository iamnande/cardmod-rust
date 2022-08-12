use uuid::Uuid;

// A limitbreak.
#[derive(Clone, Debug)]
pub struct LimitBreak {
    pub id: String,
    pub name: String,
}

// LimitBreak implementation to initialize a new limitbreak.
impl LimitBreak {
    // Creates a new instance of a limitbreak.
    pub fn new(name: &str) -> Self {
        let id = Uuid::new_v4().to_string();
        let name = String::from(name);
        Self { id, name }
    }
}
