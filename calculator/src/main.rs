use std::io;
use calculator::{add, sub, mul, div};

fn main() {
    loop {
        println!("Welcome to Rust-Calc!");
        println!("Menu:\n1) Add\n2) Subtract\n3) Multiply\n4) Divide\n5) Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read user input!");

        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid choice!");
                continue;
            }
        };
        
        println!("Please enter a number: ");

        let mut n1: String = String::new();
        io::stdin().read_line(&mut n1).expect("Could not read user input!");

        let n1: i32 = match n1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        println!("Please enter a number: ");

        let mut n2: String = String::new();
        io::stdin().read_line(&mut n2).expect("Could not read user input!");

        let n2: i32 = match n2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match input {
            1 => {
                let sum = add(n1, n2);
                println!("Sum = {sum}");
                continue;
            }

            2 => {
                let diff = sub(n1, n2);
                println!("Difference = {diff}");
                continue;
            }

            3=> {
                let mul = mul(n1 as f32, n2 as f32);
                println!("Product = {}", mul);
                continue;
            }

            4=> {
                let quo = div(n1 as f32, n2 as f32);
                println!("Quotient = {}", quo);
                continue;
            }

            5 => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("This is not an option!");
                continue;
            }
        }
    }
}
