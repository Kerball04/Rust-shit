use std::io;

fn main() {

    loop {
        println!("Welcome to the Fibonacci generator!");
        println!("Enter the number of the sequence you wish to display: ");
    
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Could not read from user!");
    
        let number = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        let mut fibonacci = vec![0, 1];

        // loop to populate array
        for _ in 2..number {
            let next = fibonacci[fibonacci.len() - 1] + fibonacci[fibonacci.len() - 2];
            fibonacci.push(next);
        }

        // print the entire string
        println!("{:?}", fibonacci);
    }
}
