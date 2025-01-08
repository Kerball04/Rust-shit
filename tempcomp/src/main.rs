use std::io;

fn main() {
    println!("Welcome to the RTC (Rust temperature conversion)!");

    // Accept user input for Fahrenheit reading
    println!("Please enter the temperature in Fahrenheit: ");
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to accept user input.");
    let fahrenheit: f64 = f.trim().parse().unwrap();

    // Convert Fahrenheit to Celsius
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("The temperature in Celsius is {:.2} degrees.", celsius);

    // Accept user input for Celsius reading
    println!("Please enter the temperature in Celsius: ");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to accept user input.");
    let celsius: f64 = c.trim().parse().unwrap();

    // Convert Celsius to Fahrenheit
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("The temperature in Fahrenheit is {:.2} degrees.", fahrenheit);
}
