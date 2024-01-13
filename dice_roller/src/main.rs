use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Dice Roller!");

    loop {
        println!("Enter the dice notation (e.g., 2d6) or 'exit' to quit:");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.to_lowercase() == "exit" {
            break;
        }

        match roll_dice(input) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }

    println!("Goodbye!");
}

fn roll_dice(input: &str) -> Result<u32, &'static str> {
    let parts: Vec<&str> = input.split('d').collect();

    if parts.len() != 2 {
        return Err("Invalid input. Use the format 'NdM' where N is the number of dice and M is the number of sides.");
    }

    let num_dice: u32 = parts[0].parse().unwrap_or(1);
    let num_sides: u32 = parts[1].parse().unwrap_or(6);

    if num_dice == 0 || num_sides == 0 {
        return Err("Number of dice and sides must be greater than zero.");
    }

    let mut rng = rand::thread_rng();
    let result: u32 = (0..num_dice).map(|_| rng.gen_range(1..=num_sides)).sum();

    Ok(result)
}
