use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};

/// Optimized Fibonacci function (iterative approach)
fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

/// Optimized Factorial function using arbitrary precision
fn factorial(x: u64) -> BigUint {
    (1..=x).fold(BigUint::one(), |acc, n| acc * BigUint::from(n))
}

fn main() {
    // Fibonacci Stress Test
    let mut n = 0;
    println!("Starting Fibonacci Stress Test...");
    loop {
        let result = fibonacci(n);
        
        // Log every 10th Fibonacci number
        if n % 10 == 0 {
            println!("Fibonacci({}): {}", n, result);
        }

        // Stop if overflow is about to occur
        if result > u64::MAX / 2 {
            println!("Fibonacci({}) caused overflow!", n);
            break;
        }

        n += 1;
    }

    // Factorial Stress Test
    let mut x = 0;
    println!("\nStarting Factorial Stress Test...");
    loop {
        let result = factorial(x);

        // Log every 5th Factorial number
        if x % 5 == 0 {
            println!("Factorial({}): {}", x, result);
        }

        // Stop if the number of bits exceeds 10,000
        if result.bits() > 10_000 {
            println!("Factorial({}) exceeded 10,000 bits!", x);
            break;
        }

        x += 1;
    }
}
