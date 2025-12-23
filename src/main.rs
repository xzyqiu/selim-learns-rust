use std::process::exit;

mod game;

fn main() {
    loop {
        println!("What would you like to do?");
        println!("1. Play the game");
        println!("2. Exit");
    
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => game::run(),
            "2" => {
                println!("Exiting...");
                exit(0);
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}