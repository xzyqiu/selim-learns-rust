use std::io;

pub fn counter() {
    let mut number = String::new();
    println!("Please enter the number to count to: ");

    io::stdin().read_line(&mut number).expect("Failed to read line.");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let mut counter = 0;
    loop {
        println!("{counter}");
        counter += 1;
        if counter == number + 1 {
            break;
        }
    };
}