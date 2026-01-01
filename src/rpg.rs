use colored::Colorize;
use std::io;

// TODO: add more classes later
#[derive(Debug, Clone, Copy)]
enum CharacterClass { 
    Warrior, 
    Mage, 
    Rogue 
}

#[derive(Debug, Clone, Copy)]
enum Item { 
    Potion(u32), 
    Weapon(u32), 
    Armor(u32), 
    Nothing,
}

impl Item {
    fn name(&self) -> &str {
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
                    return "Rusty Blade of Whispers";
                }
                if *damage >= 11 && *damage <= 25 {
                    return "Stormforge Cleaver";  
                }
                if *damage >= 26 && *damage <= 50 {
                    return "Shadowmourne the Soulrender";
                }
                return "Excalibur: Dawn's Edge";
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

#[derive(Clone, Debug)]
enum Ultimate {
    ShadowStep,
    TitanCleave, 
    DivineLight,
}

#[derive(Debug, Clone)]
enum Oath { 
    Bladesharper, 
    Oathless, 
    Jetstriker, 
    Saltchemist 
}

impl Oath {
    fn info(&self) -> &str {
        if let Oath::Bladesharper = self {
            return "Increases weapon damage.";
        } else if let Oath::Jetstriker = self {
            return "Increases attack speed.";
        } else if let Oath::Saltchemist = self {
            return "Enhances potion effects.";
        } else {
            return "Does nothing.";
        }
    }
}

#[derive(Debug, Clone)]
struct Character {
    name: String,
    class: CharacterClass,
    hp: u32,
    level: u32,
    damage: u32, 
    intelligence: u32,
    item: Item,
    oath: Oath,
    ultimate: Ultimate,
}

impl Character {
    fn new(name: String, class: CharacterClass) -> Character {
        let mut character = Character {
            name: name,
            class: class,
            hp: 100,
            level: 1,  
            damage: 10,
            intelligence: 0, // will set this later
            item: Item::Nothing,
            oath: Oath::Oathless,
            ultimate: Ultimate::ShadowStep, // default, will change
        };
        
        // set intelligence based on class
        if let CharacterClass::Rogue = character.class {
            character.intelligence = 5;
        } else if let CharacterClass::Warrior = character.class {
            character.intelligence = 2;
        } else {
            character.intelligence = 20; // mage
        }
        
        // set ultimate
        match character.class {
            CharacterClass::Mage => character.ultimate = Ultimate::DivineLight,
            CharacterClass::Warrior => character.ultimate = Ultimate::TitanCleave,
            CharacterClass::Rogue => character.ultimate = Ultimate::ShadowStep,
        }
        
        return character;
    }

    fn take_damage(&mut self, damage: u32) -> bool {
        let mut effective_damage = damage;
        
        match &self.item {
            Item::Armor(resistance) => {
                if damage > *resistance {
                    effective_damage = damage - resistance;
                } else {
                    effective_damage = 0;
                }
            },
            _ => {
                // no armor, take full damage
            },
        }
        
        if self.hp > effective_damage {
            self.hp = self.hp - effective_damage;  
        } else {
            self.hp = 0;
        }

        println!("{}", format!("{} took {} damage! HP is now {}", self.name, effective_damage, self.hp).red().bold());

        if self.hp == 0 {
            println!("{}", format!("{} has died.", self.name).red().bold());
            return true; // character died
        } else {
            return false; // character still alive
        }
    }

    fn get_oath(&mut self, oath: Oath) -> String {
        // check current oath
        match &self.oath {
            Oath::Oathless => {
                self.oath = oath;
                return "Oath set.".to_string();
            }
            _ => {
                return "Can't have 2 oaths.".to_string();
            }
        }
    }

    fn heal(&mut self) {
        let mut heal_amount = self.intelligence;
        
        // check if we have potion
        if let Item::Potion(amount) = &self.item {
            heal_amount = heal_amount + amount;
            self.item = Item::Nothing; // consume potion
        }
        
        // check oath bonus
        if let Oath::Saltchemist = &self.oath {
            heal_amount = heal_amount + 20;
        }
        
        self.hp = self.hp + heal_amount;
        
        println!("{}", format!("{} healed! HP is now {}", self.name, self.hp).green().bold());
    }

    fn equip_item(&mut self, item: Item) {
        self.item = item;
    }

    fn attack(&self, enemy: &mut Character) -> bool {
        let mut dmg = self.damage;
        
        // check for weapon
        match &self.item {
            Item::Weapon(amount) => {
                dmg = dmg + amount;
            },
            _ => {
                // no weapon
            }
        }
        
        // check oath
        match &self.oath {
            Oath::Bladesharper => {
                dmg = dmg + 20;
            },
            _ => {
                // no damage bonus
            }
        }

        let died = enemy.take_damage(dmg);
        if died {
            println!("{}", format!("{} is dead", enemy.name).red().bold());
            return true; // enemy died
        } else {
            return false; // enemy still alive
        }
    }

    fn level_up(&mut self) {
        // increase damage based on class
        if let CharacterClass::Mage = self.class {
            self.damage += 2;
        }
        if let CharacterClass::Rogue = self.class {
            self.damage += 3;
        }  
        if let CharacterClass::Warrior = self.class {
            self.damage += 5;
        }
        
        self.level = self.level + 1;
    }
    
    fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => panic!("Failed to read input"),
    }
    return input.trim().to_string();
}

