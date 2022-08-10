extern crate cardmod_lib;

use cardmod_lib::config;
use cardmod_lib::repositories::cards;

fn main() {

    // cardmod: intialize config
    let cfg = config::new();
    println!("loaded {} config", cfg.environment);

    // cardmod: test repository
    let repo = cards::Repository::new();
    let card = repo.describe("MiniMog");
    println!("card - name:'{}' level:'{}'", card.name, card.level);

}