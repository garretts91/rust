use std::io;

// Function to generate Sum of Products logic expression from a truth table
fn sum_of_products(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) -> String {
    let mut output = String::new();

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
        output.push_str(&term);
        output.push_str(" + ");
    }

    // Remove the trailing " + " from the last term
    output.pop();
    output.pop();

    output
}

// Function to print the truth table
fn create_truth_table(variables: &Vec<char>, truth_table: &Vec<Vec<u8>>) {
    // Print header
    println!("{} | F", variables.iter().map(|&v| v).collect::<String>());

    // Print dividing line
    println!("{}", "-".repeat(variables.len() * 2 + 4));

    // Print rows with function values
    for (row, values) in truth_table.iter().zip(truth_table.iter().map(|row| row.iter().map(|&v| v.to_string()).collect::<Vec<String>>())) {
        println!("{} | {}", values.join(" "), row.last().unwrap());
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
    create_truth_table(&(b'A'..(b'A' + num_variables as u8)).map(char::from).collect(), &truth_table);

    // Generate and print the Sum of Products logic expression
    let sop_expression = sum_of_products(&truth_table, &(b'A'..(b'A' + num_variables as u8)).map(char::from).collect());
    println!("Logic Expression (Sum of Products): {}", sop_expression);
}
