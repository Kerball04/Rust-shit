use std::io;
use std::time::Instant;

fn main() {
    let mut start_time: Option<Instant> = None;
    let mut is_running = false;

    loop{
        println!("Welcome to RustWatch!");
        println!("1) Start");
        println!("2) Stop");
        println!("3) Show Elapsed time");
        println!("4) Exit");

        // user input variable
        println!("Enter your choice: ");
        let mut choice = String::new();

        // read input
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not  read user input!");
        
        // convert to unsigned 32 bit integer
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This character is invalid!");
                continue;
            }
        };

        // process choice
        match choice {
            1 => {
                if is_running {
                    println!("Stopwatch is already runnning!");
                } else {
                    start_time = Some(Instant::now());
                    is_running = true;
                    println!("Stopwatch started!");
                }
            }

            2 => {
                if is_running {
                    is_running = false;
                    println!("Stopwatch stopped!");
                } else {
                    println!("Stopwatch is not running!");
                }
            }

            3 => {
                if let Some(start) = start_time {
                    if is_running {
                        let elapsed = start.elapsed();
                        println!("The time elasped since the start is: {:?}", elapsed);
                    } else {
                        println!{"Stopwatch is not runnning!"};
                    }
                } else {
                    println!("Stopwatch has not yet been started!");
                }
            }

            4 => {
                println!("Exiting...");
                break;
            }

            _ => println!("This option does not exist!")
        }
    }
}
