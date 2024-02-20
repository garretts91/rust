use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct DataRow {
    key: String,
    name: String,
}

fn read_table_from_input(prompt: &str) -> HashSet<DataRow> {
    println!("{}", prompt);
    let stdin = io::stdin();
    let mut table: HashSet<DataRow> = HashSet::new();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input.trim().to_lowercase() == "done" {
            break;
        }
        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields.len() >= 2 {
            let key = fields[0].to_string();
            let name = fields[1].to_string();
            table.insert(DataRow { key, name });
        } else {
            println!("Invalid input format. Ignoring this line.");
        }
    }
    table
}

fn read_table_from_file(file_path: &str) -> HashSet<DataRow> {
    let path = Path::new(file_path);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = io::BufReader::new(file);
    let mut table: HashSet<DataRow> = HashSet::new();
    for line in reader.lines() {
        let input = line.unwrap();
        let fields: Vec<&str> = input.split_whitespace().collect();
        if fields.len() >= 2 {
            let key = fields[0].to_string();
            let name = fields[1].to_string();
            table.insert(DataRow { key, name });
        } else {
            println!("Invalid input format. Ignoring this line.");
        }
    }
    table
}

fn inner_join(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    a.intersection(b).cloned().collect()
}

fn left_join(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    let intersection = a.intersection(b).cloned().collect::<HashSet<_>>();
    a.union(&intersection).cloned().collect()
}

fn right_join(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    let intersection = a.intersection(b).cloned().collect::<HashSet<_>>();
    b.union(&intersection).cloned().collect()
}

fn outer_join(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    let intersection = a.intersection(b).cloned().collect::<HashSet<_>>();
    let union = a.union(b).cloned().collect::<HashSet<_>>();
    union.difference(&intersection).cloned().collect()
}

fn left_difference(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    a.difference(b).cloned().collect()
}

fn right_difference(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    b.difference(a).cloned().collect()
}

fn outer_difference(a: &HashSet<DataRow>, b: &HashSet<DataRow>) -> HashSet<DataRow> {
    let diff_a_b = left_difference(a, b);
    let diff_b_a = right_difference(a, b);
    diff_a_b.union(&diff_b_a).cloned().collect()
}

fn format_set_notation(set: &HashSet<DataRow>, label: &str) {
    println!("{}: {{", label);
    for row in set {
        println!("  {:?}", row);
    }
    println!("}}");
}

