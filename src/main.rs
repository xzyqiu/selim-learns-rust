use std::process::exit;
use std::io;

mod game;
mod counter;
mod calculator;
mod color;
mod rpg;

fn main() {
    loop {
        println!("What would you like to do?");
        println!("1. Play the game");
        println!("2. Counter");
        println!("3. Calculator");
        println!("4. Color Identifier");
        println!("5. RPG");
        println!("6. Exit");
    
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => game::run(),
            "2" => counter::counter(),
            "3" => calculator::run(),
            "4" => color::run(),
            "5" => rpg::run(),
            "6" => {
                println!("Exiting...");
                exit(0);
            }
            _ => println!("Invalid choice, please try again."),
        };
    }
}