use std::io;

// Function to generate Sum of Products logic expression from a truth table
fn generate_sop_expression(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) -> String {
    let mut sop_expression = String::new();

    for row in truth_table {
        let mut term = String::new();

        // Generate a term for the current row
        for (i, &value) in row.iter().enumerate() {
            if value == 0 {
                term.push('/');
            }
            term.push(variables[i]);
        }

        // Add the term to the overall expression
        sop_expression.push_str(&term);
        sop_expression.push_str(" + ");
    }

    // Remove the trailing " + " from the last term
    sop_expression.pop();
    sop_expression.pop();

    sop_expression
}

// Function to print the truth table
fn print_truth_table(variables: &Vec<char>, truth_table: &Vec<Vec<u8>>) {
    // Print header
    for variable in variables {
        print!("{} ", variable);
    }
    println!("| F");

    // Print dividing line
    for _ in variables {
        print!("--");
    }
    println!("|--");

    // Print rows
    for row in truth_table {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn main() {
    // Read the number of variables
    println!("Enter the number of variables (up to 4): ");
    let mut num_variables = String::new();
    io::stdin().read_line(&mut num_variables).expect("Failed to read line");
    let num_variables: usize = num_variables.trim().parse().expect("Invalid number");

    // Read the truth table rows
    println!("Enter the truth table rows separated by commas: ");
    let mut input_rows = String::new();
    io::stdin().read_line(&mut input_rows).expect("Failed to read line");

    // Parse truth table rows
    let truth_table: Vec<Vec<u8>> = input_rows
        .split(',')
        .map(|row| row.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    // Generate and print the truth table
    print_truth_table(&(b'A'..(b'A' + num_variables as u8)).map(char::from).collect(), &truth_table);

    // Generate and print the Sum of Products logic expression
    let sop_expression = generate_sop_expression(&truth_table, &(b'A'..(b'A' + num_variables as u8)).map(char::from).collect());
    println!("Logic Expression (Sum of Products): {}", sop_expression);
}
