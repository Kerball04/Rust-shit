use std::io;
use num_integer::Roots;

fn main() {
    loop {
        // starting message
        println!("Welcome to the prime number checker!");

        // display a prompt
        println!("Please enter a number (0 to exit): ");

        // get a number from user input
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Could not read input from user!");
        
        // convert string into an unsigned, 32 bit integer
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid character entered!");
                continue; //next itteration of loop
            }
        };

        //check if number is greater than 1
        if number > 1 {
            // do nothing
        } else if number == 0 {
            println!("Exiting...");
            break;
        } else {
            println!("Number must be greater than 1!");
            continue; // start again
        }

        // boolean flag for logic
        let mut is_prime = true;

        // loop to check divisibility
        for n in 2..number.sqrt() {
            if number % n == 0 {
                println!("The number {number} is not prime!");
                is_prime = false;
                break;
            } else {
                continue;
            }
        }

        // print that the number is prime if the boolean variable is true after loop completion
        if is_prime {
            println!("The number {number} is prime!");
        }
    }
}
