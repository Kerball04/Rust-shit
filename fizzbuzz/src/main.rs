fn main() {
    for n in 1..101 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz"); // this has to come first since if it isn't divible by both, it's only divisible by one of them
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}
