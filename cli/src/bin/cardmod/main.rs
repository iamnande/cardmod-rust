extern crate cardmod;

use cardmod::config;
use cardmod::repositories::cards;

fn main() {
    // cardmod: intialize config
    let _cfg = config::new();

    // cardmod: test describe in repository
    let repo = cards::Repository::new();
    match repo.describe("MiniMog") {
        Ok(card) => {
            println!("card - name:'{}' level:'{}'", card.name, card.level);
        }
        Err(e) => {
            println!("failed to describe card: {:?}", e)
        }
    };

    // cardmod: test list in repository
    let results = repo.list();
    for result in results {
        println!("card - name:'{}' level:'{}'", result.name, result.level);
    }
}
