use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string().to_lowercase()
}

fn read_float(prompt: &str) -> f32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}

fn main() {
    loop{

        // menu
        println!("Welcome to the command line unit converter!");
        println!("1) meters <--> feet");
        println!("2) kilograms <--> pounds");
        println!("3) pascals <--> atmospheres");
        println!("4) exit");

        // read input
        let choice = read_float("Please eneter your choice: ");

        // match conversion choice
        match choice {
            1.0 => {
                let input1 = read_float("Please enter a value to convert: ");

                let unit = read_input("Is this value in meters [m] or feet [f]: ");

                if unit == "m" {
                    let input1 = 3.2808399 * input1;
                    println!("Converted to: {:.2} feet", input1);
                } else if unit == "f" {
                    let input1 = input1 / 3.2808399;
                    println!("Converted to: {:.2} meters", input1);
                } else {
                    println!("You didn't enter an available unit!");
                    continue;
                }
            }

            2.0 => {
                let input2 = read_float("Please enter a value to convert: ");

                let unit = read_input("Is this value in kilograms [kg] or pounds [p]: ");

                if unit == "kg" {
                    let input2 = input2 * 2.20462262;
                    println!("Converted to: {:.2} pounds", input2);
                } else if unit == "p" {
                    let input2 = input2 / 2.20462262;
                    println!("Converted to: {:.2} kilograms", input2);
                } else {
                    println!("You didn't enter an available unit!");
                    continue;
                }
            }

            3.0 => {
                let input3 = read_float("Please enter a value to convert: ");

                let unit = read_input("Is this unit in pascals [Pa] or atmospheres [atm]: ");

                if unit == "pa" {
                    let input3 = input3 / 101325.0;
                    println!("Converted to {:.2} atmospheres", input3);
                } else if unit == "atm" {
                    let input3 = input3 * 101325.0;
                    println!("Converted to {:.2} pascals", input3);
                } else {
                    println!("You didn't enter an available unit!");
                    continue;
                }
            }

            4.0 => {
                println!("Exiting...");
                break;
            }

            _ => {
                println!("This was not an option!");
                continue;
            }
        }
    }
}
