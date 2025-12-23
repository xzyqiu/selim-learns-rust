// Import the input/output module to read user input
use std::io;
// Import the random number generator
use rand::Rng;
// Import Ordering enum to compare numbers
use std::cmp::Ordering;
// Import the colored crate to print colored text
use colored::{self, Colorize};

// Main game function that runs the number guessing game loop
pub fn run() {
    // Print the game title
    println!("Guess the number!");
    // Prompt the user to enter a number
    println!("Enter your number please: ");

    // Generate a random number between 1 and 100 (inclusive)
    // We need this so the game has something to compare against
    let number: u32 = rand::rng().random_range(1..=100);

    // Print the answer for testing (delete this before releasing)
    // This lets you verify the game logic works correctly
    println!("The number is {}", number);

    // Create a mutable string to store user input
    // mut allows us to change it, String::new() creates an empty string
    let mut guess = String::new();

    // Loop forever until the user guesses correctly and breaks out
    loop {
        // Clear the string so old input doesn't mix with new input
        guess.clear();
        
        // Read a line from the keyboard into the guess variable
        // expect() crashes with a message if reading fails
        io::stdin()
            .read_line(&mut guess)
            .expect("Please try entering a number.");

        // Try to convert the text input into a u32 number
        // If it works, store it in guess; if it fails, handle the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // If conversion failed, tell user and ask again
                println!("Please enter a number");
                // Skip to the next loop iteration
                continue;
            }
        };

        // Compare the user's guess against the secret number
        match guess.cmp(&number) {
            // If guess is less than number, print red message
            Ordering::Less => println!("{}", "Too small.".red()),
            // If they match, print green message and exit the loop
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            // If guess is greater than number, print red message
            Ordering::Greater => println!("{}", "Too big.".red())
        }
    }
}
