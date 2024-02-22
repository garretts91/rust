//use std::io::{self, Write};

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// fn factorial(x: u64) -> f64 {
//     if x == 0 {
//         1.0
//     } else {
//         (1..=x).fold(1.0, |acc, n| acc * n as f64)
//     }
// }

fn main() {
    // // Stress test for Fibonacci until it panics due to stack overflow or hits a limit
    let mut n = 0;
    loop {
        match fibonacci(n) {
            _ => println!("Fibonacci({}) calculated successfully.", n),
        }
        n += 1;
        if n == u32::MAX {
            break; // This prevents an infinite loop; realistically, stack overflow will occur first
        }
    }

    // // Stress test for Factorial until it reaches infinity
    // let mut x = 0;
    // loop {
    //     let result = factorial(x);
    //     if result.is_infinite() {
    //         println!("Factorial({}) reached infinity.", x);
    //         break;
    //     }
    //     x += 1;
    // }
}