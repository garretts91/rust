use rand::seq::SliceRandom;
use std::io;

fn generate_password(length: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-=_+[]{}|;:'\",.<>/?"
        .chars()
        .collect();

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    password
}

fn main() {
    println!("Welcome to the Rust Password Generator!");

    // Prompt user for password length
    let password_length: usize = loop {
        println!("Enter the desired password length:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(length) if length > 0 => break length,
            _ => println!("Invalid input. Please enter a positive integer."),
        }
    };

    let password = generate_password(password_length);

    println!("Generated Password: {}", password);
}
