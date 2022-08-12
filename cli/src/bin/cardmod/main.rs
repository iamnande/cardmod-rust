extern crate cardmod;

use cardmod::config;
use cardmod::repositories::cards;
use cardmod::repositories::items;
use cardmod::repositories::limitbreaks;
use cardmod::repositories::magics;

fn main() {
    // cardmod: intialize config
    let _cfg = config::new();

    // cardmod: test card describe
    let repo = cards::Repository::new();
    match repo.describe("MiniMog") {
        Ok(card) => {
            println!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"level\":{}}}",
                card.id, card.name, card.level
            );
        }
        Err(e) => {
            println!("failed to describe card: {:?}", e)
        }
    };

    // cardmod: test card list
    let cards = repo.list();
    for card in cards {
        println!(
            "{{\"id\":\"{}\",\"name\":\"{}\",\"level\":{}}}",
            card.id, card.name, card.level
        );
    }

    // cardmod: test magic describe
    let repo = magics::Repository::new();
    match repo.describe("Quake") {
        Ok(magic) => {
            println!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"purpose\":\"{}\"}}",
                magic.id, magic.name, magic.purpose
            );
        }
        Err(e) => {
            println!("failed to describe magic: {:?}", e)
        }
    };

    // cardmod: test magic list
    let magics = repo.list();
    for magic in magics {
        println!(
            "{{\"id\":\"{}\",\"name\":\"{}\",\"purpose\":\"{}\"}}",
            magic.id, magic.name, magic.purpose
        );
    }

    // cardmod: test item describe
    let repo = items::Repository::new();
    match repo.describe("Fuel") {
        Ok(item) => {
            println!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"purpose\":\"{}\"}}",
                item.id, item.name, item.purpose
            );
        }
        Err(e) => {
            println!("failed to describe item: {:?}", e)
        }
    };

    // cardmod: test item list
    let items = repo.list();
    for item in items {
        println!(
            "{{\"id\":\"{}\",\"name\":\"{}\",\"purpose\":\"{}\"}}",
            item.id, item.name, item.purpose
        );
    }

    // cardmod: test limitbreak describe
    let repo = limitbreaks::Repository::new();
    match repo.describe("L?Death") {
        Ok(limitbreak) => {
            println!(
                "{{\"id\":\"{}\",\"name\":\"{}\"}}",
                limitbreak.id, limitbreak.name
            );
        }
        Err(e) => {
            println!("failed to describe limitbreak: {:?}", e)
        }
    };

    // cardmod: test limitbreak list
    let limitbreaks = repo.list();
    for limitbreak in limitbreaks {
        println!(
            "{{\"id\":\"{}\",\"name\":\"{}\"}}",
            limitbreak.id, limitbreak.name,
        );
    }
}
