use std::process::exit;
use std::io;
use colored::*;

// mod game; // Remove if not present
mod counter;
mod calculator;
mod color;
mod rpg;
mod bank;

fn main() {
    loop {
        println!("\n{}", "═══════════════════════════════════════".cyan().bold());
        println!("{}", "   What would you like to do?".bright_white().bold());
        println!("{}", "═══════════════════════════════════════".cyan().bold());
        println!("{} {}", "1.".yellow().bold(), "Play the game".white());
        println!("{} {}", "2.".yellow().bold(), "Counter".white());
        println!("{} {}", "3.".yellow().bold(), "Calculator".white());
        println!("{} {}", "4.".yellow().bold(), "Color Identifier".white());
        println!("{} {}", "5.".yellow().bold(), "RPG".white());
        println!("{} {}", "6.".yellow().bold(), "Banking App".white());
        println!("{} {}", "7.".yellow().bold(), "Exit".white());
        println!("{}", "═══════════════════════════════════════".cyan().bold());
    
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            // "1" => game::run(), // Remove or fix if not present
            "1" => println!("Game not implemented."),
            "2" => counter::counter(),
            "3" => calculator::run(),
            "4" => color::run(),
            "5" => rpg::game::run(),
            "6" => bank::run(),
            "7" => {
                println!("Goodbye!");
                exit(0);
            }
            _ => println!("{}", "Invalid option".red()),
        };
    }
}