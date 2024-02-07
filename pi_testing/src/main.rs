use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn factorial(x: u64) -> f64 {
    if x == 0 {
        1.0
    } else {
        (1..=x).fold(1.0, |acc, n| acc * n as f64)
    }
}

// Correctly calculate pi using the Ramanujan formula
fn ramanujan_pi(digits: usize) -> f64 {
    let mut sum = 0.0;
    let mut n = 0;
    let i = f64::sqrt(8.0) / 9801.0;
    let precision = 10_f64.powi(-((digits as i32) + 10)); // Increased precision for the loop

    while {
        let numerator = factorial(4 * n) * (1103.0 + 26390.0 * (n as f64));
        let denominator = factorial(n).powi(4) * (396_f64.powi(4 * n as i32));
        let tmp = (numerator / denominator) * i;

        sum += tmp;
        n += 1;

        tmp.abs() > precision
    } {}

    (1.0 / sum).recip()
}

// Format pi to the specified number of total digits, including before and after the decimal
fn format_pi(digits: usize, pi_value: f64) -> String {
    format!("{:.1$}", pi_value, digits - 1)
}

fn main() {
    println!("Recursion, Series, Irrational Numbers!");

    // User input for Fibonacci
    println!("Enter the value of n for Fibonacci sequence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a number");
    println!("Fibonacci({}) = {}", n, fibonacci(n));

    // User input for factorial
    println!("Enter the value of n for factorial:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u64 = input.trim().parse().expect("Please enter a number");
    println!("Factorial({}) = {}", n, factorial(n));

    // User input for digits of pi
    println!("Enter the number of digits of Pi you want to calculate:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let digits: usize = input.trim().parse().expect("Please enter a number");

    let pi_value = ramanujan_pi(digits);
    let formatted_pi = format_pi(digits, pi_value);
    println!("Value of Pi with {} digits is: {}", digits, formatted_pi);
}
