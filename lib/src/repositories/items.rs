use crate::domains::item::{Item, Purpose};

// The item repository.
#[derive(Debug)]
pub struct Repository {
    items: [Item; 168],
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
    // Creates a new instance of a item repository.
    pub fn new() -> Self {
        let items: [Item; 168] = [
            Item::new("Potion".to_string(), Purpose::Restorative),
            Item::new("Potion+".to_string(), Purpose::Restorative),
            Item::new("Hi-Potion".to_string(), Purpose::Restorative),
            Item::new("Hi-Potion+".to_string(), Purpose::Restorative),
            Item::new("X-Potion".to_string(), Purpose::Restorative),
            Item::new("Mega-Potion".to_string(), Purpose::Restorative),
            Item::new("Phoenix Down".to_string(), Purpose::Refinement),
            Item::new("Mega Phoenix".to_string(), Purpose::Refinement),
            Item::new("Elixir".to_string(), Purpose::ForbiddenMedicine),
            Item::new("Megalixir".to_string(), Purpose::ForbiddenMedicine),
            Item::new("Antidote".to_string(), Purpose::StatusRecovery),
            Item::new("Soft".to_string(), Purpose::StatusRecovery),
            Item::new("Eye Drops".to_string(), Purpose::StatusRecovery),
            Item::new("Echo Screen".to_string(), Purpose::StatusRecovery),
            Item::new("Holy Water".to_string(), Purpose::StatusRecovery),
            Item::new("Remedy".to_string(), Purpose::StatusRecovery),
            Item::new("Remedy+".to_string(), Purpose::StatusRecovery),
            Item::new("Hero-trial".to_string(), Purpose::Invincibility),
            Item::new("Hero".to_string(), Purpose::Invincibility),
            Item::new("Holy War-trial".to_string(), Purpose::Invincibility),
            Item::new("Holy War".to_string(), Purpose::Invincibility),
            Item::new("Shell Stone".to_string(), Purpose::SpellStones),
            Item::new("Protect Stone".to_string(), Purpose::SpellStones),
            Item::new("Aura Stone".to_string(), Purpose::SpellStones),
            Item::new("Death Stone".to_string(), Purpose::SpellStones),
            Item::new("Holy Stone".to_string(), Purpose::SpellStones),
            Item::new("Flare Stone".to_string(), Purpose::SpellStones),
            Item::new("Meteor Stone".to_string(), Purpose::SpellStones),
            Item::new("Ultima Stone".to_string(), Purpose::SpellStones),
            Item::new("Gysahl Greens".to_string(), Purpose::GFSummon),
            Item::new("Phoenix Pinion".to_string(), Purpose::GFSummon),
            Item::new("Friendship".to_string(), Purpose::GFSummon),
            Item::new("Tent".to_string(), Purpose::Shelters),
            Item::new("Pet House".to_string(), Purpose::Shelters),
            Item::new("Cottage".to_string(), Purpose::Shelters),
            Item::new("G-Potion".to_string(), Purpose::GFRecovery),
            Item::new("G-Hi-Potion".to_string(), Purpose::GFRecovery),
            Item::new("G-Mega-Potion".to_string(), Purpose::GFRecovery),
            Item::new("G-Returner".to_string(), Purpose::GFRecovery),
            Item::new("Rename Card".to_string(), Purpose::RenameCard),
            Item::new("HP-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Str-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Vit-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Mag-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Spr-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Luck-J Scroll".to_string(), Purpose::GFAbility),
            Item::new("Aegis Amulet".to_string(), Purpose::GFAbility),
            Item::new("Elem Atk".to_string(), Purpose::GFAbility),
            Item::new("Elem Guard".to_string(), Purpose::GFAbility),
            Item::new("Status Atk".to_string(), Purpose::GFAbility),
            Item::new("Status Guard".to_string(), Purpose::GFAbility),
            Item::new("Rosetta Stone".to_string(), Purpose::GFAbility),
            Item::new("Magic Scroll".to_string(), Purpose::CommandAbility),
            Item::new("GF Scroll".to_string(), Purpose::CommandAbility),
            Item::new("Draw Scroll".to_string(), Purpose::CommandAbility),
            Item::new("Item Scroll".to_string(), Purpose::CommandAbility),
            Item::new("Gambler Spirit".to_string(), Purpose::CommandAbility),
            Item::new("Healing Ring".to_string(), Purpose::CommandAbility),
            Item::new("Phoenix Spirit".to_string(), Purpose::CommandAbility),
            Item::new("Med Kit".to_string(), Purpose::CommandAbility),
            Item::new("Bomb Spirit".to_string(), Purpose::CommandAbility),
            Item::new("Hungry Cookpot".to_string(), Purpose::CommandAbility),
            Item::new("Meg's Amulet".to_string(), Purpose::CommandAbility),
            Item::new("Stell Pipe".to_string(), Purpose::GFEnhancement),
            Item::new("Star Fragment".to_string(), Purpose::GFEnhancement),
            Item::new("Energy Crystal".to_string(), Purpose::GFEnhancement),
            Item::new("Samantha Soul".to_string(), Purpose::GFEnhancement),
            Item::new("Healing Mail".to_string(), Purpose::GFEnhancement),
            Item::new("Silver Sail".to_string(), Purpose::GFEnhancement),
            Item::new("Gold Armor".to_string(), Purpose::GFEnhancement),
            Item::new("Diamond Armor".to_string(), Purpose::GFEnhancement),
            Item::new("Regen Ring".to_string(), Purpose::CharacterAbility),
            Item::new("Giant's Ring".to_string(), Purpose::CharacterAbility),
            Item::new("Gaea's Ring".to_string(), Purpose::CharacterAbility),
            Item::new("Strength Love".to_string(), Purpose::CharacterAbility),
            Item::new("Power Wrist".to_string(), Purpose::CharacterAbility),
            Item::new("Hyper Wrist".to_string(), Purpose::CharacterAbility),
            Item::new("Turtle Shell".to_string(), Purpose::CharacterAbility),
            Item::new("Orihalcon".to_string(), Purpose::CharacterAbility),
            Item::new("Adamantine".to_string(), Purpose::CharacterAbility),
            Item::new("Rune Armlet".to_string(), Purpose::CharacterAbility),
            Item::new("Force Armlet".to_string(), Purpose::CharacterAbility),
            Item::new("Magic Armlet".to_string(), Purpose::CharacterAbility),
            Item::new("Circlet".to_string(), Purpose::CharacterAbility),
            Item::new("Hypno Crown".to_string(), Purpose::CharacterAbility),
            Item::new("Royal Crown".to_string(), Purpose::CharacterAbility),
            Item::new("Jet Engine".to_string(), Purpose::CharacterAbility),
            Item::new("Rocket Engine".to_string(), Purpose::CharacterAbility),
            Item::new("Moon Curain".to_string(), Purpose::CharacterAbility),
            Item::new("Steel Curtain".to_string(), Purpose::CharacterAbility),
            Item::new("Glow Curtain".to_string(), Purpose::CharacterAbility),
            Item::new("Accelerator".to_string(), Purpose::CharacterAbility),
            Item::new("Monk's Code".to_string(), Purpose::CharacterAbility),
            Item::new("Knight's Code".to_string(), Purpose::CharacterAbility),
            Item::new("Doc's Code".to_string(), Purpose::CharacterAbility),
            Item::new("Hundred Needles".to_string(), Purpose::CharacterAbility),
            Item::new("Three Stars".to_string(), Purpose::CharacterAbility),
            Item::new("Ribbon".to_string(), Purpose::CharacterAbility),
            Item::new("Normal Ammo".to_string(), Purpose::Ammo),
            Item::new("Shotgun Ammo".to_string(), Purpose::Ammo),
            Item::new("Dark Ammo".to_string(), Purpose::Ammo),
            Item::new("Fire Ammo".to_string(), Purpose::Ammo),
            Item::new("Demolition Ammo".to_string(), Purpose::Ammo),
            Item::new("Fast Ammo".to_string(), Purpose::Ammo),
            Item::new("AP Ammo".to_string(), Purpose::Ammo),
            Item::new("Pulse Ammo".to_string(), Purpose::Ammo),
            Item::new("M-Stone Piece".to_string(), Purpose::Refinement),
            Item::new("Magic Stone".to_string(), Purpose::Refinement),
            Item::new("Wizard Stone".to_string(), Purpose::Refinement),
            Item::new("Ochu Tentacle".to_string(), Purpose::Refinement),
            Item::new("Healing Water".to_string(), Purpose::Refinement),
            Item::new("Cockatrice Pinion".to_string(), Purpose::Refinement),
            Item::new("Zombie Powder".to_string(), Purpose::Refinement),
            Item::new("Lightweight".to_string(), Purpose::Refinement),
            Item::new("Sharp Spike".to_string(), Purpose::Refinement),
            Item::new("Screw".to_string(), Purpose::Refinement),
            Item::new("Saw Blade".to_string(), Purpose::Refinement),
            Item::new("Mesmerize Blade".to_string(), Purpose::Refinement),
            Item::new("Vampire Fang".to_string(), Purpose::Refinement),
            Item::new("Fury Fragment".to_string(), Purpose::Refinement),
            Item::new("Betrayal Sword".to_string(), Purpose::Refinement),
            Item::new("Sleep Powder".to_string(), Purpose::Refinement),
            Item::new("Life Ring".to_string(), Purpose::Refinement),
            Item::new("Dragon Fang".to_string(), Purpose::Refinement),
            Item::new("Spider Web".to_string(), Purpose::BlueMagic),
            Item::new("Coral Fragment".to_string(), Purpose::BlueMagic),
            Item::new("Curse Spike".to_string(), Purpose::BlueMagic),
            Item::new("Black Hole".to_string(), Purpose::BlueMagic),
            Item::new("Water Crystal".to_string(), Purpose::BlueMagic),
            Item::new("Missile".to_string(), Purpose::BlueMagic),
            Item::new("Mystery Fluid".to_string(), Purpose::BlueMagic),
            Item::new("Running Fire".to_string(), Purpose::BlueMagic),
            Item::new("Inferno Fang".to_string(), Purpose::BlueMagic),
            Item::new("Malboro Tentacle".to_string(), Purpose::BlueMagic),
            Item::new("Whisper".to_string(), Purpose::BlueMagic),
            Item::new("Laser Cannon".to_string(), Purpose::BlueMagic),
            Item::new("Barrier".to_string(), Purpose::BlueMagic),
            Item::new("Power Generator".to_string(), Purpose::BlueMagic),
            Item::new("Dark Matter".to_string(), Purpose::BlueMagic),
            Item::new("Bomb Fragment".to_string(), Purpose::GFCompatibility),
            Item::new("Red Fang".to_string(), Purpose::GFCompatibility),
            Item::new("Arctic Wind".to_string(), Purpose::GFCompatibility),
            Item::new("North Wind".to_string(), Purpose::GFCompatibility),
            Item::new("Dynamo Stone".to_string(), Purpose::GFCompatibility),
            Item::new("Shear Feather".to_string(), Purpose::GFCompatibility),
            Item::new("Venom Fang".to_string(), Purpose::GFCompatibility),
            Item::new("Steel Orb".to_string(), Purpose::GFCompatibility),
            Item::new("Moon Stone".to_string(), Purpose::GFCompatibility),
            Item::new("Dino Bone".to_string(), Purpose::GFCompatibility),
            Item::new("Windmill".to_string(), Purpose::GFCompatibility),
            Item::new("Dragon Skin".to_string(), Purpose::GFCompatibility),
            Item::new("Fish Fin".to_string(), Purpose::GFCompatibility),
            Item::new("Dragon Fin".to_string(), Purpose::GFCompatibility),
            Item::new("Silence Powder".to_string(), Purpose::GFCompatibility),
            Item::new("Poison Powder".to_string(), Purpose::GFCompatibility),
            Item::new("Dead Spirit".to_string(), Purpose::GFCompatibility),
            Item::new("Chef's Knife".to_string(), Purpose::GFCompatibility),
            Item::new("Cactus Thorn".to_string(), Purpose::GFCompatibility),
            Item::new("Shaman Stone".to_string(), Purpose::GFCompatibility),
            Item::new("Fuel".to_string(), Purpose::Fuel),
            Item::new("HP Up".to_string(), Purpose::StatBoosting),
            Item::new("Str Up".to_string(), Purpose::StatBoosting),
            Item::new("Vit Up".to_string(), Purpose::StatBoosting),
            Item::new("Mag Up".to_string(), Purpose::StatBoosting),
            Item::new("Spr Up".to_string(), Purpose::StatBoosting),
            Item::new("Spd Up".to_string(), Purpose::StatBoosting),
            Item::new("Luck Up".to_string(), Purpose::StatBoosting),
            Item::new("LuvLuv G".to_string(), Purpose::LuvLuvG),
        ];
        Self { items }
    }

    // Describe a single item.
    pub fn describe(&self, name: String) -> Result<Item, Error> {
        let mut item: Option<Item> = None;
        for c in self.items.clone() {
            if c.name == name {
                item = Some(c)
            }
        }
        match item {
            Some(item) => Ok(item),
            None => Err(Error::NotFound),
        }
    }

    // List the items available.
    // TODO: search filter? by purpose?
    pub fn list(&self) -> [Item; 168] {
        self.items.clone()
    }
}
