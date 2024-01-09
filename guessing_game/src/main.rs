use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_attempts = 5;

    // println!("The secret number is {secret_number}");

    for attempt in 1..=max_attempts {

        println!("Attempt {} of {}", attempt, max_attempts);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if attempt == max_attempts {
            println!("You ran out of guesses. The correct answer was {}. Please try again!", secret_number);
        }
        
    }
}
