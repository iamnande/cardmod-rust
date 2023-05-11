use crate::domains::refinement::Refinement;
use serde::{Deserialize, Serialize};

// The refinement repository.
#[derive(Debug)]
pub struct Repository {
    refinements: [Refinement; 87],
}

// Refine input request.
#[derive(Debug)]
pub struct RefineInput {
    pub target: String,
    pub quantity: i64,
}

// Refine output response.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefineOutput {
    pub refinements: Vec<RefineResult>,
}

// Refinement calculation result.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefineResult {
    pub source: String,
    pub quantity: i64,
    // NOTE: A maximum of two levels will ever be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refinements: Option<Vec<RefineResult>>,
}

// Repository enumerated errors.
#[derive(Debug)]
pub enum Error {
    NotFound,
}

// Default repository.
impl Default for Repository {
    fn default() -> Self {
        Repository::new()
    }
}

// Repository implementation.
impl Repository {
    // Creates a new instance of a refinement repository.
    pub fn new() -> Self {
        let refinements: [Refinement; 87] = [
            // Blue Magic
            Refinement::new("Gesper", "Black Hole", 1, 1),
            Refinement::new("Black Hole", "Degenerator", 1, 1),
            Refinement::new("Malboro", "Malboro Tentacle", 4, 1),
            Refinement::new("Malboro Tentacle", "Bad Breath", 1, 1),
            Refinement::new("Ruby Dragon", "Inferno Fang", 10, 1),
            Refinement::new("Inferno Fang", "Fire Breath", 1, 1),
            Refinement::new("Behemoth", "Barrier", 10, 1),
            Refinement::new("Barrier", "Mighty Guard", 1, 1),
            Refinement::new("SAM08G", "Running Fire", 1, 1),
            Refinement::new("Running Fire", "Gatling Gun", 1, 1),
            Refinement::new("Fastilocalon", "Water Crystal", 1, 1),
            Refinement::new("Water Crystal", "Aqua Breath", 1, 1),
            Refinement::new("Creeps", "Coral Fragment", 1, 1),
            Refinement::new("Coral Fragment", "Electrocute", 1, 1),
            Refinement::new("Gayla", "Mystery Fluid", 1, 1),
            Refinement::new("Mystery Fluid", "Acid", 1, 1),
            Refinement::new("Tri-Face", "Curse Spike", 1, 1),
            Refinement::new("Curse Spike", "L?Death", 1, 1),
            Refinement::new("Caterchipillar", "Spider Web", 1, 1),
            Refinement::new("Spider Web", "Ultra Waves", 1, 1),
            // Refinement Moment #1
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/preparing-for-the-exam
            Refinement::new("Abyss Worm", "Windmill", 1, 1),
            Refinement::new("Windmill", "Tornado", 1, 20),
            Refinement::new("Inferno Fang", "Flare", 1, 20),
            Refinement::new("Snow Lion", "North Wind", 1, 1),
            Refinement::new("North Wind", "Blizzaga", 1, 20),
            Refinement::new("Blitz", "Dynamo Stone", 1, 1),
            Refinement::new("Dynamo Stone", "Thundaga", 1, 20),
            Refinement::new("Hexadrago", "Red Fang", 3, 1),
            Refinement::new("Red Fang", "Firaga", 1, 20),
            Refinement::new("Fastitocalon-F", "Water Crystal", 5, 1),
            Refinement::new("Water Crystal", "Water", 1, 50),
            Refinement::new("Thrustaevis", "Shear Feather", 1, 1),
            Refinement::new("Shear Feather", "Aero", 1, 20),
            Refinement::new("Bomb", "Bomb Fragment", 1, 1),
            Refinement::new("Bomb Fragment", "Fira", 1, 20),
            Refinement::new("Glacial Eye", "Arctic Wind", 1, 1),
            Refinement::new("Arctic Wind", "Blizzara", 1, 20),
            Refinement::new("Coral Fragment", "Thundara", 1, 20),
            // Refinement Moment #2
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/the-timber-mission
            Refinement::new("Quistis", "Samantha Soul", 1, 3),
            Refinement::new("Samantha Soul", "Triple", 1, 60),
            Refinement::new("Zell", "Hyper Wrist", 1, 3),
            Refinement::new("Diablos", "Black Hole", 1, 100),
            Refinement::new("Black Hole", "Demi", 1, 30),
            Refinement::new("Mystery Fluid", "Meltdown", 1, 10),
            Refinement::new("Curse Spike", "Pain", 1, 10),
            Refinement::new("Armadodo", "Dino Bone", 1, 1),
            Refinement::new("T-Rexaur", "Dino Bone", 2, 1),
            Refinement::new("Dino Bone", "Quake", 1, 20),
            Refinement::new("Mesmerize", "Mesmerize Blade", 1, 1),
            Refinement::new("Mesmerize Blade", "Regen", 1, 20),
            Refinement::new("Tonberry", "Chef's Knife", 1, 1),
            Refinement::new("Tonberry King", "Chef's Knife", 1, 1),
            Refinement::new("Chef's Knife", "Death", 1, 30),
            Refinement::new("Belhelmel", "Saw Blade", 1, 1),
            Refinement::new("Saw Blade", "Death", 1, 10),
            Refinement::new("Tent", "Curaga", 1, 10),
            Refinement::new("Cockatrice", "Cockatrice Pinion", 1, 1),
            Refinement::new("Cockatrice Pinion", "Break", 1, 20),
            Refinement::new("Forbidden", "Betrayal Sword", 1, 1),
            Refinement::new("Betrayal Sword", "Confuse", 1, 20),
            Refinement::new("Anacondaur", "Venom Fang", 1, 1),
            Refinement::new("Venom Fang", "Bio", 1, 20),
            Refinement::new("Malboro Tentacle", "Bio", 1, 40),
            Refinement::new("Wendigo", "Steel Orb", 1, 1),
            Refinement::new("Steel Orb", "Demi", 1, 15),
            Refinement::new("Torama", "Life Ring", 1, 1),
            Refinement::new("Life Ring", "Life", 1, 20),
            Refinement::new("Elastoid", "Steel Pipe", 1, 1),
            Refinement::new("Steel Pipe", "Berserk", 1, 20),
            Refinement::new("Grendel", "Dragon Fin", 1, 1),
            Refinement::new("Dragon Fin", "Double", 1, 20),
            Refinement::new("Blood Soul", "Zombie Powder", 1, 1),
            Refinement::new("Zombie Powder", "Zombie", 1, 20),
            Refinement::new("Imp", "Wizard Stone", 1, 1),
            Refinement::new("Wizard Stone", "Stop", 1, 5),
            Refinement::new("Spider Web", "Slow", 1, 20),
            Refinement::new("Grat", "Magic Stone", 1, 1),
            Refinement::new("Buel", "Magic Stone", 1, 1),
            Refinement::new("Jelleye", "Magic Stone", 1, 1),
            Refinement::new("Magic Stone", "Haste", 1, 5),
            Refinement::new("Ochu", "Ochu Tentacle", 1, 1),
            Refinement::new("Ochu Tentacle", "Blind", 1, 20),
            Refinement::new("Chimera", "Regen Ring", 10, 1),
            Refinement::new("Regen Ring", "Full-Life", 1, 20),
            Refinement::new("Elnoyle", "Energy Crystal", 20, 2),
            Refinement::new("Energy Crystal", "Pulse Ammo", 2, 20),
            Refinement::new("Iron Giant", "Star Fragment", 9, 3),
            // Refinement Moment #3
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/the-timber-mission

            // Refinement Moment #4
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/galbadia-garden

            // Refinement Moment #5
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/deling-city

            // Refinement Moment #6
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/deling-city

            // Refinement Moment #7
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/the-escape

            // Refinement Moment #8
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/exploring-the-world

            // Refinement Moment #9
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/trabia-garden

            // Refinement Moment #10
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/the-aftermath

            // Refinement Moment #11
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/esthar

            // Refinement Moment #12
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/lunar-base

            // Refinement Moment #13
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/back-on-earth
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/the-final-mission

            // Refinement Moment #14
            // ref: https://gamefaqs.gamespot.com/ps4/266152-final-fantasy-viii-remastered/faqs/72431/final-preparations
        ];
        Self { refinements }
    }

