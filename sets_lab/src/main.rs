use std::collections::HashSet;
use std::io::{self, BufRead};

fn read_keys_from_input(prompt: &str) -> HashSet<String> {
    println!("{}", prompt);
    let stdin = io::stdin();
    let mut keys: HashSet<String> = HashSet::new();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input.trim().to_lowercase() == "done" {
            break;
        }
        keys.insert(input);
    }
    keys
}

fn main() {
    // Read keys for Table A
    let table_a_keys = read_keys_from_input("Enter keys for Table A (one key per line, type 'done' to finish):");

    // Read keys for Table B
    let table_b_keys = read_keys_from_input("Enter keys for Table B (one key per line, type 'done' to finish):");

    // Compute set operations
    let union = table_a_keys.union(&table_b_keys);
    let intersection = table_a_keys.intersection(&table_b_keys);
    let difference_a_b = table_a_keys.difference(&table_b_keys);
    let difference_b_a = table_b_keys.difference(&table_a_keys);

    // Output the results
    println!("Set Operations:");
    println!("Table A ∪ Table B: {:?}", union.collect::<HashSet<_>>());
    println!("Table A ∩ Table B: {:?}", intersection.collect::<HashSet<_>>());
    println!("Table A - Table B: {:?}", difference_a_b.collect::<HashSet<_>>());
    println!("Table B - Table A: {:?}", difference_b_a.collect::<HashSet<_>>());
}
