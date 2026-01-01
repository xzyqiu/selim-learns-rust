enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}

use std::io;

pub fn run() {
    let mut choice = String::new();
    let mut number1 = String::new();
    let mut number2 = String::new(); 

    loop {
        choice.clear();
        number1.clear();
        number2.clear();

        println!("Select operator: 1=+ 2=- 3=/ 4=*\n(Or type 'q' to return to main menu)");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice_trim = choice.trim();
        if choice_trim.eq_ignore_ascii_case("q") {
            break;
        }

        println!("Enter number 1: ");
        io::stdin()
            .read_line(&mut number1)
            .expect("Failed to read line.");

        println!("Enter number 2: ");
        io::stdin()
            .read_line(&mut number2)
            .expect("Failed to read line.");

        let num1: f64 = match number1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number.");
                continue;
            }
        };
        let num2: f64 = match number2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number.");
                continue;
            }
        };

        let choice_int: u32 = match choice_trim.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        match choice_int {
            1 => {
                match calculate(Operation::Add(num1, num2)) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
                break;
            },
            2 => {
                match calculate(Operation::Subtract(num1, num2)) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
                break;
            },
            3 => {
                match calculate(Operation::Divide(num1, num2)) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
                break;
            },
            4 => {
                match calculate(Operation::Multiply(num1, num2)) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
                break;
            },
            _ => println!("Invalid choice"),
        };
    }
}