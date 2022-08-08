use serde::{Serialize};

#[derive(Serialize, Debug)]
// A health status.
// TODO: injectable version?
pub struct Health {

    // Status message.
    status: String,

}

// Creates a new instance of a Health.
pub fn new() -> Health {
    return Health { 
        status: "serving".to_string(),
    }
}