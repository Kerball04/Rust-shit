use std::io;

fn main() {
    loop {
        println!("Temperature converter");
        println!("1) Convert to Fahrenheit");
        println!("2) Convert to Celsius");
        println!("3) Exit");

        // read input from user
        println!("Choose an option: ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read input!");

        // convert string into u32 for comparison
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, invalid character entered!");
                continue;
            },
        };
        
        match choice {
            1 => {
                println!("Please enter the temperature in Celsius: ");

                // read temperature from user
                let mut celsius = String::new();
                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read input");
                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("This is not a valid number!");
                        continue;
                    },
                };
                    
                println!("Converting Celsius to Fahrenheit...");
                let fahrenheit: f32 = 9.0 / 5.0 * celsius + 32.0; 
                println!("The temperature in Fahrenheit is {:.2}", fahrenheit);
            },

            2 => {
                println!("Please enter the temperature in Fahrenheit: ");

                // read temperature from user
                let mut fahrenheit = String::new();
                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read input");
                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("This is not a valid number!");
                        continue;
                    },
                };
                
                println!("Converting Fahrenheit to Celsius...");
                let celsius: f32 = 5.0 / 9.0 * (fahrenheit - 32.0);
                println!("The temperature in Celsius is {:.2}", celsius);
            },

            3 => {
                println!("Exiting...");
                break;
            },

            _ => println!("This is not an option!")
        }
    }
}
