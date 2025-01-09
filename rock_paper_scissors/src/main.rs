use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    // rock, paper and scissors variables
    let mut input = String::new();

    // game logic
    loop {
        // random number for match to generate play tokens
        let number = rand::thread_rng().gen_range(1..=3); // random number between 1 and 3

        // match to fill rps variable
        let rps = match number {
            1 => "Rock".to_string(),

            2 => "Paper".to_string(),

            3 => "Scissors".to_string(),
            
            _ => {
                println!("An error occured!");
                break;
            }
        };

        //prompt user input
        println!("Type either Rock, Paper or Scissors to play: ");

        // get user input
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't obtain user input!");
        // no need to convert this, just compare strings directly

        // whitespace management
        let input = input.trim();

        // game logic
        match input {
            "Rock" | "Paper" | "Scissors" => {
                println!("You chose {}", input);
                println!("Computer chose {}", rps);

                if input == rps {
                    println!("It's a tie!");
                } else if
                    (input == "Rock" && rps == "Scissors") ||
                    (input == "Paper" && rps == "Rock") ||
                    (input == "Scissors" && rps == "Paper") {
                        println!("You win!");
                    } else {
                        println!("You lose!");
                    }
                }

                _ => {
                    println!("Invalid input!");
                    input.to_string().clear();
                    continue;
            }
        }

        println!("Would you like to play again? [yes/no]: ");
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Error occured!");

        if play_again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}

// todo -> fix invalid input error