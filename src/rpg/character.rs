use super::item::Item;
use super::inventory::Inventory;
use super::oath::Oath;
use super::ultimate::Ultimate;
use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub class: CharacterClass,
    pub hp: u32,
    pub level: u32,
    pub damage: u32,
    pub intelligence: u32,
    pub item: Item,
    pub oath: Oath,
    pub ultimate: Ultimate,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(name: String, class: CharacterClass) -> Character {
        let mut character = Character {
            name,
            class,
            hp: 100,
            level: 1,
            damage: 10,
            intelligence: 0,
            item: Item::Nothing,
            oath: Oath::Oathless,
            ultimate: Ultimate::ShadowStep,
            inventory: Inventory::new(),
        };
        if let CharacterClass::Rogue = character.class {
            character.intelligence = 5;
        } else if let CharacterClass::Warrior = character.class {
            character.intelligence = 2;
        } else {
            character.intelligence = 20;
        }
        match character.class {
            CharacterClass::Mage => character.ultimate = Ultimate::DivineLight,
            CharacterClass::Warrior => character.ultimate = Ultimate::TitanCleave,
            CharacterClass::Rogue => character.ultimate = Ultimate::ShadowStep,
        }
        character
    }
    pub fn take_damage(&mut self, damage: u32) -> bool {
        let mut effective_damage = damage;
        match &self.item {
            Item::Armor(resistance) => {
                if damage > *resistance {
                    effective_damage = damage - resistance;
                } else {
                    effective_damage = 0;
                }
            },
            _ => {},
        }
        if self.hp > effective_damage {
            self.hp -= effective_damage;
        } else {
            self.hp = 0;
        }
        println!("{}", format!("{} took {} damage! HP is now {}", self.name, effective_damage, self.hp).red().bold());
        if self.hp == 0 {
            println!("{}", format!("{} has died.", self.name).red().bold());
            true
        } else {
            false
        }
    }
    pub fn get_oath(&mut self, oath: Oath) -> String {
        match &self.oath {
            Oath::Oathless => {
                self.oath = oath;
                "Oath set.".to_string()
            }
            _ => "Can't have 2 oaths.".to_string(),
        }
    }
    pub fn heal(&mut self) {
        let mut heal_amount = self.intelligence;
        let potion = self.inventory.items.iter().find(|item| matches!(item, Item::Potion(_)));
        if let Some(Item::Potion(amount)) = potion {
            heal_amount += amount;
            self.inventory.remove(&Item::Potion(*amount));
        } else {
            println!("{}", "No potion in inventory!".red());
            return;
        }
        if let Oath::Saltchemist = &self.oath {
            heal_amount += 20;
        }
        self.hp += heal_amount;
        println!("{}", format!("{} healed! HP is now {}", self.name, self.hp).green().bold());
    }
    pub fn equip_item(&mut self, item: Item) {
        if self.inventory.has(&item) {
            println!("{}", format!("Equipped {}!", item.name()).green());
            self.item = item;
        } else {
            println!("{}", "Item not in inventory!".red());
        }
    }
    pub fn attack(&self, enemy: &mut Character) -> bool {
        let mut dmg = self.damage;
        match &self.item {
            Item::Weapon(amount) => {
                dmg += amount;
            },
            _ => {},
        }
        match &self.oath {
            Oath::Bladesharper => {
                dmg += 20;
            },
            _ => {},
        }
        let died = enemy.take_damage(dmg);
        if died {
            println!("{}", format!("{} is dead", enemy.name).red().bold());
            true
        } else {
            false
        }
    }
    pub fn level_up(&mut self) {
        if let CharacterClass::Mage = self.class {
            self.damage += 2;
        }
        if let CharacterClass::Rogue = self.class {
            self.damage += 3;
        }
        if let CharacterClass::Warrior = self.class {
            self.damage += 5;
        }
        self.level += 1;
    }
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}
