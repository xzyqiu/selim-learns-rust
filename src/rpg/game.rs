use super::character::{Character, CharacterClass};
use super::item::Item;
use super::oath::Oath;
use colored::Colorize;
use std::io;

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => panic!("Failed to read input"),
    }
    input.trim().to_string()
}

pub fn run() {
    println!("{}", "=== Welcome to the RPG ===".bold().cyan());
    println!("{}", "(keep in mind this is just a learning project not an actual game)".dimmed().italic().black());
    println!("\nEnter your character's name:");
    let player_name = read_input();
    println!("\nChoose your class:");
    println!("1. Warrior (High damage, low intelligence)");
    println!("2. Mage (High intelligence, moderate damage)");
    println!("3. Rogue (Balanced stats)");
    let mut selected_class = CharacterClass::Warrior;
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
    enemy.hp = 150;
    enemy.damage = 30;
    let mut heroes: Vec<Character> = vec![player];
    println!("\n{}", format!("You are {} the {:?}!", heroes[0].name, heroes[0].class).green().bold());
    println!("{}", format!("A wild {} appears!", enemy.name).red().bold());
    let mut game_over = false;
    while !game_over {
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
        println!("7. Add Item to Inventory");
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
                println!("\n{}", format!("{}'s turn!", enemy.name).red().bold());
                let player_died = enemy.attack(player);
                if player_died {
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
            let item = match item_choice.as_str() {
                "1" => Item::Potion(20),
                "2" => Item::Weapon(15),
                "3" => Item::Armor(10),
                _ => {
                    println!("{}", "Invalid choice.".red());
                    continue;
                }
            };
            player.equip_item(item);
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
                continue;
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
            println!("Inventory:");
            for item in &player.inventory.items {
                println!("- {}", item.name());
            }
        } else if action == "6" {
            println!("{}", "You fled from battle!".yellow());
            game_over = true;
        } else if action == "7" {
            println!("\nChoose an item to add to inventory:");
            println!("1. Potion (healing: 20)");
            println!("2. Weapon (damage: +15)");
            println!("3. Armor (defense: 10)");
            let item_choice = read_input();
            let item = match item_choice.as_str() {
                "1" => Item::Potion(20),
                "2" => Item::Weapon(15),
                "3" => Item::Armor(10),
                _ => {
                    println!("{}", "Invalid choice.".red());
                    continue;
                }
            };
            println!("{}", format!("Added {} to inventory!", item.name()).green());
            player.inventory.add(item);
        }
    }
    println!("\n{}", "Thanks for playing!".cyan().bold());
}
