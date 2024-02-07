// i dont know why im doing this, but here we are
// made it to Fibonacci(93) using u64
// big ups Fibonacci(3000)

use std::collections::HashMap;
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn fibonacci_memo(n: u64, memo: &mut HashMap<u64, BigUint>) -> BigUint {
    match n {
        0 => Zero::zero(),
        1 => One::one(),
        _ => {
            if let Some(fib) = memo.get(&n) {
                return fib.clone();
            }
            let fib_n_minus_1 = fibonacci_memo(n - 1, memo);
            let fib_n_minus_2 = fibonacci_memo(n - 2, memo);
            let fib_n = fib_n_minus_1 + fib_n_minus_2;
            memo.insert(n, fib_n.clone());
            fib_n
        }
    }
}

fn main() {
    let mut memo: HashMap<u64, BigUint> = HashMap::new();
    let n = 3000; // Now you can try numbers larger than 93
    println!("Fibonacci({}) = {}", n, fibonacci_memo(n, &mut memo));
}
