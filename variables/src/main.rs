use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Could not read input!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index enetered was not a number!");

    let element = a[index];

    println!("The value of the array at index {index} is: {element}");
}
