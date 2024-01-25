use std::collections::HashMap;
use std::io;

fn main() {
    // Get the logic expression as input
    println!("Enter the logic expression (ex. A + B; (A * B) + (/C + D)):");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");

    // Remove leading and trailing whitespaces
    let expression = expression.trim();

    // Deduce the number of variables from the expression
    let num_variables = expression.chars().filter(|&c| c.is_ascii_alphabetic()).count();

    // Create a HashMap to store variable assignments
    let mut variable_values = HashMap::new();

    // Get the truth table as input
    println!("Enter the truth table rows separated by commas:");
    let mut table_input = String::new();
    io::stdin().read_line(&mut table_input).expect("Failed to read line");

    // Parse truth table rows and populate the variable values HashMap
    let truth_table: Vec<Vec<u8>> = table_input
        .split(',')
        .map(|row| {
            let values: Vec<u8> = row.trim().split_whitespace().map(|val| val.parse().unwrap()).collect();
            for (i, &value) in values.iter().enumerate() {
                variable_values.insert((b'A' + i as u8) as char, value);
            }
            values
        })
        .collect();

    // Display the truth table
    println!("Truth Table:");
    display_truth_table_header(num_variables);
    display_truth_table_divider(num_variables);

    for row in &truth_table {
        display_truth_table_row(row);
    }

    // Display the logic expression (Sum of Products)
    let sop_expression = generate_sop_expression(&truth_table, &variable_values);
    println!("\nLogic Expression (Sum of Products): {}", sop_expression);
}

fn display_truth_table_header(num_variables: usize) {
    print!("| ");
    for i in 0..num_variables {
        print!("{} ", (b'A' + i as u8) as char);
    }
    println!("| F");
}

fn display_truth_table_divider(num_variables: usize) {
    print!("|");
    for _ in 0..num_variables {
        print!("---");
    }
    println!("|---");
}

fn display_truth_table_row(row: &[u8]) {
    print!("| ");
    for val in row {
        print!("{} ", val);
    }
    println!("|");
}

fn generate_sop_expression(truth_table: &Vec<Vec<u8>>, variable_values: &HashMap<char, u8>) -> String {
    let mut sop_expression = String::new();

    for (index, row) in truth_table.iter().enumerate() {
        if row.last() == Some(&1) {
            let mut term = String::new();
            for (i, &val) in row.iter().enumerate() {
                let variable = (b'A' + i as u8) as char;
                if val == 0 {
                    term.push('/');
                }
                term.push(variable);
                term.push_str(&variable_values[&variable].to_string());
            }
            sop_expression.push_str(&term);
            if index < truth_table.len() - 1 {
                sop_expression.push_str(" + ");
            }
        }
    }

    sop_expression
}
