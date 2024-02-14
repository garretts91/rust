use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct DataRow<K> {
    key: K,
    name: String,
}

fn read_table_from_input<K: std::str::FromStr>(prompt: &str) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq,
{
    println!("{}", prompt);
    let stdin = io::stdin();
    let mut table: HashSet<DataRow<K>> = HashSet::new();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input.trim().to_lowercase() == "done" {
            break;
        }
        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields.len() >= 2 {
            if let Ok(key) = fields[0].parse::<K>() {
                let name = fields[1].to_string();
                table.insert(DataRow { key, name });
            } else {
                println!("Invalid key format. Ignoring this line.");
            }
        } else {
            println!("Invalid input format. Ignoring this line.");
        }
    }
    table
}

fn inner_join<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    a.intersection(&b).cloned().collect()
}

fn left_join<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    let intersection = a.intersection(&b).cloned().collect::<HashSet<_>>();
    a.union(&intersection).cloned().collect()
}

fn right_join<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    let intersection = a.intersection(&b).cloned().collect::<HashSet<_>>();
    b.union(&intersection).cloned().collect()
}

fn outer_join<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    let intersection = a.intersection(&b).cloned().collect::<HashSet<_>>();
    let union = a.union(&b).cloned().collect::<HashSet<_>>();
    union.difference(&intersection).cloned().collect()
}

fn left_difference<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    a.difference(&b).cloned().collect()
}

fn right_difference<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    b.difference(&a).cloned().collect()
}

fn outer_difference<K>(a: &HashSet<DataRow<K>>, b: &HashSet<DataRow<K>>) -> HashSet<DataRow<K>>
where
    K: std::hash::Hash + std::cmp::Eq + Clone,
{
    let diff_a_b = left_difference(&a, &b);
    let diff_b_a = right_difference(&a, &b);
    diff_a_b.union(&diff_b_a).cloned().collect()
}

fn format_set_notation<K: std::fmt::Debug>(set: &HashSet<DataRow<K>>, label: &str) {
    println!("{}: {{", label);
    for row in set {
        println!("  {:?}", row);
    }
    println!("}}");
}

fn main() {
    // Read table A
    let table_a: HashSet<DataRow<i32>> =
        read_table_from_input("Enter data for Table A (one row per line, type 'done' to finish):");

    // Read table B
    let table_b: HashSet<DataRow<i32>> =
        read_table_from_input("Enter data for Table B (one row per line, type 'done' to finish):");

    // Compute set operations
    let inner = inner_join(&table_a, &table_b);
    let left = left_join(&table_a, &table_b);
    let right = right_join(&table_a, &table_b);
    let outer = outer_join(&table_a, &table_b);
    let left_diff = left_difference(&table_a, &table_b);
    let right_diff = right_difference(&table_a, &table_b);
    let outer_diff = outer_difference(&table_a, &table_b);

    // Output the results
    println!("Set Operations:");

    // Print each set operation in set notation
    format_set_notation(&inner, "Inner Join (A ∩ B)");
    format_set_notation(&left, "Left Join (A ∪ (A ∩ B))");
    format_set_notation(&right, "Right Join ((A ∩ B) ∪ B)");
    format_set_notation(&outer, "Outer Join (A ∪ B ∪ (A ∩ B))");
    format_set_notation(&left_diff, "Left Difference (A - B)");
    format_set_notation(&right_diff, "Right Difference (B - A)");
    format_set_notation(&outer_diff, "Outer Difference ((A - B) ∪ (B - A))");
}
