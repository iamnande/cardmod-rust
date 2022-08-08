use uuid::Uuid;
use serde::{Serialize};
use std::io::{self, Write};

#[derive(Serialize, Debug)]
struct Card {
    id: String,
    name: String,
}

fn new_card(name: String) -> Card {
    return Card { 
        id: Uuid::new_v4().to_string(),
        name, 
    }
}

// cardmod entrypoint
fn main() -> io::Result<()> {

    // initialize card
    let card = new_card("MiniMog".to_string());

    // output the card
    io::stdout().write_all(serde_json::to_string_pretty(&card).unwrap().as_bytes())?;

    // exit okay
    Ok(())

}
