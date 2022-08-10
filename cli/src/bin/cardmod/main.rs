extern crate cardmod;

use cardmod::config;
use cardmod::repositories::cards;

fn main() {

    // cardmod: intialize config
    let cfg = config::new();
    println!("loaded {} config", cfg.environment);

    // cardmod: test repository
    let repo = cards::Repository::new();
    match repo.describe("MiniMog") {
        Ok(card)  => {
            println!("card - name:'{}' level:'{}'", card.name, card.level);
        },
        Err(e) => {
            panic!("{:?}", e)
        },
    };
}