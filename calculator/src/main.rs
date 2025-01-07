fn main() {
    println!("Welcome to Rust-Calc!");
    println!("Enter the first number: ");

    // Read the first number
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    println!("Enter the second number: ");

    // Read the second number
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    // Read the operation
    println!("Enter the operation (+, -, *, /): ");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    // Perform the operation
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("The result is: {}", result);
}

// todo => add (better) error handling