pub fn run() {
    println!("{}", "=== Welcome to the RPG ===".bold().cyan());
    println!("{}", "(keep in mind this is just a learning project not an actual game)".dimmed().italic().black());
    
    // get character name
    println!("\nEnter your character's name:");
    let player_name = read_input();
    
    println!("\nChoose your class:");
    println!("1. Warrior (High damage, low intelligence)");
    println!("2. Mage (High intelligence, moderate damage)");
    println!("3. Rogue (Balanced stats)");
    
    let mut selected_class = CharacterClass::Warrior; // default
    let mut class_chosen = false;
    
    while !class_chosen {
        let choice = read_input();
        
        if choice == "1" {
            selected_class = CharacterClass::Warrior;
            class_chosen = true;
        } else if choice == "2" {
            selected_class = CharacterClass::Mage; 
            class_chosen = true;
        } else if choice == "3" {
            selected_class = CharacterClass::Rogue;
            class_chosen = true;
        } else {
            println!("{}", "Invalid choice. Please enter 1, 2, or 3.".red());
        }
    }
    
    let player = Character::new(player_name, selected_class);
    let mut enemy = Character::new("Goblin".to_string(), CharacterClass::Warrior);
    enemy.hp = 50; // make enemy weaker
    enemy.damage = 8;
    
    // use a vector to store heroes
    let mut heroes: Vec<Character> = vec![player];
    
    println!("\n{}", format!("You are {} the {:?}!", heroes[0].name, heroes[0].class).green().bold());
    println!("{}", format!("A wild {} appears!", enemy.name).red().bold());
    
    // game loop
    let mut game_over = false;
    while !game_over {
        // check if player exists
        if heroes.is_empty() {
            println!("\n{}", "Game Over! All heroes have fallen...".red().bold());
            break;
        }
        
        let player = &mut heroes[0];
        
        println!("\n{}", "--- Your Turn ---".cyan().bold());
        println!("Your HP: {} | Enemy HP: {}", player.hp.to_string().green(), enemy.hp.to_string().red());
        println!("\nWhat will you do?");
        println!("1. Attack");
        println!("2. Heal");
        println!("3. Equip Item");
        println!("4. Choose Oath");
        println!("5. Check Status");
        println!("6. Run Away");
        
        let action = read_input();
        
        if action == "1" {
            println!("{}", format!("{} attacks {}!", player.name, enemy.name).yellow().bold());
            let enemy_died = player.attack(&mut enemy);
            if enemy_died {
                println!("\n{}", format!("Victory! {} defeated {}!", player.name, enemy.name).green().bold());
                player.level_up();
                println!("{}", format!("{} leveled up to level {}!", player.name, player.level).yellow().bold());
                game_over = true;
            } else {
                // enemy attacks back
                println!("\n{}", format!("{}'s turn!", enemy.name).red().bold());
                let player_died = enemy.attack(player);
                if player_died {
                    // remove dead player from heroes
                    heroes.retain(|h| !h.is_dead());
                    println!("\n{}", "Game Over! You were defeated...".red().bold());
                    game_over = true;
                }
            }
        } else if action == "2" {
            player.heal();
        } else if action == "3" {
            println!("\nChoose an item to equip:");
            println!("1. Potion (healing: 20)");
            println!("2. Weapon (damage: +15)");
            println!("3. Armor (defense: 10)");
            
            let item_choice = read_input();
            
            if item_choice == "1" {
                player.equip_item(Item::Potion(20));
                println!("{}", "Equipped Potion!".green());
            } else if item_choice == "2" {
                player.equip_item(Item::Weapon(15));
                println!("{}", "Equipped Weapon!".green());  
            } else if item_choice == "3" {
                player.equip_item(Item::Armor(10));
                println!("{}", "Equipped Armor!".green());
            } else {
                println!("{}", "Invalid choice.".red());
            }
        } else if action == "4" {
            println!("\nChoose an Oath:");
            println!("1. Bladesharper - {}", Oath::Bladesharper.info());
            println!("2. Jetstriker - {}", Oath::Jetstriker.info());
            println!("3. Saltchemist - {}", Oath::Saltchemist.info());
            
            let oath_choice = read_input();
            
            let selected_oath: Oath;
            
            if oath_choice == "1" {
                selected_oath = Oath::Bladesharper;
            } else if oath_choice == "2" {  
                selected_oath = Oath::Jetstriker;
            } else if oath_choice == "3" {
                selected_oath = Oath::Saltchemist;
            } else {
                println!("{}", "Invalid choice.".red());
                continue; // skip the rest
            }
            
            let result = player.get_oath(selected_oath);
            println!("{}", result.green());
        } else if action == "5" {
            println!("\n{}", "--- Character Status ---".cyan().bold());
            println!("Name: {}", player.name.yellow());
            println!("Class: {:?}", player.class);
            println!("Level: {}", player.level);  
            println!("HP: {}", player.hp.to_string().green());
            println!("Damage: {}", player.damage);
            println!("Intelligence: {}", player.intelligence);
            println!("Equipped: {}", player.item.name().cyan());
            println!("Oath: {:?} - {}", player.oath, player.oath.info());
            println!("Ultimate: {:?}", player.ultimate);
        } else if action == "6" {
            println!("{}", "You fled from battle!".yellow());
            game_over = true;
        } else {
            println!("{}", "Invalid choice. Please try again.".red());
        }
    }
    
    println!("\n{}", "Thanks for playing!".cyan().bold());
}
