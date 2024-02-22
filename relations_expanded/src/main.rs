// Import the Input/Output library for handling user input.
use std::io::{self, stdin};

// Function to check if a relation is reflexive.
// A relation R on set S is reflexive if every element of S is related to itself.
fn is_reflexive<T: PartialEq>(pairs: &[(T, T)], set: &[T]) -> bool {
    // Iterate through each element in the set.
    for elem in set.iter() {
        // Check if the current element forms a pair with itself in the relations list.
        let exists_self_pair = pairs.iter().any(|(x, y)| x == elem && y == elem);
        
        // If any element does not form a pair with itself, the relation is not reflexive.
        if !exists_self_pair {
            return false;
        }
    }
    // If all elements form a pair with themselves, the relation is reflexive.
    true
}

// Function to check if a relation is symmetric.
// A relation R on set S is symmetric if for all x, y in S, if xRy then yRx.
fn is_symmetric<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    // Iterate through each pair in the relations list.
    pairs.iter().all(|(x, y)| {
        // For each pair (x, y), check if there exists a corresponding pair (y, x).
        pairs.iter().any(|(a, b)| x == b && y == a)
    })
}

// Function to check if a relation is transitive.
// A relation R on set S is transitive if for all x, y, z in S, if xRy and yRz then xRz.
fn is_transitive<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    // Iterate through each pair (x, y) in the relations list.
    pairs.iter().all(|(x, y)| {
        // For the current pair (x, y), check all other pairs (a, b) to ensure transitivity.
        pairs.iter().all(|(a, b)| {
            // If y == a, then there must exist a pair (c, d) such that x == c and b == d.
            y != a || pairs.iter().any(|(c, d)| x == c && b == d)
        })
    })
}

// Function to check if a relation is antisymmetric.
// A relation R on set S is antisymmetric if for all x, y in S, if xRy and yRx then x = y.
fn is_antisymmetric<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    // Iterate through each pair in the relations list.
    pairs.iter().all(|(x, y)| {
        // For each pair (x, y), ensure it is either reflexive (x = y) or there's no corresponding pair (y, x).
        x == y || !pairs.iter().any(|(a, b)| y == a && x == b)
    })
}

// Function to check if a relation is an equivalence relation.
// A relation R on set S is an equivalence relation if it is reflexive, symmetric, and transitive.
fn is_equivalence<T: PartialEq + Clone>(pairs: &[(T, T)], set: &[T]) -> bool {
    // Check if the relation is reflexive.
    let reflexive = is_reflexive(pairs, set);
    // Check if the relation is symmetric.
    let symmetric = is_symmetric(pairs);
    // Check if the relation is transitive.
    let transitive = is_transitive(pairs);

    // The relation is an equivalence relation if it is reflexive, symmetric, and transitive.
    reflexive && symmetric && transitive
}

// The main function to drive the program.
fn main() {
    // Prompt the user for input.
    println!("Enter pairs as tuples (x, y), one per line. Enter an empty line to finish:");

    // Initialize a vector to store the pairs entered by the user.
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    // String to store the user's input.
    let mut input: String = String::new();

    // Loop to read lines from the user until an empty line is entered.
    while let Ok(_) = io::stdin().read_line(&mut input) {
        // Trim whitespace from the input for processing.
        let trimmed_input = input.trim();
        // Check if the input is empty, indicating the user has finished entering pairs.
        if trimmed_input.is_empty() {
            break;
        }

        // Attempt to parse the input into a pair of integers.
        let pair: Result<Vec<i32>, _> = trimmed_input
            .split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();

        // Check the result of the parsing attempt.
        match pair {
            Ok(p) if p.len() == 2 => {
                // If successful and exactly two numbers are provided, add the pair to the list.
                pairs.push((p[0], p[1]));
            },
            _ => {
                // If parsing fails or the wrong number of numbers are provided, notify the user.
                println!("Invalid input, please enter pairs as two comma-separated numbers.");
            },
        }
        // Clear the input string for the next iteration.
        input.clear();
    }

    // Generate the set of elements from the pairs to ensure reflexivity can be checked.
    let set: Vec<i32> = pairs.iter()
                             .flat_map(|&(x, y)| vec![x, y])
                             .collect::<Vec<_>>()
                             .into_iter()
                             .collect();

    // Print the results of checking each property on the entered pairs.
    println!("Is reflexive: {}", is_reflexive(&pairs, &set));
    println!("Is symmetric: {}", is_symmetric(&pairs));
    println!("Is transitive: {}", is_transitive(&pairs));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs));
    println!("Is equivalence: {}", is_equivalence(&pairs, &set));
}