    // List the refinements available.
    fn list(&self, filter: Option<String>) -> Vec<Refinement> {
        // list: initialize refinements output
        let mut results: Vec<Refinement> = Vec::new();

        // list: iterate and filter (if applicable)
        let refinements = self.refinements.clone();
        for refinement in refinements {
            // list: get current refinement
            let current = refinement;

            match filter.clone() {
                Some(filter) => {
                    if current.target == filter {
                        results.push(current);
                    }
                }
                None => {
                    results.push(current);
                }
            }
        }

        // list: return (possibly) filtered list
        results
    }

    // Calculate the refinement based on the desired resource and quantity.
    // target, quantity -> [source, quantity, Refinement]
    pub fn refine(&self, input: RefineInput) -> RefineOutput {
        // TODO: check input target exists
        // TODO: check input quantity is `q <= MAX_INPUT_QUANTITY`

        // refine: initialize refinement output
        let mut results: Vec<RefineResult> = Vec::new();

        // refine: fetch the list of refinements
        let refinements = self.list(Some(input.target));

        // refine: iterate and look for matching refinements
        for refinement in refinements {
            // refine: calculate the refinement
            let quantity = input.quantity / i64::from(refinement.denominator)
                * i64::from(refinement.numerator);

            // refine: construct the refinement result
            let mut result = RefineResult {
                source: refinement.source.clone(),
                quantity,
                refinements: None,
            };

            // calculate: do the same thing for child refinements (if applicable)
            let child_refinements = self.list(Some(refinement.source));
            for child_refinement in child_refinements {
                let child_quantity = quantity * i64::from(child_refinement.numerator);
                let child_result = vec![RefineResult {
                    source: String::from("foo"),
                    quantity: child_quantity,
                    refinements: None,
                }];
                if result.refinements.is_some() {
                    result.refinements = Some(child_result);
                }
            }

            // refine: add the refinement to the results vector
            results.push(result);
        }

        RefineOutput {
            refinements: results,
        }
    }
}