fn main() {
    loop {
        println!("Choose input method:");
        println!("1. Terminal input");
        println!("2. File input");
        println!("3. Exit");
        
        let mut input_method = String::new();
        io::stdin().read_line(&mut input_method).expect("Failed to read line");
        let input_method = input_method.trim();

        match input_method {
            "1" => {
                let table_a = read_table_from_input("Enter data for Table A (one row per line, type 'done' to finish):");
                let table_b = read_table_from_input("Enter data for Table B (one row per line, type 'done' to finish):");
                let table_c = read_table_from_input("Enter data for Table C (one row per line, type 'done' to finish):");

                // Compute set operations involving table A and table B
                let inner_ab = inner_join(&table_a, &table_b);
                let left_ab = left_join(&table_a, &table_b);
                let right_ab = right_join(&table_a, &table_b);
                let outer_ab = outer_join(&table_a, &table_b);
                let left_diff_ab = left_difference(&table_a, &table_b);
                let right_diff_ab = right_difference(&table_a, &table_b);
                let outer_diff_ab = outer_difference(&table_a, &table_b);

                // Output the results involving tables A and B
                println!("Set Operations involving Tables A and B:");

                // Print each set operation in set notation
                format_set_notation(&inner_ab, "Inner Join (A ∩ B)");
                format_set_notation(&left_ab, "Left Join (A ∪ (A ∩ B))");
                format_set_notation(&right_ab, "Right Join ((A ∩ B) ∪ B)");
                format_set_notation(&outer_ab, "Outer Join (A ∪ B ∪ (A ∩ B))");
                format_set_notation(&left_diff_ab, "Left Difference (A - B)");
                format_set_notation(&right_diff_ab, "Right Difference (B - A)");
                format_set_notation(&outer_diff_ab, "Outer Difference ((A - B) ∪ (B - A))");

                // Check if table C is empty
                if !table_c.is_empty() {
                    // Compute set operations involving table A, table B, and table C
                    let inner_ac = inner_join(&table_a, &table_c);
                    let left_ac = left_join(&table_a, &table_c);
                    let right_ac = right_join(&table_a, &table_c);
                    let outer_ac = outer_join(&table_a, &table_c);
                    let left_diff_ac = left_difference(&table_a, &table_c);
                    let right_diff_ac = right_difference(&table_a, &table_c);
                    let outer_diff_ac = outer_difference(&table_a, &table_c);

                    // Output the results involving tables A, B, and C
                    println!("Set Operations involving Tables A, B, and C:");

                    // Print each set operation in set notation
                    format_set_notation(&inner_ac, "Inner Join (A ∩ C)");
                    format_set_notation(&left_ac, "Left Join (A ∪ (A ∩ C))");
                    format_set_notation(&right_ac, "Right Join ((A ∩ C) ∪ C)");
                    format_set_notation(&outer_ac, "Outer Join (A ∪ C ∪ (A ∩ C))");
                    format_set_notation(&left_diff_ac, "Left Difference (A - C)");
                    format_set_notation(&right_diff_ac, "Right Difference (C - A)");
                    format_set_notation(&outer_diff_ac, "Outer Difference ((A - C) ∪ (C - A))");

                    // Compute set operations involving table B and table C
                    let inner_bc = inner_join(&table_b, &table_c);
                    let left_bc = left_join(&table_b, &table_c);
                    let right_bc = right_join(&table_b, &table_c);
                    let outer_bc = outer_join(&table_b, &table_c);
                    let left_diff_bc = left_difference(&table_b, &table_c);
                    let right_diff_bc = right_difference(&table_b, &table_c);
                    let outer_diff_bc = outer_difference(&table_b, &table_c);

                    // Output the results involving tables B and C
                    println!("Set Operations involving Tables B and C:");

                    // Print each set operation in set notation
                    format_set_notation(&inner_bc, "Inner Join (B ∩ C)");
                    format_set_notation(&left_bc, "Left Join (B ∪ (B ∩ C))");
                    format_set_notation(&right_bc, "Right Join ((B ∩ C) ∪ C)");
                    format_set_notation(&outer_bc, "Outer Join (B ∪ C ∪ (B ∩ C))");
                    format_set_notation(&left_diff_bc, "Left Difference (B - C)");
                    format_set_notation(&right_diff_bc, "Right Difference (C - B)");
                    format_set_notation(&outer_diff_bc, "Outer Difference ((B - C) ∪ (C - B))");
                }
            }
            "2" => {
                // modify the filespath here:
                let table_a = read_table_from_file("X:\\Users\\grrtt\\source\\repos\\rust\\sets\\table_a.txt");
                let table_b = read_table_from_file("X:\\Users\\grrtt\\source\\repos\\rust\\sets\\table_b.txt");
                let table_c = read_table_from_file("X:\\Users\\grrtt\\source\\repos\\rust\\sets\\table_c.txt");

                // Compute set operations involving table A and table B
                let inner_ab = inner_join(&table_a, &table_b);
                let left_ab = left_join(&table_a, &table_b);
                let right_ab = right_join(&table_a, &table_b);
                let outer_ab = outer_join(&table_a, &table_b);
                let left_diff_ab = left_difference(&table_a, &table_b);
                let right_diff_ab = right_difference(&table_a, &table_b);
                let outer_diff_ab = outer_difference(&table_a, &table_b);

                // Output the results involving tables A and B
                println!("Set Operations involving Tables A and B:");

                // Print each set operation in set notation
                format_set_notation(&inner_ab, "Inner Join (A ∩ B)");
                format_set_notation(&left_ab, "Left Join (A ∪ (A ∩ B))");
                format_set_notation(&right_ab, "Right Join ((A ∩ B) ∪ B)");
                format_set_notation(&outer_ab, "Outer Join (A ∪ B ∪ (A ∩ B))");
                format_set_notation(&left_diff_ab, "Left Difference (A - B)");
                format_set_notation(&right_diff_ab, "Right Difference (B - A)");
                format_set_notation(&outer_diff_ab, "Outer Difference ((A - B) ∪ (B - A))");

                // Check if table C is empty
                if !table_c.is_empty() {
                    // Compute set operations involving table A, table B, and table C
                    let inner_ac = inner_join(&table_a, &table_c);
                    let left_ac = left_join(&table_a, &table_c);
                    let right_ac = right_join(&table_a, &table_c);
                    let outer_ac = outer_join(&table_a, &table_c);
                    let left_diff_ac = left_difference(&table_a, &table_c);
                    let right_diff_ac = right_difference(&table_a, &table_c);
                    let outer_diff_ac = outer_difference(&table_a, &table_c);

                    // Output the results involving tables A, B, and C
                    println!("Set Operations involving Tables A, B, and C:");

                    // Print each set operation in set notation
                    format_set_notation(&inner_ac, "Inner Join (A ∩ C)");
                    format_set_notation(&left_ac, "Left Join (A ∪ (A ∩ C))");
                    format_set_notation(&right_ac, "Right Join ((A ∩ C) ∪ C)");
                    format_set_notation(&outer_ac, "Outer Join (A ∪ C ∪ (A ∩ C))");
                    format_set_notation(&left_diff_ac, "Left Difference (A - C)");
                    format_set_notation(&right_diff_ac, "Right Difference (C - A)");
                    format_set_notation(&outer_diff_ac, "Outer Difference ((A - C) ∪ (C - A))");

                    // Compute set operations involving table B and table C
                    let inner_bc = inner_join(&table_b, &table_c);
                    let left_bc = left_join(&table_b, &table_c);
                    let right_bc = right_join(&table_b, &table_c);
                    let outer_bc = outer_join(&table_b, &table_c);
                    let left_diff_bc = left_difference(&table_b, &table_c);
                    let right_diff_bc = right_difference(&table_b, &table_c);
                    let outer_diff_bc = outer_difference(&table_b, &table_c);

                    // Output the results involving tables B and C
                    println!("Set Operations involving Tables B and C:");

                    // Print each set operation in set notation
                    format_set_notation(&inner_bc, "Inner Join (B ∩ C)");
                    format_set_notation(&left_bc, "Left Join (B ∪ (B ∩ C))");
                    format_set_notation(&right_bc, "Right Join ((B ∩ C) ∪ C)");
                    format_set_notation(&outer_bc, "Outer Join (B ∪ C ∪ (B ∩ C))");
                    format_set_notation(&left_diff_bc, "Left Difference (B - C)");
                    format_set_notation(&right_diff_bc, "Right Difference (C - B)");
                    format_set_notation(&outer_diff_bc, "Outer Difference ((B - C) ∪ (C - B))");
                }
            }
            "3" => {
                println!("Exiting program.");
                return; // Exit the program
            }
            _ => println!("Invalid choice. Please choose 1, 2, or 3."),
        }
    }
}
