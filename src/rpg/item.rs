#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Potion(u32),
    Weapon(u32),
    Armor(u32),
    Nothing,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Potion(amount) => {
                if *amount >= 1 && *amount <= 10 {
                    "Vial of Crimson Vitality"
                } else if *amount >= 11 && *amount <= 25 {
                    "Elixir of Phoenix Tears"
                } else if *amount >= 26 && *amount <= 50 {
                    "Draught of Dragon's Blood"
                } else {
                    "Ambrosia of the Gods"
                }
            },
            Item::Weapon(damage) => {
                if *damage >= 1 && *damage <= 10 {
                    "Rusty Blade of Whispers"
                } else if *damage >= 11 && *damage <= 25 {
                    "Stormforge Cleaver"
                } else if *damage >= 26 && *damage <= 50 {
                    "Shadowmourne the Soulrender"
                } else {
                    "Excalibur: Dawn's Edge"
                }
            },
            Item::Armor(defense) => {
                match defense {
                    1..=5 => "Leather of the Night Stalker",
                    6..=15 => "Chainmail of Iron Resolve",
                    16..=30 => "Dragonscale Aegis",
                    _ => "Plate of the Eternal Guardian",
                }
            },
            Item::Nothing => "Empty Hands",
        }
    }
}
