extern crate cardmod;

use cardmod::repositories::refinements;

fn main() {
    // cardmod: test refinements refine
    let repo = refinements::Repository::new();
    let res = repo.refine(refinements::RefineInput {
        target: String::from("Dino Bone"),
        quantity: 100,
    });
    println!("{}", serde_json::to_string(&res).unwrap());
}
