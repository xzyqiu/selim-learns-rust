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

        let choice: u32 = match choice_trim.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number between 1 and 4.");
                continue;
            }
        };
        let num1: i32 = match number1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number.");
                continue;
            }
        };
        let num2: i32 = match number2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try entering a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("{}", add(num1, num2));
                break;
            },
            2 => {
                println!("{}", sub(num1, num2));
                break;
            },
            3 => match div(num1, num2) {
                Some(r) => {
                    println!("{}", r);
                    break;
                },
                None => println!("Cannot divide by zero"),
            },
            4 => {
                println!("{}", mul(num1, num2));
                break;
            },
            _ => println!("Invalid choice"),
        };
    }
}

fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}

fn sub(x: i32, y: i32) -> i32 {
    let result = x - y;
    return result;
}

fn div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn mul(x: i32, y: i32) -> i32 {
    let result = x * y;
    return result;
}
