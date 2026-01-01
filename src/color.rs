use std::{cmp::Ordering, io};
use colored::Colorize;
use std::cmp;

pub fn run() {
    let mut red = String::new();
    let mut green = String::new();
    let mut blue = String::new();

    println!("Please enter colors in RGB format.");
    println!("Please enter Red:");
    match io::stdin().read_line(&mut red) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read line.");
            return;
        }
    }
    let red: u32 = match red.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number for Red.");
            return;
        }
    };

    println!("Please enter Green:");
    match io::stdin().read_line(&mut green) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read line.");
            return;
        }
    }
    let green: u32 = match green.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number for Green.");
            return;
        }
    };

    println!("Please enter Blue:");
    match io::stdin().read_line(&mut blue) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read line.");
            return;
        }
    }
    let blue: u32 = match blue.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number for Blue.");
            return;
        }
    };

    if red > 255 || green > 255 || blue > 255 {
        println!("Color values must be between 0 and 255!");
        return;
    }

    if red == 0 && green == 0 && blue == 0 {
        println!("{}", "The color is Black".black());
    } else if green == 255 && red == 255 && blue == 255 {
        println!("{}", "The color is White".white());
    } else {
        match (red.cmp(&green), green.cmp(&blue), red.cmp(&blue)) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => println!("{}", "The color is Gray".white()),
            (Ordering::Less, Ordering::Less, _) => println!("{}", "The color is Blue".blue()),
            (Ordering::Greater, Ordering::Greater, _) => println!("{}", "The color is Red".red()),
            (_, Ordering::Greater, Ordering::Greater) => println!("{}", "The color is Green".green()),
            (Ordering::Equal, Ordering::Greater, _) => println!("{}", "The color is Cyan".cyan()),
            (Ordering::Greater, Ordering::Equal, _) => println!("{}", "The color is Yellow".yellow()),
            (Ordering::Less, Ordering::Equal, _) => println!("{}", "The color is Magenta".magenta()),
            (Ordering::Greater, Ordering::Less, Ordering::Greater) => println!("{}", "The color is Orange".bright_yellow()),
            (Ordering::Less, Ordering::Greater, Ordering::Less) => println!("{}", "The color is Purple".bright_magenta()),
            (Ordering::Equal, Ordering::Less, Ordering::Greater) => println!("{}", "The color is Lime".bright_green()),
            (Ordering::Less, Ordering::Greater, Ordering::Greater) => println!("{}", "The color is Teal".bright_cyan()),
            (Ordering::Greater, Ordering::Greater, Ordering::Less) => println!("{}", "The color is Pink".bright_red()),
            (Ordering::Less, Ordering::Less, Ordering::Equal) => println!("{}", "The color is Navy".bright_blue()),
            (Ordering::Greater, Ordering::Less, Ordering::Less) => println!("{}", "The color is Maroon".bright_black()),
            (Ordering::Equal, Ordering::Less, Ordering::Less) => println!("{}", "The color is Olive".yellow()),
            (Ordering::Less, Ordering::Equal, Ordering::Greater) => println!("{}", "The color is Aqua".bright_cyan()),
            _ => println!("Mixed color")
        }
    }
}
