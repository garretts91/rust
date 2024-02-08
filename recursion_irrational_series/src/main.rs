use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Finds factorial for given number
fn factorial(x: u64) -> f64 {
    if x == 0 {
        1.0
    } else {
        (1..=x).fold(1.0, |acc, n| acc * n as f64)
    }
}

// Ramanujan formula for pi calculation
fn ramanujan_pi(digits: usize) -> String {
    let mut sum = 0.0;
    let mut n = 0;
    let i = f64::sqrt(8.0) / 9801.0;
    let precision = 10_f64.powi(-(digits as i32));

    loop {
        let tmp = i * (factorial(4 * n) / (factorial(n).powi(4)))
            * ((26390 * n as i32 + 1103) as f64 / 396_f64.powi(4 * n as i32));
        sum += tmp;

        if tmp.abs() < precision {
            break;
        }
        n += 1;
    }

    format!("{:.1$}", 1.0 / sum, digits)
}

fn main() {
    loop {
        println!("Recursion, Series, Irrational Numbers!");
        println!("Choose an option:");
        println!("1. Fibonacci Sequence");
        println!("2. Factorial");
        println!("3. Calculate Pi");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, prompt the menu again
        };

        match choice {
            1 => {
                println!("Enter the value of n for Fibonacci sequence:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let n: u32 = input.trim().parse().expect("Please enter a number");
                println!("Fibonacci({}) = {}", n, fibonacci(n));
            },
            2 => {
                println!("Enter the value of n for factorial:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let n: u64 = input.trim().parse().expect("Please enter a number");
                println!("Factorial({}) = {}", n, factorial(n));
            },
            3 => {
                println!("Enter the number of digits of Pi you want to calculate (up to 15 digits:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let digits: usize = input.trim().parse().expect("Please enter a number");
                println!("Value of Pi with {} digits is: {}", digits, ramanujan_pi(digits));
            },
            4 => {
                println!("Goodbye.");
                break;
            },
            _ => println!("Invalid option, please enter a number 1-4."),
        }
    }
}