use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

// Item purpose enumerated values.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Purpose {
    Ammo,
    BlueMagic,
    CharacterAbility,
    CommandAbility,
    ForbiddenMedicine,
    Fuel,
    GFAbility,
    GFCompatibility,
    GFEnhancement,
    GFRecovery,
    GFSummon,
    Invincibility,
    LuvLuvG,
    Refinement,
    RenameCard,
    Restorative,
    Revival,
    Shelters,
    SpellStones,
    StatBoosting,
    StatusRecovery,
}

impl fmt::Display for Purpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Purpose::BlueMagic => write!(f, "Blue Magic"),
            Purpose::CharacterAbility => write!(f, "Character Ability"),
            Purpose::CommandAbility => write!(f, "Command Ability"),
            Purpose::ForbiddenMedicine => write!(f, "Forbidden Medicine"),
            Purpose::GFAbility => write!(f, "GF Ability"),
            Purpose::GFCompatibility => write!(f, "GF Compatibility"),
            Purpose::GFEnhancement => write!(f, "GF Enhancement"),
            Purpose::GFRecovery => write!(f, "GF Recovery"),
            Purpose::GFSummon => write!(f, "GF Summon"),
            Purpose::RenameCard => write!(f, "Rename Card"),
            Purpose::SpellStones => write!(f, "Spell Stones"),
            Purpose::StatBoosting => write!(f, "Stat Boosting"),
            Purpose::StatusRecovery => write!(f, "Status Recovery"),
            _ => write!(f, "{:?}", self),
        }
    }
}

// A item.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub purpose: Purpose,
}

// Item implementation to initialize a new item.
impl Item {
    // Creates a new instance of a item.
    pub fn new(name: String, purpose: Purpose) -> Self {
        Self { name, purpose }
    }
}
