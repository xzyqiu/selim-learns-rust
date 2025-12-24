// Import the input/output module to read user input from the keyboard
use std::io;
// Import the random number generator crate so we can generate random numbers
use rand::Rng;
// Import the Ordering enum which has Less, Equal, and Greater variants for comparing numbers
use std::cmp::Ordering;
// Import the colored crate and bring Colorize trait into scope so we can color text output
use colored::{self, Colorize};

// Define a public function named run that contains the entire number guessing game loop
pub fn run() {
    // Print the text "Guess the number!" to the console screen
    println!("Guess the number!");
    // Print the text "Enter your number please: " to prompt the user to input a number
    println!("Enter your number please: ");

    // Generate a random u32 number between 1 and 100 inclusive using the rand crate's random_range method
    // We store this in a variable named number so we can compare the user's guesses against it later
    let number: u32 = rand::rng().random_range(1..=100);

    // Print the randomly generated number to the console for testing purposes (remove before release)
    // This lets you see what the secret number is so you can verify the game logic works correctly
    println!("The number is {}", number);

    // Create a mutable variable named guess that holds an empty String to store keyboard input
    // We use mut because we need to modify this string by adding user input to it in the loop
    let mut guess = String::new();

    // Start an infinite loop that will repeat until we break out of it manually when the user wins
    loop {
        // Clear all the text from the guess string so the previous guess doesn't get mixed with the new input
        guess.clear();
        
        // Read a line of text from the keyboard, add it to the guess variable, and crash with our message if it fails
        // The &mut guess means we're passing a mutable reference so read_line can modify the string
        io::stdin().read_line(&mut guess).expect("Try entering a number");

        // Try to parse the guess string as a u32 integer and handle both success and failure cases
        // If parsing succeeds, we store the number in a new guess variable, shadowing the old string variable
        let guess: u32 = match guess.trim().parse() {
            // If the parsing worked successfully, use the parsed number and continue the match
            Ok(nmb) => nmb,
            // If parsing failed, pattern match the error with _ (we don't care what the error was)
            Err(_) => {
                // Print a message telling the user they need to enter a valid number
                println!("Please enter a number");
                // Skip the rest of this loop iteration and jump back to the top of the loop
                continue;
            }
        };

        // Compare the user's guess number against the secret number and handle each possible result
        match guess.cmp(&number) {
            // If the guess is less than the secret number, print "Too small." in red color
            Ordering::Less => println!("{}", "Too small.".red()),
            // If the guess equals the secret number, print "You win!" in green color and exit the loop
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            // If the guess is greater than the secret number, print "Too big." in red color
            Ordering::Greater => println!("{}", "Too big.".red())
        }
    }
}